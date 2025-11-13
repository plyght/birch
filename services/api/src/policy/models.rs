use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PolicyScope {
    Workspace,
    Provider,
    Secret,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub priority: i32,
    pub enabled: bool,
    pub scope: PolicyScope,
    pub provider_pattern: Option<String>,
    pub secret_pattern: Option<String>,
    pub rules: PolicyRules,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyRules {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_limits: Option<RotationLimits>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_windows: Option<Vec<MaintenanceWindow>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_first: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_approval: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_redeploy: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_environments: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotationLimits {
    pub soft_limit: Option<i32>,
    pub hard_limit: Option<i32>,
    pub period: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceWindow {
    pub day_of_week: Option<String>,
    pub start_time: String,
    pub end_time: String,
    pub timezone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyEvaluationContext {
    pub workspace_id: Uuid,
    pub provider: String,
    pub secret_name: String,
    pub environment: Option<String>,
    pub current_rotation_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyEvaluationResult {
    pub policy_id: Uuid,
    pub policy_name: String,
    pub passed: bool,
    pub reason: Option<String>,
    pub action: PolicyAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PolicyAction {
    Allow,
    Warn,
    Block,
    RequireApproval,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyEvaluationSummary {
    pub allowed: bool,
    pub requires_approval: bool,
    pub warnings: Vec<String>,
    pub blocking_reasons: Vec<String>,
    pub results: Vec<PolicyEvaluationResult>,
}
