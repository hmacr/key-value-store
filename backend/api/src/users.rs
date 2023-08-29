use crate::http_response;
use axum::{
    body::BoxBody, http::header, response::Response as AxumResponse, routing::post, Extension,
    Json, Router,
};
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
) -> AxumResponse<BoxBody> {
    let (password_hash, password_salt) = crypt
        .generate_password_hash(&user_request.password)
        .unwrap();
    tracing::debug!("password_hash = {password_hash}\npassword_salt={password_salt}");
    match store
        .add_user(&user_request.name, &password_hash, &password_salt)
        .await
    {
        Some(_) => http_response::string_body("user created successfully".to_string(), None),
        None => http_response::bad_request(),
    }
}

async fn login_user(
    Extension(store): Extension<PostgresStore>,
    Extension(redis): Extension<Redis>,
    Extension(crypt): Extension<Crypt>,
    Json(user_request): Json<UserRequest>,
) -> AxumResponse<BoxBody> {
    tracing::debug!("user request = {:?}", user_request);
    match store.get_user(&user_request.name).await {
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
                    let session_cookie = format!("user_id={}", user.id);
                    let mut header_map = header::HeaderMap::new();
                    header_map.insert(
                        header::SET_COOKIE,
                        header::HeaderValue::from_str(&session_cookie).unwrap(),
                    );
                    http_response::string_body("user authenticated".to_string(), Some(header_map))
                } else {
                    http_response::internal_server_error()
                }
            } else {
                http_response::unauth()
            }
        }
        None => http_response::unauth(),
    }
}
