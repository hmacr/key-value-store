use axum::{Extension, Router};
use common::crypt;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use std::net::SocketAddr;
use store::postgres::PostgresStore;
use store::redis::Redis;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing;

mod health_check;
mod in_memory;
mod postgres;
mod users;

pub async fn run_server(db: Pool<Postgres>, redis_client: redis::Client) -> anyhow::Result<()> {
    let store = PostgresStore::new(db);
    let redis = Redis::new(redis_client);
    let crypt = crypt::Crypt::new();
    let middlewares = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .layer(Extension(store))
        .layer(Extension(redis))
        .layer(Extension(crypt));
    let router = Router::new()
        .nest("/in_memory", in_memory::get_router())
        .nest("/postgres", postgres::get_router())
        .nest("/users", users::get_router());
    let app = Router::new()
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
