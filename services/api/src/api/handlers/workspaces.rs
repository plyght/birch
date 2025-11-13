use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{api::routes::AppState, workspace::models::Workspace};

fn validate_workspace_name(name: &str) -> Result<(), StatusCode> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    if trimmed.len() > 100 {
        return Err(StatusCode::BAD_REQUEST);
    }
    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct CreateWorkspaceRequest {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct ListWorkspacesQuery {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_limit")]
    pub limit: i64,
}

fn default_page() -> i64 {
    0
}

fn default_limit() -> i64 {
    20
}

#[derive(Debug, Serialize)]
pub struct WorkspaceResponse {
    pub workspace: Workspace,
}

pub async fn create_workspace(
    State(state): State<AppState>,
    Json(req): Json<CreateWorkspaceRequest>,
) -> Result<Json<WorkspaceResponse>, StatusCode> {
    validate_workspace_name(&req.name)?;

    let db_client = state
        .client
        .get_client()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let workspace_id = Uuid::new_v4();

    let stmt = db_client
        .prepare(
            "INSERT INTO workspaces (id, name, plan_tier)
             VALUES ($1, $2, 'free')
             RETURNING id, name, plan_tier, created_at, updated_at",
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let row = db_client
        .query_one(&stmt, &[&workspace_id, &req.name])
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let plan_tier = row
        .get::<_, String>(2)
        .parse()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let workspace = Workspace {
        id: row.get(0),
        name: row.get(1),
        plan_tier,
        created_at: row.get(3),
        updated_at: row.get(4),
    };

    Ok(Json(WorkspaceResponse { workspace }))
}

pub async fn list_workspaces(
    State(state): State<AppState>,
    Query(query): Query<ListWorkspacesQuery>,
) -> Result<Json<Vec<Workspace>>, StatusCode> {
    let db_client = state
        .client
        .get_client()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let limit = query.limit.clamp(1, 100);
    let offset = query.page.max(0) * limit;

    let stmt = db_client
        .prepare(
            "SELECT id, name, plan_tier, created_at, updated_at 
             FROM workspaces 
             ORDER BY created_at DESC 
             LIMIT $1 OFFSET $2",
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let rows = db_client
        .query(&stmt, &[&limit, &offset])
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let workspaces: Vec<Workspace> = rows
        .iter()
        .map(|row| {
            let plan_tier = row
                .get::<_, String>(2)
                .parse()
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(Workspace {
                id: row.get(0),
                name: row.get(1),
                plan_tier,
                created_at: row.get(3),
                updated_at: row.get(4),
            })
        })
        .collect::<Result<Vec<_>, StatusCode>>()?;

    Ok(Json(workspaces))
}

pub async fn get_workspace(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<WorkspaceResponse>, StatusCode> {
    let db_client = state
        .client
        .get_client()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let stmt = db_client
        .prepare("SELECT id, name, plan_tier, created_at, updated_at FROM workspaces WHERE id = $1")
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let row = db_client
        .query_one(&stmt, &[&id])
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let plan_tier = row
        .get::<_, String>(2)
        .parse()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let workspace = Workspace {
        id: row.get(0),
        name: row.get(1),
        plan_tier,
        created_at: row.get(3),
        updated_at: row.get(4),
    };

    Ok(Json(WorkspaceResponse { workspace }))
}

#[derive(Debug, Deserialize)]
pub struct UpdateWorkspaceRequest {
    pub name: Option<String>,
}

pub async fn update_workspace(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateWorkspaceRequest>,
) -> Result<Json<WorkspaceResponse>, StatusCode> {
    let db_client = state
        .client
        .get_client()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(name) = req.name {
        validate_workspace_name(&name)?;

        let stmt = db_client
            .prepare(
                "UPDATE workspaces SET name = $2, updated_at = NOW()
                 WHERE id = $1
                 RETURNING id, name, plan_tier, created_at, updated_at",
            )
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let row = db_client
            .query_one(&stmt, &[&id, &name])
            .await
            .map_err(|_| StatusCode::NOT_FOUND)?;

        let plan_tier = row
            .get::<_, String>(2)
            .parse()
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let workspace = Workspace {
            id: row.get(0),
            name: row.get(1),
            plan_tier,
            created_at: row.get(3),
            updated_at: row.get(4),
        };

        return Ok(Json(WorkspaceResponse { workspace }));
    }

    Err(StatusCode::BAD_REQUEST)
}

pub async fn delete_workspace(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let db_client = state
        .client
        .get_client()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let stmt = db_client
        .prepare("DELETE FROM workspaces WHERE id = $1")
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let rows_affected = db_client
        .execute(&stmt, &[&id])
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if rows_affected > 0 {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
