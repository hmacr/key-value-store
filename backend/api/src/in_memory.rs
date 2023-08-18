use crate::KeyValue;
use axum::{
    extract::Path,
    http::StatusCode,
    routing::{delete, get, post},
    Extension, Json, Router,
};
use store::in_memory::InMemoryStore;
use tower::ServiceBuilder;

pub fn get_router() -> Router {
    let store = InMemoryStore::new();
    let layer = ServiceBuilder::new().layer(Extension(store));
    Router::new()
        .route("/", get(get_all_data))
        .route("/", post(add_data))
        .route("/:key", get(get_data))
        .route("/:key", delete(delete_data))
        .layer(layer)
}

async fn get_all_data(
    Extension(store): Extension<InMemoryStore>,
) -> Result<Json<Vec<KeyValue>>, StatusCode> {
    let mut all_data = vec![];
    for (key, value) in store.get_all_data() {
        all_data.push(KeyValue { key, value });
    }
    Ok(Json(all_data))
}

async fn add_data(
    Extension(mut store): Extension<InMemoryStore>,
    Json(key_value): Json<KeyValue>,
) -> Result<String, StatusCode> {
    store.put_data(&key_value.key, &key_value.value);
    Ok(String::from("successfully inserted data"))
}

async fn get_data(
    Extension(store): Extension<InMemoryStore>,
    Path(key): Path<String>,
) -> Result<Json<KeyValue>, StatusCode> {
    match store.get_data(&key) {
        Some(value) => {
            let key_value = KeyValue { key, value };
            Ok(Json(key_value))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn delete_data(
    Extension(mut store): Extension<InMemoryStore>,
    Path(key): Path<String>,
) -> Result<String, StatusCode> {
    store.remove_data(&key);
    Ok(String::from("successfully deleted data for the input key"))
}
