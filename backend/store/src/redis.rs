use redis::AsyncCommands;

#[derive(Clone)]
pub struct Redis {
    client: redis::Client,
}

impl Redis {
    pub fn new(client: redis::Client) -> Self {
        Redis { client }
    }

    pub async fn set_session_id(self, user_id: &String, session_id: &String) -> bool {
        let mut conn = self.get_conn().await;
        match conn.set(user_id.clone(), session_id.clone()).await {
            Ok(()) => true,
            Err(_) => false,
        }
    }

    pub async fn get_session_id(self, user_id: &String) -> Option<String> {
        let mut conn = self.get_conn().await;
        match conn.get(user_id).await {
            Ok(session_id) => session_id,
            Err(_) => None,
        }
    }

    async fn get_conn(self) -> redis::aio::Connection {
        self.client.get_async_connection().await.unwrap()
    }
}
