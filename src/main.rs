use axum::{
    Json, Router,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use serde_json::json;
use std::sync::Arc;
use supabase_rs::SupabaseClient;
// Remove unused import: use dotenv::dotenv;

pub mod routes;

#[derive(Debug)]
pub enum ApiError {
    NotFound,
    InvalidInputs(String),
    InternalError,
    Other(String),
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub supabase_url: String,
    pub supabase_key: String,
    pub supabase_service_key: String,
    pub supabase: SupabaseClient,
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
    // load the Env stuff first
    dotenv::dotenv().ok(); // This works even without the import at top

    // READ THE Environment Variables
    let supabase_url = match std::env::var("SUPABASE_URL") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Error: SUPABASE_URL must be set");
            std::process::exit(1);
        }
    };
    let supabase_key = match std::env::var("SUPABASE_KEY") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Error: SUPABASE_KEY must be set");
            std::process::exit(1);
        }
    };
    let supabase_service_key = match std::env::var("SUPABASE_SERVICE_KEY") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Error: SUPABASE_SERVICE_KEY must be set");
            std::process::exit(1);
        }
    };

    // Create Supabase client
    let supabase = match SupabaseClient::new(supabase_url.clone(), supabase_key.clone()) {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Error creating Supabase client: {:?}", e);
            std::process::exit(1);
        }
    };

    // Create the shared state
    let state = Arc::new(AppState {
        supabase_url,
        supabase_key,
        supabase_service_key,
        supabase,
    });

    // build our application with a route
    let app = Router::new()
        .route("/", get(routes::root::hello))
        .route("/users/", post(routes::add_user::create_user))
        .with_state(state);

    // bind to address
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    println!("✅ Server running!");
    println!("📡 Try these URLs:");
    println!("   http://localhost:8080/");
    println!("   http://localhost:8080/users/");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
