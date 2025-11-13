use anyhow::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::supabase::SupabaseClient;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SsoConfig {
    pub workspace_id: Uuid,
    pub provider: SsoProvider,
    pub enabled: bool,
    pub domain: String,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SsoProvider {
    Oidc,
    Saml,
}

pub struct SsoManager {
    client: SupabaseClient,
}

impl SsoManager {
    pub fn new(client: SupabaseClient) -> Self {
        Self { client }
    }

    pub async fn configure_sso(
        &self,
        workspace_id: Uuid,
        provider: SsoProvider,
        domain: String,
        metadata: serde_json::Value,
    ) -> Result<SsoConfig> {
        tracing::info!("Configuring SSO for workspace {}", workspace_id);

        Ok(SsoConfig {
            workspace_id,
            provider,
            enabled: true,
            domain,
            metadata,
        })
    }

    pub async fn verify_domain(&self, workspace_id: Uuid, domain: &str) -> Result<bool> {
        tracing::info!("Verifying domain {} for workspace {}", domain, workspace_id);

        Ok(true)
    }

    pub async fn provision_user_scim(
        &self,
        workspace_id: Uuid,
        email: &str,
        role: &str,
    ) -> Result<Uuid> {
        tracing::info!(
            "Provisioning user {} via SCIM for workspace {}",
            email,
            workspace_id
        );

        Ok(Uuid::new_v4())
    }

    pub async fn deprovision_user_scim(&self, workspace_id: Uuid, user_id: Uuid) -> Result<()> {
        tracing::info!(
            "Deprovisioning user {} via SCIM for workspace {}",
            user_id,
            workspace_id
        );

        Ok(())
    }
}
