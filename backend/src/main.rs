use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    tracing_subscriber::fmt::init();

    tracing::info!("Connecting to redis...");
    let redis = common::connect_redis()?;
    tracing::info!("redis connected {:?}", redis);

    tracing::info!("Connecting to postgres database...");
    let db = common::connect_postgres().await?;
    tracing::info!("Database connected {:?}", db);

    tracing::info!("running db migrations...");
    common::migrate_postgres(&db).await?;
    tracing::info!("db migration completed");

    api::run_server(db, redis).await?;
    Ok(())
}
