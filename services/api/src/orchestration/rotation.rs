use anyhow::Result;
use uuid::Uuid;

use crate::orchestration::connector::{ConnectorConfig, ConnectorOrchestrator, RotationRequest};
use crate::policy::{PolicyEngine, PolicyEvaluationContext};
use crate::supabase::SupabaseClient;

pub struct RotationOrchestrator {
    policy_engine: PolicyEngine,
    connector_orchestrator: ConnectorOrchestrator,
}

impl RotationOrchestrator {
    pub fn new(client: SupabaseClient) -> Self {
        Self {
            policy_engine: PolicyEngine::new(client),
            connector_orchestrator: ConnectorOrchestrator::new(),
        }
    }

    pub async fn execute_rotation(
        &self,
        workspace_id: Uuid,
        provider: &str,
        secret_name: &str,
        environment: &str,
        config: &ConnectorConfig,
        dry_run: bool,
    ) -> Result<serde_json::Value> {
        let rotation_count = self
            .policy_engine
            .get_rotation_count(workspace_id, 30)
            .await?;

        let context = PolicyEvaluationContext {
            workspace_id,
            provider: provider.to_string(),
            secret_name: secret_name.to_string(),
            environment: Some(environment.to_string()),
            current_rotation_count: rotation_count,
        };

        let policy_summary = self.policy_engine.evaluate_policies(&context).await?;

        if !policy_summary.allowed {
            return Ok(serde_json::json!({
                "success": false,
                "blocked": true,
                "reasons": policy_summary.blocking_reasons,
                "policy_results": policy_summary.results
            }));
        }

        if policy_summary.requires_approval {
            return Ok(serde_json::json!({
                "success": false,
                "requires_approval": true,
                "warnings": policy_summary.warnings,
                "policy_results": policy_summary.results
            }));
        }

        let request = RotationRequest {
            workspace_id,
            provider: provider.to_string(),
            secret_name: secret_name.to_string(),
            environment: environment.to_string(),
            dry_run,
        };

        let result = self
            .connector_orchestrator
            .rotate_secret(&request, config)
            .await?;

        Ok(serde_json::json!({
            "success": result.success,
            "rotation_result": result,
            "warnings": policy_summary.warnings,
            "policy_results": policy_summary.results
        }))
    }
}
