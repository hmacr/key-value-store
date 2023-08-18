use uuid::Uuid;

pub struct PostgresStore {
    db: sqlx::Pool<sqlx::Postgres>,
}

impl PostgresStore {
    pub fn new(db: sqlx::Pool<sqlx::Postgres>) -> Self {
        PostgresStore { db }
    }

    pub async fn get_data(&self, key: &String) -> Option<String> {
        match sqlx::query!("SELECT name, data from store where name = $1", key)
            .fetch_one(&self.db)
            .await
        {
            Ok(row) => Some(row.data),
            Err(_) => None,
        }
    }

    pub async fn put_data(&mut self, key: &String, value: &String) {
        let id = Uuid::new_v4().to_string();
        tracing::debug!("generated uuid for {key} = {id}");
        let res = sqlx::query!(
            "INSERT INTO store (id, name, data) VALUES ($1, $2, $3)",
            id,
            key,
            value
        )
        .execute(&self.db)
        .await;
        if let Err(e) = res {
            tracing::info!("postgres insert failed = {e}");
        }
    }

    pub async fn remove_data(&mut self, key: &String) -> Option<String> {
        match sqlx::query_scalar!("DELETE FROM store where name = $1 RETURNING data", key)
            .fetch_one(&self.db)
            .await
        {
            Ok(row) => Some(row),
            Err(_) => None,
        }
    }
}
