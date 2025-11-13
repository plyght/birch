use anyhow::Result;
use chrono::Utc;
use ed25519_dalek::{Signature, Signer, SigningKey};
use rand::rngs::OsRng;
use serde_json::Value as JsonValue;
use uuid::Uuid;

use crate::audit::models::*;
use crate::supabase::SupabaseClient;

pub struct AuditLogger {
    client: SupabaseClient,
    signing_key: SigningKey,
}

impl AuditLogger {
    pub fn new(client: SupabaseClient) -> Self {
        let signing_key = SigningKey::generate(&mut OsRng);
        Self {
            client,
            signing_key,
        }
    }

    pub async fn log(&self, log_request: CreateAuditLog) -> Result<AuditLog> {
        let timestamp = Utc::now();
        let id = Uuid::new_v4();

        let payload = serde_json::json!({
            "id": id,
            "workspace_id": log_request.workspace_id,
            "timestamp": timestamp,
            "actor_type": log_request.actor_type,
            "actor_identifier": log_request.actor_identifier,
            "action": log_request.action,
            "resource_type": log_request.resource_type,
            "resource_id": log_request.resource_id,
            "success": log_request.success,
        });

        let signature = self.sign_payload(&payload)?;

        let db_client = self.client.get_client().await?;

        let actor_type_str = match log_request.actor_type {
            ActorType::User => "user",
            ActorType::ApiKey => "api_key",
            ActorType::System => "system",
        };

        let stmt = db_client
            .prepare(
                "INSERT INTO audit_logs 
                 (id, workspace_id, timestamp, actor_id, actor_type, actor_identifier,
                  action, resource_type, resource_id, provider, secret_name, environment,
                  success, error_message, policy_results, metadata, signature)
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
                 RETURNING id, workspace_id, timestamp, actor_id, actor_type, actor_identifier,
                           action, resource_type, resource_id, provider, secret_name, environment,
                           success, error_message, policy_results, metadata, signature, created_at",
            )
            .await?;

        let row = db_client
            .query_one(
                &stmt,
                &[
                    &id,
                    &log_request.workspace_id,
                    &timestamp,
                    &log_request.actor_id,
                    &actor_type_str,
                    &log_request.actor_identifier,
                    &log_request.action,
                    &log_request.resource_type,
                    &log_request.resource_id,
                    &log_request.provider,
                    &log_request.secret_name,
                    &log_request.environment,
                    &log_request.success,
                    &log_request.error_message,
                    &log_request.policy_results,
                    &log_request.metadata,
                    &signature,
                ],
            )
            .await?;

        Ok(self.row_to_audit_log(&row)?)
    }

    pub async fn list_logs(
        &self,
        workspace_id: Uuid,
        filters: AuditFilters,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<AuditLog>> {
        let db_client = self.client.get_client().await?;

        let mut query = String::from(
            "SELECT id, workspace_id, timestamp, actor_id, actor_type, actor_identifier,
                    action, resource_type, resource_id, provider, secret_name, environment,
                    success, error_message, policy_results, metadata, signature, created_at
             FROM audit_logs
             WHERE workspace_id = $1",
        );

        let mut param_count = 1;
        let mut params: Vec<Box<dyn tokio_postgres::types::ToSql + Sync>> =
            vec![Box::new(workspace_id)];

        if let Some(actor_id) = filters.actor_id {
            param_count += 1;
            query.push_str(&format!(" AND actor_id = ${}", param_count));
            params.push(Box::new(actor_id));
        }

        if let Some(action) = filters.action {
            param_count += 1;
            query.push_str(&format!(" AND action = ${}", param_count));
            params.push(Box::new(action));
        }

        if let Some(resource_type) = filters.resource_type {
            param_count += 1;
            query.push_str(&format!(" AND resource_type = ${}", param_count));
            params.push(Box::new(resource_type));
        }

        if let Some(provider) = filters.provider {
            param_count += 1;
            query.push_str(&format!(" AND provider = ${}", param_count));
            params.push(Box::new(provider));
        }

        if let Some(success) = filters.success {
            param_count += 1;
            query.push_str(&format!(" AND success = ${}", param_count));
            params.push(Box::new(success));
        }

        query.push_str(&format!(
            " ORDER BY timestamp DESC LIMIT ${} OFFSET ${}",
            param_count + 1,
            param_count + 2
        ));
        params.push(Box::new(limit));
        params.push(Box::new(offset));

        let stmt = db_client.prepare(&query).await?;
        let param_refs: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> =
            params.iter().map(|p| p.as_ref()).collect();
        let rows = db_client.query(&stmt, &param_refs[..]).await?;

        let mut logs = Vec::new();
        for row in rows {
            logs.push(self.row_to_audit_log(&row)?);
        }

        Ok(logs)
    }

    pub async fn export_logs(&self, request: AuditExportRequest) -> Result<String> {
        let start = request
            .start_date
            .unwrap_or_else(|| Utc::now() - chrono::Duration::days(30));
        let end = request.end_date.unwrap_or_else(|| Utc::now());

        let db_client = self.client.get_client().await?;

        let stmt = db_client
            .prepare(
                "SELECT id, workspace_id, timestamp, actor_id, actor_type, actor_identifier,
                        action, resource_type, resource_id, provider, secret_name, environment,
                        success, error_message, policy_results, metadata, signature, created_at
                 FROM audit_logs
                 WHERE workspace_id = $1 AND timestamp >= $2 AND timestamp <= $3
                 ORDER BY timestamp DESC",
            )
            .await?;

        let rows = db_client
            .query(&stmt, &[&request.workspace_id, &start, &end])
            .await?;

        let mut logs = Vec::new();
        for row in rows {
            logs.push(self.row_to_audit_log(&row)?);
        }

        match request.format {
            ExportFormat::Json => Ok(serde_json::to_string_pretty(&logs)?),
            ExportFormat::Csv => self.export_as_csv(logs),
        }
    }

    fn export_as_csv(&self, logs: Vec<AuditLog>) -> Result<String> {
        let mut csv = String::from("timestamp,actor,action,resource_type,resource_id,provider,secret_name,environment,success\n");

        for log in logs {
            csv.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                log.timestamp,
                log.actor_identifier,
                log.action,
                log.resource_type,
                log.resource_id.unwrap_or_default(),
                log.provider.unwrap_or_default(),
                log.secret_name.unwrap_or_default(),
                log.environment.unwrap_or_default(),
                log.success
            ));
        }

        Ok(csv)
    }

    fn sign_payload(&self, payload: &JsonValue) -> Result<String> {
        let payload_str = serde_json::to_string(payload)?;
        let signature: Signature = self.signing_key.sign(payload_str.as_bytes());
        Ok(hex::encode(signature.to_bytes()))
    }

    fn row_to_audit_log(&self, row: &tokio_postgres::Row) -> Result<AuditLog> {
        let actor_type_str: String = row.get(4);
        let actor_type = match actor_type_str.as_str() {
            "user" => ActorType::User,
            "api_key" => ActorType::ApiKey,
            "system" => ActorType::System,
            _ => anyhow::bail!("Invalid actor type: {}", actor_type_str),
        };

        Ok(AuditLog {
            id: row.get(0),
            workspace_id: row.get(1),
            timestamp: row.get(2),
            actor_id: row.get(3),
            actor_type,
            actor_identifier: row.get(5),
            action: row.get(6),
            resource_type: row.get(7),
            resource_id: row.get(8),
            provider: row.get(9),
            secret_name: row.get(10),
            environment: row.get(11),
            success: row.get(12),
            error_message: row.get(13),
            policy_results: row.get(14),
            metadata: row.get(15),
            signature: row.get(16),
            created_at: row.get(17),
        })
    }
}
