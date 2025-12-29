use axum::{Json, response::Html, response::IntoResponse};
use serde_json::json;

use crate::ApiError;

pub async fn hello() -> impl IntoResponse {
    Html("<h1>Welcome to RustySEO API !</h1>")
}
