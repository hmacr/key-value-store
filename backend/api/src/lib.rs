use axum::Router;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use std::net::SocketAddr;
use tracing;

mod health_check;
mod in_memory;
mod postgres;

pub async fn run_server(db: Pool<Postgres>) -> anyhow::Result<()> {
    let router = Router::new()
        .nest("/in_memory", in_memory::get_router())
        .nest("/postgres", postgres::get_router(db));
    let app = Router::new()
        .nest("/", health_check::get_router())
        .nest("/api", router);

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
