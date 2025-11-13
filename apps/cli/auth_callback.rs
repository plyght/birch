use anyhow::{Context, Result};
use axum::{
    extract::Query,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Deserialize)]
pub struct CallbackParams {
    pub token: Option<String>,
    pub state: Option<String>,
    pub error: Option<String>,
    pub error_description: Option<String>,
}

#[derive(Clone)]
struct CallbackState {
    expected_state: String,
    result: Arc<Mutex<Option<CallbackResult>>>,
}

#[derive(Clone)]
struct CallbackResult {
    token: Option<String>,
    error: Option<String>,
}

async fn handle_callback(
    Query(params): Query<CallbackParams>,
    state: axum::extract::State<CallbackState>,
) -> impl IntoResponse {
    // Validate state token
    if let Some(received_state) = &params.state {
        if received_state != &state.expected_state {
            return (
                StatusCode::BAD_REQUEST,
                Html(error_page("Invalid state token")),
            );
        }
    } else {
        return (
            StatusCode::BAD_REQUEST,
            Html(error_page("Missing state token")),
        );
    }

    // Check for errors
    if let Some(error) = &params.error {
        let error_msg = params.error_description.as_deref().unwrap_or(error);

        let mut result = state.result.lock().await;
        *result = Some(CallbackResult {
            token: None,
            error: Some(error_msg.to_string()),
        });

        return (StatusCode::OK, Html(error_page(error_msg)));
    }

    // Store the token
    if let Some(token) = params.token {
        let mut result = state.result.lock().await;
        *result = Some(CallbackResult {
            token: Some(token),
            error: None,
        });

        (StatusCode::OK, Html(success_page()))
    } else {
        (StatusCode::BAD_REQUEST, Html(error_page("Missing token")))
    }
}

fn success_page() -> String {
    r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Birch CLI - Authentication Successful</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
            display: flex;
            justify-content: center;
            align-items: center;
            min-height: 100vh;
            margin: 0;
            background: #faf9f6;
            color: #1a1a1a;
        }
        .container {
            text-align: center;
            padding: 3rem;
            max-width: 500px;
        }
        .success {
            font-size: 4rem;
            margin-bottom: 1rem;
        }
        h1 {
            font-size: 1.75rem;
            margin-bottom: 0.5rem;
            font-weight: 600;
        }
        p {
            color: #666;
            line-height: 1.6;
        }
        .close-message {
            margin-top: 2rem;
            padding: 1rem;
            background: #f5f5f5;
            border-radius: 8px;
            font-size: 0.875rem;
        }
    </style>
</head>
<body>
    <div class="container">
        <div class="success">✓</div>
        <h1>Authentication Successful</h1>
        <p>You have successfully authenticated with Birch.</p>
        <div class="close-message">
            You can close this window and return to your terminal.
        </div>
    </div>
</body>
</html>"#
        .to_string()
}

fn error_page(error: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Birch CLI - Authentication Failed</title>
    <style>
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
            display: flex;
            justify-content: center;
            align-items: center;
            min-height: 100vh;
            margin: 0;
            background: #faf9f6;
            color: #1a1a1a;
        }}
        .container {{
            text-align: center;
            padding: 3rem;
            max-width: 500px;
        }}
        .error {{
            font-size: 4rem;
            margin-bottom: 1rem;
            color: #dc2626;
        }}
        h1 {{
            font-size: 1.75rem;
            margin-bottom: 0.5rem;
            font-weight: 600;
        }}
        p {{
            color: #666;
            line-height: 1.6;
        }}
        .error-detail {{
            margin-top: 2rem;
            padding: 1rem;
            background: #fee;
            border-radius: 8px;
            font-size: 0.875rem;
            color: #dc2626;
        }}
    </style>
</head>
<body>
    <div class="container">
        <div class="error">✗</div>
        <h1>Authentication Failed</h1>
        <p>There was a problem authenticating with Birch.</p>
        <div class="error-detail">{}</div>
        <p style="margin-top: 2rem;">You can close this window and try again.</p>
    </div>
</body>
</html>"#,
        error
    )
}

pub async fn start_callback_server(expected_state: String, port: u16) -> Result<Option<String>> {
    let result = Arc::new(Mutex::new(None));

    let callback_state = CallbackState {
        expected_state,
        result: Arc::clone(&result),
    };

    let app = Router::new()
        .route("/auth/callback", get(handle_callback))
        .with_state(callback_state);

    let addr = format!("127.0.0.1:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .context(format!("Failed to bind to {}", addr))?;

    println!("  Waiting for authentication callback on {}...", addr);
    println!();

    // Run the server and wait for a callback
    let server_handle = tokio::spawn(async move {
        axum::serve(listener, app).await.ok();
    });

    // Poll the result for up to 5 minutes
    let timeout = tokio::time::Duration::from_secs(300);
    let start = tokio::time::Instant::now();

    loop {
        {
            let result_guard = result.lock().await;
            if let Some(callback_result) = result_guard.as_ref() {
                if let Some(error) = &callback_result.error {
                    anyhow::bail!("Authentication failed: {}", error);
                }
                if let Some(token) = &callback_result.token {
                    // Give the response time to be sent to the browser
                    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                    server_handle.abort();
                    return Ok(Some(token.clone()));
                }
            }
        }

        if start.elapsed() > timeout {
            server_handle.abort();
            anyhow::bail!("Authentication timeout - no callback received within 5 minutes");
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}
