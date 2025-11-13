use anyhow::{Context, Result};
use html_escape::encode_text;
use resend_rs::mail::Mail;
use resend_rs::resend_client::ResendClient;
use serde_json::Value as JsonValue;

use crate::alerts::models::Alert;

pub struct EmailChannel {
    resend: ResendClient,
    from_email: String,
    from_name: Option<String>,
}

impl EmailChannel {
    pub fn new(config: JsonValue) -> Result<Self> {
        let api_key = config["api_key"]
            .as_str()
            .map(|s| s.to_string())
            .or_else(|| std::env::var("RESEND_API_KEY").ok())
            .ok_or_else(|| anyhow::anyhow!("Missing Resend API key. Provide 'api_key' in config or set RESEND_API_KEY environment variable"))?;

        let resend = ResendClient::new(api_key);

        let from_email = config["from_email"]
            .as_str()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "alerts@birch.dev".to_string());

        let from_name = config["from_name"].as_str().map(|s| s.to_string());

        Ok(Self {
            resend,
            from_email,
            from_name,
        })
    }

    pub async fn send(&self, alert: &Alert, recipients: Vec<String>) -> Result<()> {
        if recipients.is_empty() {
            anyhow::bail!("No recipients provided for email alert");
        }

        let from = if let Some(name) = &self.from_name {
            format!("{} <{}>", name, self.from_email)
        } else {
            self.from_email.clone()
        };

        let severity_color = match alert.severity {
            crate::alerts::models::AlertSeverity::Info => "#36a64f",
            crate::alerts::models::AlertSeverity::Warning => "#ff9900",
            crate::alerts::models::AlertSeverity::Error => "#ff0000",
            crate::alerts::models::AlertSeverity::Critical => "#8b0000",
        };

        let severity_text = format!("{:?}", alert.severity);

        let html_body = format!(
            r#"
            <!DOCTYPE html>
            <html>
            <head>
                <meta charset="utf-8">
                <style>
                    body {{ font-family: Arial, sans-serif; line-height: 1.6; color: #333; }}
                    .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
                    .header {{ background-color: {}; color: white; padding: 20px; border-radius: 5px 5px 0 0; }}
                    .content {{ background-color: #f9f9f9; padding: 20px; border-radius: 0 0 5px 5px; }}
                    .alert-title {{ font-size: 24px; margin: 0 0 10px 0; }}
                    .alert-message {{ font-size: 16px; margin: 10px 0; }}
                    .metadata {{ margin-top: 20px; padding-top: 20px; border-top: 1px solid #ddd; }}
                    .metadata-item {{ margin: 5px 0; }}
                    .label {{ font-weight: bold; }}
                </style>
            </head>
            <body>
                <div class="container">
                    <div class="header">
                        <h1 class="alert-title">{}</h1>
                        <p style="margin: 0;">Severity: {}</p>
                    </div>
                    <div class="content">
                        <div class="alert-message">{}</div>
                        <div class="metadata">
                            <div class="metadata-item"><span class="label">Type:</span> {:?}</div>
                            {}
                            {}
                            <div class="metadata-item"><span class="label">Time:</span> {}</div>
                        </div>
                    </div>
                </div>
            </body>
            </html>
            "#,
            severity_color,
            encode_text(&alert.title),
            severity_text,
            encode_text(&alert.message),
            alert.alert_type,
            alert
                .provider
                .as_ref()
                .map(|p| format!(
                    "<div class=\"metadata-item\"><span class=\"label\">Provider:</span> {}</div>",
                    encode_text(p)
                ))
                .unwrap_or_default(),
            alert
                .secret_name
                .as_ref()
                .map(|s| format!(
                    "<div class=\"metadata-item\"><span class=\"label\">Secret:</span> {}</div>",
                    encode_text(s)
                ))
                .unwrap_or_default(),
            alert.created_at.format("%Y-%m-%d %H:%M:%S UTC")
        );

        for recipient in &recipients {
            let mail = Mail::new(&from, recipient, &alert.title, &html_body);
            self.resend
                .send_async(mail)
                .await
                .context("Failed to send email via Resend")?;
        }

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
