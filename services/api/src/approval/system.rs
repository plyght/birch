use anyhow::Result;
use chrono::{Duration, Utc};
use uuid::Uuid;

use crate::alerts::manager::AlertManager;
use crate::approval::models::*;
use crate::supabase::SupabaseClient;

pub struct ApprovalSystem {
    client: SupabaseClient,
    alert_manager: AlertManager,
}

impl ApprovalSystem {
    pub fn new(client: SupabaseClient) -> Self {
        let alert_manager = AlertManager::new(client.clone());
        Self {
            client,
            alert_manager,
        }
    }

    pub async fn create_approval_request(
        &self,
        request: CreateApprovalRequest,
    ) -> Result<ApprovalRequest> {
        let timeout_hours = request.timeout_hours.unwrap_or(24);
        let expires_at = Utc::now() + Duration::hours(timeout_hours);

        let db_client = self.client.get_client().await?;

        let action_type_str = match request.action_type {
            ActionType::Rotation => "rotation",
            ActionType::Rollback => "rollback",
            ActionType::PolicyChange => "policy_change",
        };

        let stmt = db_client
            .prepare(
                "INSERT INTO approval_requests 
                 (workspace_id, requester_id, action_type, provider, secret_name, expires_at, metadata)
                 VALUES ($1, $2, $3, $4, $5, $6, $7)
                 RETURNING id, workspace_id, requester_id, action_type, provider, secret_name,
                           status, approved_by, approved_at, rejected_by, rejected_at, rejection_reason,
                           expires_at, metadata, created_at, updated_at",
            )
            .await?;

        let row = db_client
            .query_one(
                &stmt,
                &[
                    &request.workspace_id,
                    &request.requester_id,
                    &action_type_str,
                    &request.provider,
                    &request.secret_name,
                    &expires_at,
                    &request.metadata,
                ],
            )
            .await?;

        let approval_request = self.row_to_approval_request(&row)?;

        self.alert_manager
            .send_approval_request_alert(
                request.workspace_id,
                "User",
                &request.provider,
                request.secret_name.as_deref().unwrap_or("N/A"),
                approval_request.id,
            )
            .await?;

        Ok(approval_request)
    }

    pub async fn get_approval_request(&self, request_id: Uuid) -> Result<Option<ApprovalRequest>> {
        let db_client = self.client.get_client().await?;

        let stmt = db_client
            .prepare(
                "SELECT id, workspace_id, requester_id, action_type, provider, secret_name,
                        status, approved_by, approved_at, rejected_by, rejected_at, rejection_reason,
                        expires_at, metadata, created_at, updated_at
                 FROM approval_requests
                 WHERE id = $1",
            )
            .await?;

        let rows = db_client.query(&stmt, &[&request_id]).await?;

        if rows.is_empty() {
            return Ok(None);
        }

        Ok(Some(self.row_to_approval_request(&rows[0])?))
    }

    pub async fn approve_request(
        &self,
        request_id: Uuid,
        approver_id: Uuid,
    ) -> Result<ApprovalRequest> {
        let db_client = self.client.get_client().await?;

        let stmt = db_client
            .prepare(
                "UPDATE approval_requests
                 SET status = 'approved', approved_by = $1, approved_at = NOW(), updated_at = NOW()
                 WHERE id = $2 AND status = 'pending'
                 RETURNING id, workspace_id, requester_id, action_type, provider, secret_name,
                           status, approved_by, approved_at, rejected_by, rejected_at, rejection_reason,
                           expires_at, metadata, created_at, updated_at",
            )
            .await?;

        let rows = db_client.query(&stmt, &[&approver_id, &request_id]).await?;

        if rows.is_empty() {
            anyhow::bail!("Approval request not found or already processed");
        }

        self.row_to_approval_request(&rows[0])
    }

    pub async fn reject_request(
        &self,
        request_id: Uuid,
        rejector_id: Uuid,
        reason: Option<String>,
    ) -> Result<ApprovalRequest> {
        let db_client = self.client.get_client().await?;

        let stmt = db_client
            .prepare(
                "UPDATE approval_requests
                 SET status = 'rejected', rejected_by = $1, rejected_at = NOW(), 
                     rejection_reason = $2, updated_at = NOW()
                 WHERE id = $3 AND status = 'pending'
                 RETURNING id, workspace_id, requester_id, action_type, provider, secret_name,
                           status, approved_by, approved_at, rejected_by, rejected_at, rejection_reason,
                           expires_at, metadata, created_at, updated_at",
            )
            .await?;

        let rows = db_client
            .query(&stmt, &[&rejector_id, &reason, &request_id])
            .await?;

        if rows.is_empty() {
            anyhow::bail!("Approval request not found or already processed");
        }

        self.row_to_approval_request(&rows[0])
    }

    pub async fn cancel_request(&self, request_id: Uuid) -> Result<ApprovalRequest> {
        let db_client = self.client.get_client().await?;

        let stmt = db_client
            .prepare(
                "UPDATE approval_requests
                 SET status = 'cancelled', updated_at = NOW()
                 WHERE id = $1 AND status = 'pending'
                 RETURNING id, workspace_id, requester_id, action_type, provider, secret_name,
                           status, approved_by, approved_at, rejected_by, rejected_at, rejection_reason,
                           expires_at, metadata, created_at, updated_at",
            )
            .await?;

        let rows = db_client.query(&stmt, &[&request_id]).await?;

        if rows.is_empty() {
            anyhow::bail!("Approval request not found or already processed");
        }

        self.row_to_approval_request(&rows[0])
    }

    pub async fn expire_old_requests(&self) -> Result<Vec<Uuid>> {
        let db_client = self.client.get_client().await?;

        let stmt = db_client
            .prepare(
                "UPDATE approval_requests
                 SET status = 'expired', updated_at = NOW()
                 WHERE status = 'pending' AND expires_at < NOW()
                 RETURNING id",
            )
            .await?;

        let rows = db_client.query(&stmt, &[]).await?;

        let expired_ids: Vec<Uuid> = rows.iter().map(|row| row.get(0)).collect();

        tracing::info!("Expired {} approval requests", expired_ids.len());

        Ok(expired_ids)
    }

    pub async fn list_pending_requests(&self, workspace_id: Uuid) -> Result<Vec<ApprovalRequest>> {
        let db_client = self.client.get_client().await?;

        let stmt = db_client
            .prepare(
                "SELECT id, workspace_id, requester_id, action_type, provider, secret_name,
                        status, approved_by, approved_at, rejected_by, rejected_at, rejection_reason,
                        expires_at, metadata, created_at, updated_at
                 FROM approval_requests
                 WHERE workspace_id = $1 AND status = 'pending'
                 ORDER BY created_at ASC",
            )
            .await?;

        let rows = db_client.query(&stmt, &[&workspace_id]).await?;

        let mut requests = Vec::new();
        for row in rows {
            requests.push(self.row_to_approval_request(&row)?);
        }

        Ok(requests)
    }

    fn row_to_approval_request(&self, row: &tokio_postgres::Row) -> Result<ApprovalRequest> {
        let action_type_str: String = row.get(3);
        let action_type = match action_type_str.as_str() {
            "rotation" => ActionType::Rotation,
            "rollback" => ActionType::Rollback,
            "policy_change" => ActionType::PolicyChange,
            _ => anyhow::bail!("Invalid action type: {}", action_type_str),
        };

        let status_str: String = row.get(6);
        let status = match status_str.as_str() {
            "pending" => ApprovalStatus::Pending,
            "approved" => ApprovalStatus::Approved,
            "rejected" => ApprovalStatus::Rejected,
            "expired" => ApprovalStatus::Expired,
            "cancelled" => ApprovalStatus::Cancelled,
            _ => anyhow::bail!("Invalid status: {}", status_str),
        };

        Ok(ApprovalRequest {
            id: row.get(0),
            workspace_id: row.get(1),
            requester_id: row.get(2),
            action_type,
            provider: row.get(4),
            secret_name: row.get(5),
            status,
            approved_by: row.get(7),
            approved_at: row.get(8),
            rejected_by: row.get(9),
            rejected_at: row.get(10),
            rejection_reason: row.get(11),
            expires_at: row.get(12),
            metadata: row.get(13),
            created_at: row.get(14),
            updated_at: row.get(15),
        })
    }
}
