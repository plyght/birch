use anyhow::Result;
use chrono::Utc;
use uuid::Uuid;

use crate::alerts::channels::{EmailChannel, SlackChannel, WebhookChannel};
use crate::alerts::models::*;
use crate::supabase::SupabaseClient;

pub struct AlertManager {
    client: SupabaseClient,
}

impl AlertManager {
    pub fn new(client: SupabaseClient) -> Self {
        Self { client }
    }

    pub async fn send_alert(
        &self,
        workspace_id: Uuid,
        alert_type: AlertType,
        severity: AlertSeverity,
        title: String,
        message: String,
        provider: Option<String>,
        secret_name: Option<String>,
        metadata: serde_json::Value,
    ) -> Result<()> {
        let alert = Alert {
            id: Uuid::new_v4(),
            workspace_id,
            alert_type,
            severity,
            title,
            message,
            provider,
            secret_name,
            metadata,
            created_at: Utc::now(),
        };

        let config = self.get_alert_config(workspace_id).await?;

        for channel_config in config.channels {
            if !channel_config.enabled {
                continue;
            }

            let result = match channel_config.channel {
                AlertChannel::Email => self.send_email(&alert, &channel_config.config).await,
                AlertChannel::Slack => self.send_slack(&alert, &channel_config.config).await,
                AlertChannel::Webhook => self.send_webhook(&alert, &channel_config.config).await,
                AlertChannel::InApp => self.store_in_app(&alert).await,
            };

            if let Err(e) = result {
                tracing::error!(
                    "Failed to send alert via {:?}: {}",
                    channel_config.channel,
                    e
                );
            }
        }

        Ok(())
    }

    async fn get_alert_config(&self, workspace_id: Uuid) -> Result<AlertConfig> {
        Ok(AlertConfig {
            workspace_id,
            channels: vec![AlertChannelConfig {
                channel: AlertChannel::Email,
                enabled: true,
                config: serde_json::json!({}),
            }],
        })
    }

    async fn send_email(&self, alert: &Alert, config: &serde_json::Value) -> Result<()> {
        let email_channel = EmailChannel::new(config.clone());
        let recipients = vec!["admin@example.com".to_string()];
        email_channel.send(alert, recipients).await
    }

    async fn send_slack(&self, alert: &Alert, config: &serde_json::Value) -> Result<()> {
        let webhook_url = config["webhook_url"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing webhook_url in Slack config"))?;

        let slack_channel = SlackChannel::new(webhook_url.to_string());
        slack_channel.send(alert).await
    }

    async fn send_webhook(&self, alert: &Alert, config: &serde_json::Value) -> Result<()> {
        let url = config["url"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing url in webhook config"))?;

        let webhook_channel = WebhookChannel::new(url.to_string());
        webhook_channel.send(alert).await
    }

    async fn store_in_app(&self, alert: &Alert) -> Result<()> {
        tracing::info!("In-app alert stored: {} - {}", alert.title, alert.message);
        Ok(())
    }

    pub async fn send_near_limit_alert(
        &self,
        workspace_id: Uuid,
        current_count: i32,
        limit: i32,
    ) -> Result<()> {
        self.send_alert(
            workspace_id,
            AlertType::NearLimit,
            AlertSeverity::Warning,
            "Rotation Limit Warning".to_string(),
            format!(
                "You have used {} of {} allowed rotations this period",
                current_count, limit
            ),
            None,
            None,
            serde_json::json!({
                "current_count": current_count,
                "limit": limit
            }),
        )
        .await
    }

    pub async fn send_rotation_success_alert(
        &self,
        workspace_id: Uuid,
        provider: &str,
        secret_name: &str,
    ) -> Result<()> {
        self.send_alert(
            workspace_id,
            AlertType::RotationSuccess,
            AlertSeverity::Info,
            "Rotation Successful".to_string(),
            format!(
                "Successfully rotated secret '{}' for provider '{}'",
                secret_name, provider
            ),
            Some(provider.to_string()),
            Some(secret_name.to_string()),
            serde_json::json!({}),
        )
        .await
    }

    pub async fn send_rotation_failure_alert(
        &self,
        workspace_id: Uuid,
        provider: &str,
        secret_name: &str,
        error: &str,
    ) -> Result<()> {
        self.send_alert(
            workspace_id,
            AlertType::RotationFailure,
            AlertSeverity::Error,
            "Rotation Failed".to_string(),
            format!(
                "Failed to rotate secret '{}' for provider '{}': {}",
                secret_name, provider, error
            ),
            Some(provider.to_string()),
            Some(secret_name.to_string()),
            serde_json::json!({
                "error": error
            }),
        )
        .await
    }

    pub async fn send_approval_request_alert(
        &self,
        workspace_id: Uuid,
        requester: &str,
        provider: &str,
        secret_name: &str,
        approval_id: Uuid,
    ) -> Result<()> {
        self.send_alert(
            workspace_id,
            AlertType::ApprovalRequest,
            AlertSeverity::Warning,
            "Approval Required".to_string(),
            format!(
                "{} requested approval to rotate '{}' for provider '{}'",
                requester, secret_name, provider
            ),
            Some(provider.to_string()),
            Some(secret_name.to_string()),
            serde_json::json!({
                "requester": requester,
                "approval_id": approval_id
            }),
        )
        .await
    }
}
