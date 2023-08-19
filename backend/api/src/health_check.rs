use axum::{http::StatusCode, routing::get, Router};

pub fn get_router() -> Router {
    Router::new().route("/health_check", get(health_check))
}

async fn health_check() -> Result<String, StatusCode> {
    Ok("app is healthy".to_string())
}
