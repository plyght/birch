use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::Config;

#[derive(Debug, Serialize, Deserialize)]
struct CreateWorkspaceRequest {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct WorkspaceResponse {
    workspace: Workspace,
}

#[derive(Debug, Serialize, Deserialize)]
struct Workspace {
    id: Uuid,
    name: String,
    plan_tier: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProviderConfig {
    id: Uuid,
    workspace_id: Uuid,
    provider: String,
    mode: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateProviderConfigRequest {
    provider: String,
    mode: String,
    config: serde_json::Value,
}

pub async fn login(api_url: Option<String>) -> Result<()> {
    let api_url_resolved = api_url.unwrap_or_else(|| "https://api.birch.sh".to_string());

    // Determine dashboard URL from API URL
    let dashboard_url =
        if api_url_resolved.contains("localhost") || api_url_resolved.contains("127.0.0.1") {
            // Local development
            "http://localhost:3001".to_string()
        } else {
            // Production
            "https://birch.sh".to_string()
        };

    println!("Login to Birch SaaS");
    println!("API URL: {}", api_url_resolved);
    println!("Dashboard URL: {}", dashboard_url);
    println!();

    // Generate random state token for CSRF protection
    let state = uuid::Uuid::new_v4().to_string();

    // Find an available port for callback server
    let callback_port = 9124;
    let callback_url = format!("http://localhost:{}/auth/callback", callback_port);

    // Construct OAuth URL
    let auth_url = format!(
        "{}/auth/cli?state={}&callback={}",
        dashboard_url,
        urlencoding::encode(&state),
        urlencoding::encode(&callback_url)
    );

    println!("Opening browser for authentication...");
    println!();

    // Open browser
    if let Err(e) = open_browser(&auth_url) {
        println!("Failed to open browser automatically: {}", e);
        println!("Please open this URL in your browser:");
        println!("{}", auth_url);
        println!();
    }

    // Start callback server and wait for token
    let token = crate::auth_callback::start_callback_server(state, callback_port)
        .await?
        .context("No token received from callback")?;

    // Store JWT token in config
    let mut config = Config::load()?;
    config.mode = "saas".to_string();
    config.saas_api_url = Some(api_url_resolved.clone());
    config.saas_jwt_token = Some(token);
    // Clear old API key if present
    config.saas_api_key = None;
    config.save()?;

    println!();
    println!("✓ Successfully logged in to Birch SaaS");
    println!("  API URL: {}", api_url_resolved);
    println!();
    println!("Run 'birch workspace list' to see your workspaces");

    Ok(())
}

fn open_browser(url: &str) -> Result<()> {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(url)
            .spawn()
            .context("Failed to open browser")?;
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(url)
            .spawn()
            .context("Failed to open browser")?;
    }

    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(&["/C", "start", url])
            .spawn()
            .context("Failed to open browser")?;
    }

    Ok(())
}

fn get_auth_token(config: &Config) -> Result<String> {
    // Prefer JWT token over API key
    if let Some(jwt_token) = &config.saas_jwt_token {
        Ok(jwt_token.clone())
    } else if let Some(api_key) = &config.saas_api_key {
        Ok(api_key.clone())
    } else {
        anyhow::bail!("Not authenticated. Run 'birch login' first.");
    }
}

pub async fn workspace_create(name: String) -> Result<()> {
    let config = Config::load()?;

    if config.mode != "saas" {
        anyhow::bail!("Not in SaaS mode. Run 'birch login' first.");
    }

    let api_url = config
        .saas_api_url
        .clone()
        .context("SaaS API URL not configured")?;
    let auth_token = get_auth_token(&config)?;

    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/api/v1/workspaces", api_url))
        .header("Authorization", format!("Bearer {}", auth_token))
        .json(&CreateWorkspaceRequest { name: name.clone() })
        .send()
        .await?;

    if !response.status().is_success() {
        anyhow::bail!("Failed to create workspace: {}", response.status());
    }

    let workspace_response: WorkspaceResponse = response.json().await?;

    println!("✓ Created workspace: {}", workspace_response.workspace.name);
    println!("  ID: {}", workspace_response.workspace.id);
    println!("  Plan: {}", workspace_response.workspace.plan_tier);
    println!();
    println!(
        "Run 'birch workspace select {}' to use this workspace",
        workspace_response.workspace.id
    );

    Ok(())
}

pub async fn workspace_list() -> Result<()> {
    let config = Config::load()?;

    if config.mode != "saas" {
        anyhow::bail!("Not in SaaS mode. Run 'birch login' first.");
    }

    let api_url = config
        .saas_api_url
        .clone()
        .context("SaaS API URL not configured")?;
    let auth_token = get_auth_token(&config)?;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/api/v1/workspaces", api_url))
        .header("Authorization", format!("Bearer {}", auth_token))
        .send()
        .await?;

    if !response.status().is_success() {
        anyhow::bail!("Failed to list workspaces: {}", response.status());
    }

    let workspaces: Vec<Workspace> = response.json().await?;

    if workspaces.is_empty() {
        println!("No workspaces found.");
        println!("Create one with: birch workspace create <name>");
        return Ok(());
    }

    println!("Workspaces:");
    for workspace in workspaces {
        let selected = config
            .saas_workspace_id
            .as_ref()
            .map(|id| id == &workspace.id.to_string())
            .unwrap_or(false);

        let marker = if selected { "→" } else { " " };

        println!(
            "{} {} - {} ({})",
            marker, workspace.id, workspace.name, workspace.plan_tier
        );
    }

    Ok(())
}

pub async fn workspace_select(id: String) -> Result<()> {
    let mut config = Config::load()?;

    if config.mode != "saas" {
        anyhow::bail!("Not in SaaS mode. Run 'birch login' first.");
    }

    config.saas_workspace_id = Some(id.clone());
    config.save()?;

    println!("✓ Selected workspace: {}", id);

    Ok(())
}

pub async fn provider_set(provider: String, mode: String) -> Result<()> {
    let config = Config::load()?;

    if config.mode != "saas" {
        anyhow::bail!("Not in SaaS mode. Run 'birch login' first.");
    }

    let workspace_id = config
        .saas_workspace_id
        .clone()
        .context("No workspace selected. Run 'birch workspace select <id>' first.")?;

    let api_url = config
        .saas_api_url
        .clone()
        .context("SaaS API URL not configured")?;
    let auth_token = get_auth_token(&config)?;

    let client = reqwest::Client::new();
    let response = client
        .post(format!(
            "{}/api/v1/workspaces/{}/providers",
            api_url, workspace_id
        ))
        .header("Authorization", format!("Bearer {}", auth_token))
        .json(&CreateProviderConfigRequest {
            provider: provider.clone(),
            mode: mode.clone(),
            config: serde_json::json!({}),
        })
        .send()
        .await?;

    if !response.status().is_success() {
        anyhow::bail!("Failed to configure provider: {}", response.text().await?);
    }

    println!("✓ Configured provider '{}' with mode '{}'", provider, mode);

    Ok(())
}

pub async fn provider_list() -> Result<()> {
    let config = Config::load()?;

    if config.mode != "saas" {
        anyhow::bail!("Not in SaaS mode. Run 'birch login' first.");
    }

    let workspace_id = config
        .saas_workspace_id
        .clone()
        .context("No workspace selected. Run 'birch workspace select <id>' first.")?;

    let api_url = config
        .saas_api_url
        .clone()
        .context("SaaS API URL not configured")?;
    let auth_token = get_auth_token(&config)?;

    let client = reqwest::Client::new();
    let response = client
        .get(format!(
            "{}/api/v1/workspaces/{}/providers",
            api_url, workspace_id
        ))
        .header("Authorization", format!("Bearer {}", auth_token))
        .send()
        .await?;

    if !response.status().is_success() {
        anyhow::bail!("Failed to list providers: {}", response.status());
    }

    let providers: Vec<ProviderConfig> = response.json().await?;

    if providers.is_empty() {
        println!("No providers configured.");
        println!("Configure one with: birch provider set <provider> --mode <mode>");
        return Ok(());
    }

    println!("Configured providers:");
    for provider in providers {
        println!("  {} - {}", provider.provider, provider.mode);
    }

    Ok(())
}

#[allow(dead_code)]
pub async fn resolve_credential(provider: &str, secret_name: &str) -> Result<Option<String>> {
    let config = Config::load()?;

    if config.mode != "saas" {
        return Ok(None);
    }

    let workspace_id = match config.saas_workspace_id {
        Some(ref id) => id.clone(),
        None => return Ok(None),
    };

    let api_url = match config.saas_api_url {
        Some(ref url) => url.clone(),
        None => return Ok(None),
    };

    let auth_token = match get_auth_token(&config) {
        Ok(token) => token,
        Err(_) => return Ok(None),
    };

    let client = reqwest::Client::new();
    let response = client
        .get(format!(
            "{}/api/v1/workspaces/{}/credentials/{}/{}",
            api_url, workspace_id, provider, secret_name
        ))
        .header("Authorization", format!("Bearer {}", auth_token))
        .send()
        .await?;

    if !response.status().is_success() {
        return Ok(None);
    }

    #[derive(Deserialize)]
    struct CredentialResponse {
        value: String,
    }

    let cred_response: CredentialResponse = response.json().await?;
    Ok(Some(cred_response.value))
}
