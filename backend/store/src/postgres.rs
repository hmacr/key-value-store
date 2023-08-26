use uuid::Uuid;

#[derive(Clone)]
pub struct PostgresStore {
    db: sqlx::Pool<sqlx::Postgres>,
}

impl PostgresStore {
    pub fn new(db: sqlx::Pool<sqlx::Postgres>) -> Self {
        PostgresStore { db }
    }

    pub async fn get_all_data(&self) -> Vec<(String, String)> {
        match sqlx::query!("SELECT name, data from store")
            .fetch_all(&self.db)
            .await
        {
            Ok(rows) => {
                let mut all_data = vec![];
                for row in rows {
                    all_data.push((row.name, row.data));
                }
                all_data
            }
            Err(_) => vec![],
        }
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

    pub async fn add_user(
        &mut self,
        name: &String,
        password_hash: &String,
        password_salt: &String,
    ) -> Option<bool> {
        let id = Uuid::new_v4().to_string();
        tracing::debug!("generated uuid for {name} = {id}");
        let res = sqlx::query!(
            "INSERT INTO users (id, name, password_hash, password_salt) VALUES ($1, $2, $3, $4)",
            id,
            name,
            password_hash,
            password_salt
        )
        .execute(&self.db)
        .await;
        if let Err(e) = res {
            tracing::debug!("postgres insert failed = {e}");
            None
        } else {
            Some(true)
        }
    }

    pub async fn get_user(&self, name: &String) -> Option<DBUser> {
        tracing::debug!("db get user for {name}");
        let res = sqlx::query_as!(
            DBUser,
            "SELECT id, name, password_hash, password_salt from users WHERE name = $1",
            name
        )
        .fetch_optional(&self.db)
        .await;
        match res {
            Ok(user) => {
                tracing::debug!("db_user = {:?}", user);
                user
            }
            Err(err) => {
                tracing::debug!("get user errored = {err}");
                None
            }
        }
    }
}

#[derive(Debug)]
pub struct DBUser {
    pub id: String,
    pub name: String,
    pub password_hash: String,
    pub password_salt: String,
}
