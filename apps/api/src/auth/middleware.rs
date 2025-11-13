use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

use crate::api::routes::AppState;
use crate::auth::jwt::JwtValidator;

#[derive(Clone)]
pub struct AuthContext {
    pub user_id: Uuid,
    pub workspace_id: Option<Uuid>,
}

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let headers = request.headers();
    let auth_header = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if !auth_header.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = &auth_header[7..];

    // Try JWT token first (JWT tokens typically start with "eyJ")
    let auth_ctx = if token.starts_with("eyJ") {
        // Validate as JWT token
        let jwt_validator = JwtValidator::new(state.jwt_secret.clone());
        let user_id = jwt_validator.validate_token(token).map_err(|e| {
            tracing::debug!("JWT validation failed: {}", e);
            StatusCode::UNAUTHORIZED
        })?;

        // JWT tokens don't have a fixed workspace, it will be determined by the request
        AuthContext {
            user_id,
            workspace_id: None,
        }
    } else {
        // Try as API key
        let api_key_record = state.client.get_api_key_by_hash(token).await.map_err(|e| {
            tracing::debug!("API key validation failed: {}", e);
            StatusCode::UNAUTHORIZED
        })?;

        AuthContext {
            user_id: api_key_record.user_id,
            workspace_id: Some(api_key_record.workspace_id),
        }
    };

    request.extensions_mut().insert(auth_ctx);

    Ok(next.run(request).await)
}
