use anyhow::Result;
use std::time::Duration;
use tokio::time::sleep;
use uuid::Uuid;

use crate::credentials::api_key::ApiKeyHandler;
use crate::credentials::cache::CredentialCache;
use crate::credentials::circuit_breaker::CircuitBreaker;
use crate::credentials::health::HealthMonitor;
use crate::credentials::kms::KmsHandler;
use crate::credentials::modes::CredentialMode;
use crate::credentials::oauth::OAuthHandler;
use crate::supabase::SupabaseClient;
use crate::vault::encryption::VaultEncryption;
use crate::vault::storage::VaultStorage;

const MAX_RETRIES: u32 = 3;
const INITIAL_BACKOFF_MS: u64 = 100;

pub struct CredentialResolver {
    client: SupabaseClient,
    vault: VaultStorage,
    cache: CredentialCache,
    oauth_handler: OAuthHandler,
    kms_handler: KmsHandler,
    api_key_handler: ApiKeyHandler,
    circuit_breaker: CircuitBreaker,
    health_monitor: HealthMonitor,
}

impl CredentialResolver {
    pub fn new(client: SupabaseClient, vault: VaultStorage, cache: CredentialCache) -> Self {
        let encryption = VaultEncryption::new().expect("Failed to initialize encryption");
        let oauth_handler = OAuthHandler::new(client.clone(), encryption);
        let kms_handler = KmsHandler::new(client.clone());
        let api_key_handler = ApiKeyHandler::new(client.clone());
        let circuit_breaker = CircuitBreaker::default();
        let health_monitor = HealthMonitor::new(client.clone());

        Self {
            client,
            vault,
            cache,
            oauth_handler,
            kms_handler,
            api_key_handler,
            circuit_breaker,
            health_monitor,
        }
    }

    async fn get_provider_mode(
        &self,
        workspace_id: &Uuid,
        provider: &str,
    ) -> Result<CredentialMode> {
        let db_client = self.client.get_client().await?;

        let stmt = db_client
            .prepare(
                "SELECT mode FROM provider_configs
                 WHERE workspace_id = $1 AND provider = $2",
            )
            .await?;

        let rows = db_client.query(&stmt, &[workspace_id, &provider]).await?;

        if rows.is_empty() {
            return Ok(CredentialMode::Hosted);
        }

        let mode_str: String = rows[0].get(0);
        mode_str.parse()
    }

    pub async fn resolve(
        &mut self,
        workspace_id: &Uuid,
        provider: &str,
        secret_name: &str,
    ) -> Result<String> {
        if let Some(cached) = self.cache.get(workspace_id, provider, secret_name).await? {
            tracing::debug!("Cache hit for credential");
            return Ok(cached);
        }

        let mode = self.get_provider_mode(workspace_id, provider).await?;

        let credential = match mode {
            CredentialMode::Hosted => {
                self.resolve_hosted(workspace_id, provider, secret_name)
                    .await?
            }
            CredentialMode::OAuth => {
                self.resolve_oauth(*workspace_id, provider, secret_name)
                    .await?
            }
            CredentialMode::Kms => {
                self.resolve_kms(*workspace_id, provider, secret_name)
                    .await?
            }
            CredentialMode::ApiKey => {
                self.resolve_api_key(*workspace_id, provider, secret_name)
                    .await?
            }
        };

        self.cache
            .set(workspace_id, provider, secret_name, &credential)
            .await?;

        Ok(credential)
    }

    async fn resolve_hosted(
        &self,
        workspace_id: &Uuid,
        provider: &str,
        secret_name: &str,
    ) -> Result<String> {
        let mut attempt = 0;

        loop {
            match self
                .vault
                .get_credential(*workspace_id, provider, secret_name)
                .await
            {
                Ok(Some(credential)) => return Ok(credential),
                Ok(None) => anyhow::bail!("Credential not found"),
                Err(e) => {
                    attempt += 1;
                    if attempt >= MAX_RETRIES {
                        return Err(e);
                    }

                    let backoff = INITIAL_BACKOFF_MS * 2_u64.pow(attempt - 1);
                    tracing::warn!(
                        "Credential resolution failed (attempt {}), retrying in {}ms",
                        attempt,
                        backoff
                    );
                    sleep(Duration::from_millis(backoff)).await;
                }
            }
        }
    }

    async fn resolve_oauth(
        &self,
        workspace_id: Uuid,
        provider: &str,
        _secret_name: &str,
    ) -> Result<String> {
        let circuit_key = format!("oauth:{}:{}", workspace_id, provider);

        if !self.circuit_breaker.can_attempt(&circuit_key) {
            self.health_monitor
                .record_failure(workspace_id, provider, "oauth", "Circuit breaker open")
                .await?;
            anyhow::bail!("Circuit breaker open for OAuth provider: {}", provider);
        }

        let mut attempt = 0;

        loop {
            match self
                .oauth_handler
                .exchange_refresh_token(workspace_id, provider)
                .await
            {
                Ok(access_token) => {
                    self.circuit_breaker.record_success(&circuit_key);
                    self.health_monitor
                        .record_success(workspace_id, provider, "oauth")
                        .await?;
                    return Ok(access_token);
                }
                Err(e) => {
                    attempt += 1;
                    if attempt >= MAX_RETRIES {
                        self.circuit_breaker.record_failure(&circuit_key);
                        self.health_monitor
                            .record_failure(workspace_id, provider, "oauth", &e.to_string())
                            .await?;
                        return Err(e);
                    }

                    let backoff = INITIAL_BACKOFF_MS * 2_u64.pow(attempt - 1);
                    tracing::warn!(
                        "OAuth token exchange failed (attempt {}), retrying in {}ms",
                        attempt,
                        backoff
                    );
                    sleep(Duration::from_millis(backoff)).await;
                }
            }
        }
    }

    async fn resolve_kms(
        &self,
        workspace_id: Uuid,
        provider: &str,
        secret_name: &str,
    ) -> Result<String> {
        let circuit_key = format!("kms:{}:{}", workspace_id, provider);

        if !self.circuit_breaker.can_attempt(&circuit_key) {
            self.health_monitor
                .record_failure(workspace_id, provider, "kms", "Circuit breaker open")
                .await?;

            tracing::warn!("Circuit breaker open for KMS, attempting fallback to hosted mode");
            return self
                .resolve_hosted(&workspace_id, provider, secret_name)
                .await;
        }

        let mut attempt = 0;

        loop {
            match self
                .kms_handler
                .resolve_from_kms(workspace_id, provider, secret_name)
                .await
            {
                Ok(credential) => {
                    self.circuit_breaker.record_success(&circuit_key);
                    self.health_monitor
                        .record_success(workspace_id, provider, "kms")
                        .await?;
                    return Ok(credential);
                }
                Err(e) => {
                    attempt += 1;
                    if attempt >= MAX_RETRIES {
                        self.circuit_breaker.record_failure(&circuit_key);
                        self.health_monitor
                            .record_failure(workspace_id, provider, "kms", &e.to_string())
                            .await?;

                        tracing::error!(
                            "KMS resolution failed after {} attempts, attempting fallback",
                            MAX_RETRIES
                        );
                        return self
                            .resolve_hosted(&workspace_id, provider, secret_name)
                            .await;
                    }

                    let backoff = INITIAL_BACKOFF_MS * 2_u64.pow(attempt - 1);
                    tracing::warn!(
                        "KMS resolution failed (attempt {}), retrying in {}ms",
                        attempt,
                        backoff
                    );
                    sleep(Duration::from_millis(backoff)).await;
                }
            }
        }
    }

    async fn resolve_api_key(
        &self,
        workspace_id: Uuid,
        provider: &str,
        secret_name: &str,
    ) -> Result<String> {
        self.api_key_handler
            .fetch_credential(workspace_id, provider, secret_name)
            .await
    }

    pub async fn invalidate_cache(
        &mut self,
        workspace_id: Uuid,
        provider: &str,
        secret_name: &str,
    ) -> Result<()> {
        self.cache
            .invalidate(&workspace_id, provider, secret_name)
            .await
    }
}
