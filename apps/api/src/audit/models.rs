use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ActorType {
    User,
    ApiKey,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub actor_id: Option<Uuid>,
    pub actor_type: ActorType,
    pub actor_identifier: String,
    pub action: String,
    pub resource_type: String,
    pub resource_id: Option<String>,
    pub provider: Option<String>,
    pub secret_name: Option<String>,
    pub environment: Option<String>,
    pub success: bool,
    pub error_message: Option<String>,
    pub policy_results: Option<serde_json::Value>,
    pub metadata: serde_json::Value,
    pub signature: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAuditLog {
    pub workspace_id: Uuid,
    pub actor_id: Option<Uuid>,
    pub actor_type: ActorType,
    pub actor_identifier: String,
    pub action: String,
    pub resource_type: String,
    pub resource_id: Option<String>,
    pub provider: Option<String>,
    pub secret_name: Option<String>,
    pub environment: Option<String>,
    pub success: bool,
    pub error_message: Option<String>,
    pub policy_results: Option<serde_json::Value>,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExportFormat {
    Json,
    Csv,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditExportRequest {
    pub workspace_id: Uuid,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub format: ExportFormat,
    pub filters: AuditFilters,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuditFilters {
    pub actor_id: Option<Uuid>,
    pub action: Option<String>,
    pub resource_type: Option<String>,
    pub provider: Option<String>,
    pub success: Option<bool>,
}
