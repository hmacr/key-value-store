use axum::Router;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing;

mod health_check;
mod in_memory;
mod postgres;

pub async fn run_server(db: Pool<Postgres>) -> anyhow::Result<()> {
    let middlewares = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive());
    let router = Router::new()
        .nest("/in_memory", in_memory::get_router())
        .nest("/postgres", postgres::get_router(db));
    let app = Router::new()
        // .layer(CorsLayer::permissive())
        // .layer(TraceLayer::new_for_http());
        .nest("/", health_check::get_router())
        .nest("/api", router)
        .layer(middlewares);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let server = axum::Server::bind(&addr).serve(app.into_make_service());
    tracing::info!("server listening on {}", addr);
    server.await?;
    Ok(())
}

#[derive(Deserialize, Serialize)]
pub struct KeyValue {
    key: String,
    value: String,
}
