use axum::{body::BoxBody, response::Response as AxumResponse, routing::get, Router};

use crate::http_response;

pub fn get_router() -> Router {
    Router::new().route("/health_check", get(health_check))
}

async fn health_check() -> AxumResponse<BoxBody> {
    http_response::string_body("app is healthy".to_string(), None)
}
