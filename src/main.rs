use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::get};
use serde_json::json;

pub mod routes;

#[derive(Debug)]
pub enum ApiError {
    NotFound,
    InvalidInputs(String),
    InternalError,
    Other(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message): (StatusCode, String) = match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Not Found".to_string()),
            ApiError::InvalidInputs(error) => (StatusCode::BAD_REQUEST, error),
            ApiError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Error".to_string(),
            ),
            ApiError::Other(error) => (StatusCode::INTERNAL_SERVER_ERROR, error),
        };

        let body = Json(json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
}

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/", get(routes::root::hello));

    // bind to address
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3002").await.unwrap();

    println!("Listening on http://0.0.0.0:3002");

    // start server (axum 0.7+)
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
