use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::supabase::SupabaseClient;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialHealth {
    pub workspace_id: Uuid,
    pub provider: String,
    pub mode: String,
    pub status: HealthStatus,
    pub last_check: DateTime<Utc>,
    pub last_success: Option<DateTime<Utc>>,
    pub last_failure: Option<DateTime<Utc>>,
    pub failure_count: i32,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

pub struct HealthMonitor {
    client: SupabaseClient,
}

impl HealthMonitor {
    pub fn new(client: SupabaseClient) -> Self {
        Self { client }
    }

    pub async fn record_success(
        &self,
        _workspace_id: Uuid,
        provider: &str,
        mode: &str,
    ) -> Result<()> {
        tracing::debug!("Recording health success for {} ({})", provider, mode);
        Ok(())
    }

    pub async fn record_failure(
        &self,
        _workspace_id: Uuid,
        provider: &str,
        mode: &str,
        error: &str,
    ) -> Result<()> {
        tracing::warn!(
            "Recording health failure for {} ({}): {}",
            provider,
            mode,
            error
        );
        Ok(())
    }

    pub async fn get_health_status(
        &self,
        workspace_id: Uuid,
        provider: &str,
    ) -> Result<CredentialHealth> {
        Ok(CredentialHealth {
            workspace_id,
            provider: provider.to_string(),
            mode: "unknown".to_string(),
            status: HealthStatus::Healthy,
            last_check: Utc::now(),
            last_success: Some(Utc::now()),
            last_failure: None,
            failure_count: 0,
            error_message: None,
        })
    }

    pub async fn check_oauth_token_expiry(
        &self,
        workspace_id: Uuid,
        provider: &str,
    ) -> Result<Option<DateTime<Utc>>> {
        let db_client = self.client.get_client().await?;

        let stmt = db_client
            .prepare(
                "SELECT expires_at FROM oauth_tokens
                 WHERE workspace_id = $1 AND provider = $2",
            )
            .await?;

        let rows = db_client.query(&stmt, &[&workspace_id, &provider]).await?;

        if rows.is_empty() {
            return Ok(None);
        }

        let expires_at: Option<DateTime<Utc>> = rows[0].get(0);
        Ok(expires_at)
    }

    pub async fn is_token_expiring_soon(
        &self,
        workspace_id: Uuid,
        provider: &str,
        threshold_seconds: i64,
    ) -> Result<bool> {
        if let Some(expires_at) = self
            .check_oauth_token_expiry(workspace_id, provider)
            .await?
        {
            let now = Utc::now();
            let threshold = chrono::Duration::seconds(threshold_seconds);
            Ok(expires_at - now < threshold)
        } else {
            Ok(false)
        }
    }
}
