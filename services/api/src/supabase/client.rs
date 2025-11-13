use anyhow::Result;
use deadpool_postgres::{Config, Manager, ManagerConfig, Pool, RecyclingMethod, Runtime};
use native_tls::TlsConnector;
use postgres_native_tls::MakeTlsConnector;
use tokio_postgres::NoTls;
use uuid::Uuid;

#[derive(Clone)]
pub struct SupabaseClient {
    pool: Pool,
}

#[derive(Debug)]
pub struct ApiKeyRecord {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub user_id: Uuid,
}

impl SupabaseClient {
    pub async fn new(database_url: &str) -> Result<Self> {
        let mut cfg = Config::new();
        cfg.url = Some(database_url.to_string());
        cfg.manager = Some(ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        });

        let pool = if database_url.contains("localhost") || database_url.contains("127.0.0.1") {
            cfg.create_pool(Some(Runtime::Tokio1), NoTls)?
        } else {
            let connector = TlsConnector::builder().build()?;
            let connector = MakeTlsConnector::new(connector);
            cfg.create_pool(Some(Runtime::Tokio1), connector)?
        };

        Ok(Self { pool })
    }

    pub fn pool(&self) -> &Pool {
        &self.pool
    }

    pub async fn get_client(&self) -> Result<deadpool_postgres::Client> {
        Ok(self.pool.get().await?)
    }

    pub async fn get_api_key_by_hash(&self, api_key: &str) -> Result<ApiKeyRecord> {
        use crate::auth::api_keys::ApiKeyService;

        let db_client = self.get_client().await?;

        let stmt = db_client
            .prepare(
                "SELECT id, workspace_id, key_hash, revoked_at FROM api_keys
                 WHERE revoked_at IS NULL",
            )
            .await?;

        let rows = db_client.query(&stmt, &[]).await?;

        for row in rows {
            let key_hash: String = row.get(2);
            if ApiKeyService::verify_api_key(api_key, &key_hash).unwrap_or(false) {
                let key_id: Uuid = row.get(0);
                let workspace_id: Uuid = row.get(1);

                let update_stmt = db_client
                    .prepare("UPDATE api_keys SET last_used_at = NOW() WHERE id = $1")
                    .await?;
                let _ = db_client.execute(&update_stmt, &[&key_id]).await;

                let user_id = self.get_workspace_owner(workspace_id).await?;
                return Ok(ApiKeyRecord {
                    id: key_id,
                    workspace_id,
                    user_id,
                });
            }
        }

        anyhow::bail!("Invalid API key")
    }

    async fn get_workspace_owner(&self, workspace_id: Uuid) -> Result<Uuid> {
        let db_client = self.get_client().await?;

        let stmt = db_client
            .prepare(
                "SELECT user_id FROM workspace_members
                 WHERE workspace_id = $1 AND role = 'owner'
                 LIMIT 1",
            )
            .await?;

        let rows = db_client.query(&stmt, &[&workspace_id]).await?;

        if rows.is_empty() {
            anyhow::bail!("No owner found for workspace");
        }

        Ok(rows[0].get(0))
    }
}
