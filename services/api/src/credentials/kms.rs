use anyhow::{Context, Result};
use serde_json::Value as JsonValue;
use uuid::Uuid;

use crate::supabase::SupabaseClient;

pub struct KmsHandler {
    client: SupabaseClient,
}

impl KmsHandler {
    pub fn new(client: SupabaseClient) -> Self {
        Self { client }
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

    pub async fn resolve_from_kms(
        &self,
        workspace_id: Uuid,
        provider: &str,
        secret_name: &str,
    ) -> Result<String> {
        let config = self.get_provider_config(workspace_id, provider).await?;

        let kms_provider = config["kms_provider"]
            .as_str()
            .context("Missing kms_provider in config")?;

        match kms_provider {
            "aws" => self.resolve_from_aws_kms(&config, secret_name).await,
            "gcp" => {
                self.resolve_from_gcp_secret_manager(&config, secret_name)
                    .await
            }
            "azure" => self.resolve_from_azure_keyvault(&config, secret_name).await,
            _ => anyhow::bail!("Unsupported KMS provider: {}", kms_provider),
        }
    }

    async fn resolve_from_aws_kms(&self, config: &JsonValue, secret_name: &str) -> Result<String> {
        let region = config["aws_region"]
            .as_str()
            .context("Missing aws_region in config")?;
        let kms_key_id = config["kms_key_id"]
            .as_str()
            .context("Missing kms_key_id in config")?;
        let secret_arn = config["secret_arn"]
            .as_str()
            .or_else(|| config["secrets"].get(secret_name).and_then(|v| v.as_str()))
            .context("Missing secret ARN")?;

        let aws_config = aws_config::defaults(aws_config::BehaviorVersion::latest())
            .region(aws_sdk_secretsmanager::config::Region::new(
                region.to_string(),
            ))
            .load()
            .await;

        let secrets_client = aws_sdk_secretsmanager::Client::new(&aws_config);

        let response = secrets_client
            .get_secret_value()
            .secret_id(secret_arn)
            .send()
            .await
            .context("Failed to retrieve secret from AWS Secrets Manager")?;

        let secret_string = response
            .secret_string()
            .context("Secret has no string value")?;

        Ok(secret_string.to_string())
    }

    async fn resolve_from_gcp_secret_manager(
        &self,
        config: &JsonValue,
        secret_name: &str,
    ) -> Result<String> {
        let project_id = config["gcp_project_id"]
            .as_str()
            .context("Missing gcp_project_id in config")?;
        let secret_id = config["secrets"]
            .get(secret_name)
            .and_then(|v| v.as_str())
            .context("Missing secret mapping for this secret")?;

        let secret_path = format!(
            "projects/{}/secrets/{}/versions/latest",
            project_id, secret_id
        );

        anyhow::bail!("GCP Secret Manager integration requires service account authentication")
    }

    async fn resolve_from_azure_keyvault(
        &self,
        config: &JsonValue,
        secret_name: &str,
    ) -> Result<String> {
        let vault_url = config["azure_vault_url"]
            .as_str()
            .context("Missing azure_vault_url in config")?;
        let secret_id = config["secrets"]
            .get(secret_name)
            .and_then(|v| v.as_str())
            .unwrap_or(secret_name);

        anyhow::bail!("Azure Key Vault integration requires managed identity or service principal")
    }
}
