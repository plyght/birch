use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::supabase::SupabaseClient;
use crate::vault::encryption::VaultEncryption;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthToken {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub provider: String,
    pub encrypted_refresh_token: Vec<u8>,
    pub access_token: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct OAuthHandler {
    client: SupabaseClient,
    encryption: VaultEncryption,
}

impl OAuthHandler {
    pub fn new(client: SupabaseClient, encryption: VaultEncryption) -> Self {
        Self { client, encryption }
    }

    pub async fn store_refresh_token(
        &self,
        workspace_id: Uuid,
        provider: &str,
        refresh_token: &str,
    ) -> Result<()> {
        let encrypted = self
            .encryption
            .encrypt(workspace_id, refresh_token.as_bytes())?;

        let db_client = self.client.get_client().await?;

        let stmt = db_client
            .prepare(
                "INSERT INTO oauth_tokens (workspace_id, provider, encrypted_refresh_token)
                 VALUES ($1, $2, $3)
                 ON CONFLICT (workspace_id, provider)
                 DO UPDATE SET encrypted_refresh_token = EXCLUDED.encrypted_refresh_token,
                               updated_at = NOW()",
            )
            .await?;

        db_client
            .execute(&stmt, &[&workspace_id, &provider, &encrypted])
            .await?;

        Ok(())
    }

    async fn get_refresh_token(&self, workspace_id: Uuid, provider: &str) -> Result<String> {
        let db_client = self.client.get_client().await?;

        let stmt = db_client
            .prepare(
                "SELECT encrypted_refresh_token FROM oauth_tokens
                 WHERE workspace_id = $1 AND provider = $2",
            )
            .await?;

        let rows = db_client.query(&stmt, &[&workspace_id, &provider]).await?;

        if rows.is_empty() {
            anyhow::bail!("OAuth refresh token not found for provider: {}", provider);
        }

        let encrypted: Vec<u8> = rows[0].get(0);
        let decrypted = self.encryption.decrypt(workspace_id, &encrypted)?;

        String::from_utf8(decrypted).context("Failed to decode refresh token as UTF-8")
    }

    pub async fn get_cached_access_token(
        &self,
        workspace_id: Uuid,
        provider: &str,
    ) -> Result<Option<String>> {
        let db_client = self.client.get_client().await?;

        let stmt = db_client
            .prepare(
                "SELECT access_token, expires_at FROM oauth_tokens
                 WHERE workspace_id = $1 AND provider = $2",
            )
            .await?;

        let rows = db_client.query(&stmt, &[&workspace_id, &provider]).await?;

        if rows.is_empty() {
            return Ok(None);
        }

        let access_token: Option<String> = rows[0].get(0);
        let expires_at: Option<DateTime<Utc>> = rows[0].get(1);

        if let (Some(token), Some(expiry)) = (access_token, expires_at) {
            if expiry > Utc::now() {
                return Ok(Some(token));
            }
        }

        Ok(None)
    }

    pub async fn cache_access_token(
        &self,
        workspace_id: Uuid,
        provider: &str,
        access_token: &str,
        expires_in_seconds: i64,
    ) -> Result<()> {
        let expires_at = Utc::now() + chrono::Duration::seconds(expires_in_seconds);

        let db_client = self.client.get_client().await?;

        let stmt = db_client
            .prepare(
                "UPDATE oauth_tokens
                 SET access_token = $1, expires_at = $2, updated_at = NOW()
                 WHERE workspace_id = $3 AND provider = $4",
            )
            .await?;

        db_client
            .execute(
                &stmt,
                &[&access_token, &expires_at, &workspace_id, &provider],
            )
            .await?;

        Ok(())
    }

    pub async fn exchange_refresh_token(
        &self,
        workspace_id: Uuid,
        provider: &str,
    ) -> Result<String> {
        if let Some(cached) = self.get_cached_access_token(workspace_id, provider).await? {
            tracing::debug!("Using cached OAuth access token for {}", provider);
            return Ok(cached);
        }

        let refresh_token = self.get_refresh_token(workspace_id, provider).await?;

        let access_token = match provider {
            "github" => self.exchange_github_token(&refresh_token).await?,
            "gitlab" => self.exchange_gitlab_token(&refresh_token).await?,
            "vercel" => {
                return Ok(refresh_token);
            }
            _ => anyhow::bail!("OAuth provider not supported: {}", provider),
        };

        let expires_in = 3600;
        self.cache_access_token(workspace_id, provider, &access_token, expires_in)
            .await?;

        Ok(access_token)
    }

    async fn exchange_github_token(&self, refresh_token: &str) -> Result<String> {
        Ok(refresh_token.to_string())
    }

    async fn exchange_gitlab_token(&self, refresh_token: &str) -> Result<String> {
        Ok(refresh_token.to_string())
    }
}
