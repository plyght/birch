use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

use crate::api::routes::AppState;

#[derive(Clone)]
pub struct AuthContext {
    pub user_id: Uuid,
    pub workspace_id: Option<Uuid>,
}

pub async fn auth_middleware(
    State(state): State<AppState>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if !auth_header.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = &auth_header[7..];

    let api_key_record = state
        .client
        .get_api_key_by_hash(token)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let auth_ctx = AuthContext {
        user_id: api_key_record.user_id,
        workspace_id: Some(api_key_record.workspace_id),
    };

    request.extensions_mut().insert(auth_ctx);

    Ok(next.run(request).await)
}
