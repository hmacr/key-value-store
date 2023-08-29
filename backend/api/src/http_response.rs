use axum::{
    body::{self, BoxBody},
    http::{header, StatusCode},
    response::Response,
};

const HEADER_TEXT_PLAIN: &str = "text/plain";
const HEADER_APPLICATION_JSON: &str = "application/json";

pub fn bad_request() -> Response<BoxBody> {
    Response::builder()
        .status(StatusCode::BAD_REQUEST.as_str())
        .body(body::boxed(body::Empty::new()))
        .unwrap()
}

pub fn unauth() -> Response<BoxBody> {
    Response::builder()
        .status(StatusCode::UNAUTHORIZED.as_str())
        .body(body::boxed(body::Empty::new()))
        .unwrap()
}

pub fn not_found() -> Response<BoxBody> {
    Response::builder()
        .status(StatusCode::NOT_FOUND.as_str())
        .body(body::boxed(body::Empty::new()))
        .unwrap()
}

pub fn internal_server_error() -> Response<BoxBody> {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR.as_str())
        .body(body::boxed(body::Empty::new()))
        .unwrap()
}

pub fn string_body(body: String, header_map: Option<header::HeaderMap>) -> Response<BoxBody> {
    let mut resp_builder = Response::builder()
        .status(StatusCode::OK.as_str())
        .header(header::CONTENT_TYPE, HEADER_TEXT_PLAIN);
    if let Some(header_map) = header_map {
        for (key, value) in header_map {
            resp_builder = resp_builder.header(key.unwrap(), value);
        }
    }
    resp_builder.body(body::boxed(body)).unwrap()
}

pub fn json_body<T: serde::Serialize>(json_struct: &T) -> Response<BoxBody> {
    let json_string = serde_json::to_string(json_struct).unwrap();
    Response::builder()
        .status(StatusCode::OK.as_str())
        .header(header::CONTENT_TYPE, HEADER_APPLICATION_JSON)
        .body(body::boxed(json_string))
        .unwrap()
}
