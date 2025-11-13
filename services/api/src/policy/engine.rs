use anyhow::Result;
use serde_json::Value as JsonValue;
use uuid::Uuid;

use crate::policy::evaluator::PolicyEvaluator;
use crate::policy::models::*;
use crate::supabase::SupabaseClient;

pub struct PolicyEngine {
    client: SupabaseClient,
    evaluator: PolicyEvaluator,
}

impl PolicyEngine {
    pub fn new(client: SupabaseClient) -> Self {
        Self {
            client,
            evaluator: PolicyEvaluator::new(),
        }
    }

    pub async fn get_applicable_policies(
        &self,
        workspace_id: Uuid,
        provider: &str,
        secret_name: &str,
    ) -> Result<Vec<Policy>> {
        let db_client = self.client.get_client().await?;

        let stmt = db_client
            .prepare(
                "SELECT id, workspace_id, name, description, priority, enabled, 
                        scope, provider_pattern, secret_pattern, rules, created_at, updated_at
                 FROM policies
                 WHERE workspace_id = $1 
                 AND enabled = true
                 AND (
                     scope = 'workspace'
                     OR (scope = 'provider' AND (provider_pattern IS NULL OR provider_pattern = $2))
                     OR (scope = 'secret' AND 
                         (provider_pattern IS NULL OR provider_pattern = $2) AND
                         (secret_pattern IS NULL OR secret_pattern = $3))
                 )
                 ORDER BY priority DESC, created_at ASC",
            )
            .await?;

        let rows = db_client
            .query(&stmt, &[&workspace_id, &provider, &secret_name])
            .await?;

        let mut policies = Vec::new();
        for row in rows {
            let scope_str: String = row.get(6);
            let scope = match scope_str.as_str() {
                "workspace" => PolicyScope::Workspace,
                "provider" => PolicyScope::Provider,
                "secret" => PolicyScope::Secret,
                _ => continue,
            };

            let rules_json: JsonValue = row.get(9);
            let rules: PolicyRules = serde_json::from_value(rules_json)?;

            policies.push(Policy {
                id: row.get(0),
                workspace_id: row.get(1),
                name: row.get(2),
                description: row.get(3),
                priority: row.get(4),
                enabled: row.get(5),
                scope,
                provider_pattern: row.get(7),
                secret_pattern: row.get(8),
                rules,
                created_at: row.get(10),
                updated_at: row.get(11),
            });
        }

        Ok(policies)
    }

    pub async fn evaluate_policies(
        &self,
        context: &PolicyEvaluationContext,
    ) -> Result<PolicyEvaluationSummary> {
        let policies = self
            .get_applicable_policies(
                context.workspace_id,
                &context.provider,
                &context.secret_name,
            )
            .await?;

        let mut results = Vec::new();
        for policy in policies {
            let result = self.evaluator.evaluate(&policy, context)?;
            results.push(result);
        }

        Ok(self.evaluator.summarize_results(results))
    }

    pub async fn create_policy(
        &self,
        workspace_id: Uuid,
        name: String,
        description: Option<String>,
        priority: i32,
        scope: PolicyScope,
        provider_pattern: Option<String>,
        secret_pattern: Option<String>,
        rules: PolicyRules,
    ) -> Result<Policy> {
        let db_client = self.client.get_client().await?;

        let scope_str = match scope {
            PolicyScope::Workspace => "workspace",
            PolicyScope::Provider => "provider",
            PolicyScope::Secret => "secret",
        };

        let rules_json = serde_json::to_value(&rules)?;

        let stmt = db_client
            .prepare(
                "INSERT INTO policies 
                 (workspace_id, name, description, priority, scope, provider_pattern, secret_pattern, rules)
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                 RETURNING id, workspace_id, name, description, priority, enabled, 
                           scope, provider_pattern, secret_pattern, rules, created_at, updated_at",
            )
            .await?;

        let row = db_client
            .query_one(
                &stmt,
                &[
                    &workspace_id,
                    &name,
                    &description,
                    &priority,
                    &scope_str,
                    &provider_pattern,
                    &secret_pattern,
                    &rules_json,
                ],
            )
            .await?;

        let rules_json: JsonValue = row.get(9);
        let rules: PolicyRules = serde_json::from_value(rules_json)?;

        Ok(Policy {
            id: row.get(0),
            workspace_id: row.get(1),
            name: row.get(2),
            description: row.get(3),
            priority: row.get(4),
            enabled: row.get(5),
            scope,
            provider_pattern: row.get(7),
            secret_pattern: row.get(8),
            rules,
            created_at: row.get(10),
            updated_at: row.get(11),
        })
    }

    pub async fn get_rotation_count(&self, workspace_id: Uuid, period_days: i32) -> Result<i32> {
        let db_client = self.client.get_client().await?;

        let stmt = db_client
            .prepare(
                "SELECT COALESCE(SUM(rotation_count), 0) as total
                 FROM rotation_metering
                 WHERE workspace_id = $1
                 AND date >= CURRENT_DATE - $2",
            )
            .await?;

        let row = db_client
            .query_one(&stmt, &[&workspace_id, &period_days])
            .await?;

        let total: i64 = row.get(0);
        Ok(total as i32)
    }
}
