use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use uuid::Uuid;

use crate::supabase::SupabaseClient;

#[derive(Debug, Serialize, Deserialize)]
struct CredentialResponse {
    credential: String,
}

pub struct ApiKeyHandler {
    client: SupabaseClient,
    http_client: reqwest::Client,
}

impl ApiKeyHandler {
    pub fn new(client: SupabaseClient) -> Self {
        Self {
            client,
            http_client: reqwest::Client::new(),
        }
    }

    async fn get_provider_config(&self, workspace_id: Uuid, provider: &str) -> Result<JsonValue> {
        let db_client = self.client.get_client().await?;

        let stmt = db_client
            .prepare(
                "SELECT config_jsonb FROM provider_configs
                 WHERE workspace_id = $1 AND provider = $2",
            )
            .await?;

        let rows = db_client.query(&stmt, &[&workspace_id, &provider]).await?;

        if rows.is_empty() {
            anyhow::bail!("Provider config not found for: {}", provider);
        }

        let config: JsonValue = rows[0].get(0);
        Ok(config)
    }

    pub async fn fetch_credential(
        &self,
        workspace_id: Uuid,
        provider: &str,
        secret_name: &str,
    ) -> Result<String> {
        let config = self.get_provider_config(workspace_id, provider).await?;

        let endpoint = config["api_endpoint"]
            .as_str()
            .context("Missing api_endpoint in config")?;
        let auth_header = config["auth_header"].as_str().unwrap_or("Authorization");
        let auth_token = config["auth_token"]
            .as_str()
            .context("Missing auth_token in config")?;

        let url = format!("{}/credentials/{}", endpoint, secret_name);

        let response = self
            .http_client
            .get(&url)
            .header(auth_header, format!("Bearer {}", auth_token))
            .send()
            .await
            .context("Failed to fetch credential from API endpoint")?;

        if !response.status().is_success() {
            anyhow::bail!(
                "API endpoint returned error: {} - {}",
                response.status(),
                response.text().await.unwrap_or_default()
            );
        }

        let credential_response: CredentialResponse = response
            .json()
            .await
            .context("Failed to parse credential response")?;

        Ok(credential_response.credential)
    }
}
