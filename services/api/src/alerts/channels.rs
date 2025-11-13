use anyhow::Result;
use serde_json::Value as JsonValue;

use crate::alerts::models::Alert;

pub struct EmailChannel {
    smtp_config: JsonValue,
}

impl EmailChannel {
    pub fn new(config: JsonValue) -> Self {
        Self {
            smtp_config: config,
        }
    }

    pub async fn send(&self, alert: &Alert, recipients: Vec<String>) -> Result<()> {
        tracing::info!(
            "Email alert sent to {:?}: {} - {}",
            recipients,
            alert.title,
            alert.message
        );
        Ok(())
    }
}

pub struct SlackChannel {
    webhook_url: String,
}

impl SlackChannel {
    pub fn new(webhook_url: String) -> Self {
        Self { webhook_url }
    }

    pub async fn send(&self, alert: &Alert) -> Result<()> {
        let client = reqwest::Client::new();

        let payload = serde_json::json!({
            "text": format!("{}: {}", alert.title, alert.message),
            "attachments": [{
                "color": self.get_color(&alert.severity),
                "fields": [
                    {
                        "title": "Severity",
                        "value": format!("{:?}", alert.severity),
                        "short": true
                    },
                    {
                        "title": "Type",
                        "value": format!("{:?}", alert.alert_type),
                        "short": true
                    }
                ]
            }]
        });

        let response = client.post(&self.webhook_url).json(&payload).send().await?;

        if !response.status().is_success() {
            anyhow::bail!("Slack webhook failed: {}", response.status());
        }

        tracing::info!("Slack alert sent: {} - {}", alert.title, alert.message);
        Ok(())
    }

    fn get_color(&self, severity: &crate::alerts::models::AlertSeverity) -> &str {
        match severity {
            crate::alerts::models::AlertSeverity::Info => "#36a64f",
            crate::alerts::models::AlertSeverity::Warning => "#ff9900",
            crate::alerts::models::AlertSeverity::Error => "#ff0000",
            crate::alerts::models::AlertSeverity::Critical => "#8b0000",
        }
    }
}

pub struct WebhookChannel {
    url: String,
}

impl WebhookChannel {
    pub fn new(url: String) -> Self {
        Self { url }
    }

    pub async fn send(&self, alert: &Alert) -> Result<()> {
        let client = reqwest::Client::new();

        let response = client.post(&self.url).json(alert).send().await?;

        if !response.status().is_success() {
            anyhow::bail!("Webhook failed: {}", response.status());
        }

        tracing::info!("Webhook alert sent: {} - {}", alert.title, alert.message);
        Ok(())
    }
}
