use anyhow::anyhow;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub mod crypt;

type DB = Pool<Postgres>;

pub async fn connect_postgres() -> anyhow::Result<DB> {
    let db_url = std::env::var("DATABASE_URL")?;
    sqlx::postgres::PgPoolOptions::new()
        .connect(&db_url)
        .await
        .map_err(|err| anyhow!(err.to_string()))
}

pub fn connect_redis() -> anyhow::Result<redis::Client> {
    let redis_url = std::env::var("REDIS_URL")?;
    redis::Client::open(redis_url).map_err(|err| anyhow!(err.to_string()))
}

pub async fn migrate_postgres(db: &DB) -> anyhow::Result<()> {
    match sqlx::migrate!("../migrations").run(db).await {
        Ok(_) => Ok(()),
        Err(_) => Ok(()),
    }
}

pub fn generate_uuid() -> String {
    Uuid::new_v4().to_string()
}
