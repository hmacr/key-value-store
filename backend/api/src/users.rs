use axum::{http::StatusCode, routing::post, Extension, Json, Router};
use common::crypt::Crypt;
use serde::Deserialize;
use store::postgres::PostgresStore;
use store::redis::Redis;

#[derive(Deserialize, Debug)]
struct UserRequest {
    name: String,
    password: String,
}

pub fn get_router() -> Router {
    Router::new()
        .route("/", post(register_user))
        .route("/login", post(login_user))
}

async fn register_user(
    Extension(mut store): Extension<PostgresStore>,
    Extension(crypt): Extension<Crypt>,
    Json(user_request): Json<UserRequest>,
) -> Result<String, StatusCode> {
    let (password_hash, password_salt) = crypt
        .generate_password_hash(&user_request.password)
        .unwrap();
    tracing::debug!("password_hash = {password_hash}\npassword_salt={password_salt}");
    let res = store
        .add_user(&user_request.name, &password_hash, &password_salt)
        .await;
    if let Some(_) = res {
        Ok("user created successfully".to_string())
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}

async fn login_user(
    Extension(store): Extension<PostgresStore>,
    Extension(redis): Extension<Redis>,
    Extension(crypt): Extension<Crypt>,
    Json(user_request): Json<UserRequest>,
) -> Result<String, StatusCode> {
    tracing::debug!("user request = {:?}", user_request);
    let db_user = store.get_user(&user_request.name).await;
    match db_user {
        Some(user) => {
            let authenticated = crypt.verify_password(
                &user_request.password,
                &user.password_hash,
                &user.password_salt,
            );
            tracing::debug!(authenticated);
            if authenticated {
                let session_id = common::generate_uuid();
                let session_set = redis.set_session_id(&user.id, &session_id).await;
                if session_set {
                    let s = format!("user authenticated with session id = {session_id}");
                    Ok(s)
                } else {
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            } else {
                Err(StatusCode::UNAUTHORIZED)
            }
        }
        None => Err(StatusCode::UNAUTHORIZED),
    }
}
