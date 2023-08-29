use crate::{http_response, KeyValue};
use axum::{
    body::BoxBody,
    extract::Path,
    response::Response as AxumResponse,
    routing::{delete, get, post},
    Extension, Json, Router,
};
use store::postgres::PostgresStore;

pub fn get_router() -> Router {
    Router::new()
        .route("/", get(get_all_data))
        .route("/", post(add_data))
        .route("/:key", get(get_data))
        .route("/:key", delete(delete_data))
}

async fn get_all_data(Extension(postgres): Extension<PostgresStore>) -> AxumResponse<BoxBody> {
    let mut all_data = vec![];
    for (key, value) in postgres.get_all_data().await {
        all_data.push(KeyValue { key, value });
    }
    http_response::json_body(&all_data)
}

async fn add_data(
    Extension(mut postgres): Extension<PostgresStore>,
    Json(key_value): Json<KeyValue>,
) -> AxumResponse<BoxBody> {
    postgres.put_data(&key_value.key, &key_value.value).await;
    http_response::string_body("successfully inserted data".to_string(), None)
}

async fn get_data(
    Extension(postgres): Extension<PostgresStore>,
    Path(key): Path<String>,
) -> AxumResponse<BoxBody> {
    match postgres.get_data(&key).await {
        Some(value) => {
            let key_value = KeyValue { key, value };
            http_response::json_body(&key_value)
        }
        None => http_response::not_found(),
    }
}

async fn delete_data(
    Extension(mut postgres): Extension<PostgresStore>,
    Path(key): Path<String>,
) -> AxumResponse<BoxBody> {
    postgres.remove_data(&key).await;
    http_response::string_body(
        "successfully deleted data for the input key".to_string(),
        None,
    )
}
