use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    tracing_subscriber::fmt::init();

    tracing::info!("Connecting to postgres database...");
    let db = common::connect_postgres().await?;
    tracing::info!("Database connected {:?}", db);

    common::migrate_postgres(&db).await?;

    api::run_server(db).await?;
    Ok(())

    // let dog_key = String::from("dog");
    // let dog_val = String::from("friendly");
    // let cat_key = String::from("cat");
    // let cat_val = String::from("self-centered");
    // let pigeon_key = String::from("pigeon");

    // tracing::info!("--- in_memory_store ---");
    // let mut in_memory_store = store::new_in_memory_store();
    // in_memory_store.put_data(&dog_key, &dog_val);
    // in_memory_store.put_data(&cat_key, &cat_val);
    // tracing::info!("dog val = {:?}", in_memory_store.get_data(&dog_key));
    // tracing::info!("cat val = {:?}", in_memory_store.get_data(&cat_key));
    // tracing::info!("pigeon val = {:?}", in_memory_store.get_data(&pigeon_key));
    // let _ = in_memory_store.remove_data(&cat_key);
    // tracing::info!(
    //     "cat val after remove = {:?}",
    //     in_memory_store.get_data(&cat_key)
    // );

    // tracing::info!("--- file_store ---");
    // let mut file_store = store::new_file_store(String::from("./file_store.csv"));
    // file_store.put_data(&dog_key, &dog_val);
    // file_store.put_data(&cat_key, &cat_val);
    // tracing::info!("dog val = {:?}", file_store.get_data(&dog_key));
    // tracing::info!("cat val = {:?}", file_store.get_data(&cat_key));
    // tracing::info!("pigeon val = {:?}", file_store.get_data(&pigeon_key));
    // let _ = file_store.remove_data(&cat_key);
    // tracing::info!("cat val after remove = {:?}", file_store.get_data(&cat_key));

    // tracing::info!("--- postgres_store ---");
    // let mut postgres_store = store::new_postgres_store(db);
    // postgres_store.put_data(&dog_key, &dog_val).await;
    // postgres_store.put_data(&cat_key, &cat_val).await;
    // tracing::info!("dog val = {:?}", postgres_store.get_data(&dog_key).await);
    // tracing::info!("cat val = {:?}", postgres_store.get_data(&cat_key).await);
    // tracing::info!(
    //     "pigeon val = {:?}",
    //     postgres_store.get_data(&pigeon_key).await
    // );
    // let _ = postgres_store.remove_data(&cat_key).await;
    // tracing::info!(
    //     "cat val after remove = {:?}",
    //     postgres_store.get_data(&cat_key).await
    // );
}
