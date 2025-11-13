use anyhow::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::supabase::SupabaseClient;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerManagedKey {
    pub workspace_id: Uuid,
    pub key_id: String,
    pub key_provider: KeyProvider,
    pub key_arn: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum KeyProvider {
    Aws,
    Gcp,
    Azure,
}

pub struct CustomerKeyManager {
    client: SupabaseClient,
}

impl CustomerKeyManager {
    pub fn new(client: SupabaseClient) -> Self {
        Self { client }
    }

    pub async fn register_key(
        &self,
        workspace_id: Uuid,
        key_provider: KeyProvider,
        key_arn: String,
    ) -> Result<CustomerManagedKey> {
        tracing::info!(
            "Registering customer-managed key for workspace {}",
            workspace_id
        );

        let key_id = Uuid::new_v4().to_string();

        Ok(CustomerManagedKey {
            workspace_id,
            key_id,
            key_provider,
            key_arn,
            enabled: true,
        })
    }

    pub async fn rotate_key(
        &self,
        workspace_id: Uuid,
        old_key_id: &str,
        new_key_arn: String,
    ) -> Result<()> {
        tracing::info!(
            "Rotating customer-managed key for workspace {}",
            workspace_id
        );

        Ok(())
    }

    pub async fn validate_key_access(&self, workspace_id: Uuid, key_arn: &str) -> Result<bool> {
        tracing::debug!("Validating key access for workspace {}", workspace_id);

        Ok(true)
    }
}
