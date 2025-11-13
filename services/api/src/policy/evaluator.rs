use anyhow::Result;
use chrono::{Datelike, NaiveTime, Utc, Weekday};
use chrono_tz::Tz;

use crate::policy::models::*;

pub struct PolicyEvaluator;

impl PolicyEvaluator {
    pub fn new() -> Self {
        Self
    }

    pub fn evaluate(
        &self,
        policy: &Policy,
        context: &PolicyEvaluationContext,
    ) -> Result<PolicyEvaluationResult> {
        if !policy.enabled {
            return Ok(PolicyEvaluationResult {
                policy_id: policy.id,
                policy_name: policy.name.clone(),
                passed: true,
                reason: Some("Policy disabled".to_string()),
                action: PolicyAction::Allow,
            });
        }

        if !self.matches_scope(policy, context) {
            return Ok(PolicyEvaluationResult {
                policy_id: policy.id,
                policy_name: policy.name.clone(),
                passed: true,
                reason: Some("Policy scope does not match".to_string()),
                action: PolicyAction::Allow,
            });
        }

        let mut reasons = Vec::new();
        let mut action = PolicyAction::Allow;

        if let Some(limits) = &policy.rules.rotation_limits {
            if let Some(hard_limit) = limits.hard_limit {
                if context.current_rotation_count >= hard_limit {
                    action = PolicyAction::Block;
                    reasons.push(format!(
                        "Hard limit reached: {} rotations in {}",
                        context.current_rotation_count, limits.period
                    ));
                }
            }

            if let Some(soft_limit) = limits.soft_limit {
                if context.current_rotation_count >= soft_limit && action == PolicyAction::Allow {
                    action = PolicyAction::Warn;
                    reasons.push(format!(
                        "Soft limit reached: {} rotations in {}",
                        context.current_rotation_count, limits.period
                    ));
                }
            }
        }

        if let Some(windows) = &policy.rules.maintenance_windows {
            if !self.is_in_maintenance_window(windows)? {
                if action == PolicyAction::Allow {
                    action = PolicyAction::Block;
                }
                reasons.push("Outside of maintenance window".to_string());
            }
        }

        if let Some(allowed_envs) = &policy.rules.allowed_environments {
            if let Some(env) = &context.environment {
                if !allowed_envs.contains(env) {
                    if action == PolicyAction::Allow {
                        action = PolicyAction::Block;
                    }
                    reasons.push(format!("Environment '{}' not allowed", env));
                }
            }
        }

        if policy.rules.require_approval == Some(true) && action == PolicyAction::Allow {
            action = PolicyAction::RequireApproval;
            reasons.push("Approval required by policy".to_string());
        }

        let passed = matches!(action, PolicyAction::Allow | PolicyAction::Warn);

        Ok(PolicyEvaluationResult {
            policy_id: policy.id,
            policy_name: policy.name.clone(),
            passed,
            reason: if reasons.is_empty() {
                None
            } else {
                Some(reasons.join("; "))
            },
            action,
        })
    }

    fn matches_scope(&self, policy: &Policy, context: &PolicyEvaluationContext) -> bool {
        match policy.scope {
            PolicyScope::Workspace => true,
            PolicyScope::Provider => {
                if let Some(pattern) = &policy.provider_pattern {
                    self.matches_pattern(pattern, &context.provider)
                } else {
                    true
                }
            }
            PolicyScope::Secret => {
                let provider_matches = if let Some(pattern) = &policy.provider_pattern {
                    self.matches_pattern(pattern, &context.provider)
                } else {
                    true
                };

                let secret_matches = if let Some(pattern) = &policy.secret_pattern {
                    self.matches_pattern(pattern, &context.secret_name)
                } else {
                    true
                };

                provider_matches && secret_matches
            }
        }
    }

    fn matches_pattern(&self, pattern: &str, value: &str) -> bool {
        if pattern.contains('*') {
            let regex_pattern = pattern.replace('*', ".*");
            regex::Regex::new(&format!("^{}$", regex_pattern))
                .map(|r| r.is_match(value))
                .unwrap_or(false)
        } else {
            pattern == value
        }
    }

    fn is_in_maintenance_window(&self, windows: &[MaintenanceWindow]) -> Result<bool> {
        let now = Utc::now();

        for window in windows {
            let tz: Tz = window.timezone.parse()?;
            let now_in_tz = now.with_timezone(&tz);

            if let Some(day_filter) = &window.day_of_week {
                let current_weekday = now_in_tz.weekday();
                if !self.matches_weekday(day_filter, current_weekday) {
                    continue;
                }
            }

            let start_time: NaiveTime = window.start_time.parse()?;
            let end_time: NaiveTime = window.end_time.parse()?;
            let current_time = now_in_tz.time();

            if start_time <= end_time {
                if current_time >= start_time && current_time <= end_time {
                    return Ok(true);
                }
            } else if current_time >= start_time || current_time <= end_time {
                return Ok(true);
            }
        }

        Ok(false)
    }

    fn matches_weekday(&self, day_filter: &str, weekday: Weekday) -> bool {
        let day_lower = day_filter.to_lowercase();
        match weekday {
            Weekday::Mon => day_lower.contains("mon"),
            Weekday::Tue => day_lower.contains("tue"),
            Weekday::Wed => day_lower.contains("wed"),
            Weekday::Thu => day_lower.contains("thu"),
            Weekday::Fri => day_lower.contains("fri"),
            Weekday::Sat => day_lower.contains("sat"),
            Weekday::Sun => day_lower.contains("sun"),
        }
    }

    pub fn summarize_results(
        &self,
        results: Vec<PolicyEvaluationResult>,
    ) -> PolicyEvaluationSummary {
        let mut allowed = true;
        let mut requires_approval = false;
        let mut warnings = Vec::new();
        let mut blocking_reasons = Vec::new();

        for result in &results {
            match result.action {
                PolicyAction::Allow => {}
                PolicyAction::Warn => {
                    if let Some(reason) = &result.reason {
                        warnings.push(format!("{}: {}", result.policy_name, reason));
                    }
                }
                PolicyAction::Block => {
                    allowed = false;
                    if let Some(reason) = &result.reason {
                        blocking_reasons.push(format!("{}: {}", result.policy_name, reason));
                    }
                }
                PolicyAction::RequireApproval => {
                    requires_approval = true;
                }
            }
        }

        PolicyEvaluationSummary {
            allowed,
            requires_approval,
            warnings,
            blocking_reasons,
            results,
        }
    }
}

impl Default for PolicyEvaluator {
    fn default() -> Self {
        Self::new()
    }
}
