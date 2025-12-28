use axum::{
    Json, // Add this import
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use uuid::Uuid;

use chrono::{DateTime, Utc};

use crate::{ApiError, AppState}; // Import ApiError from crate

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub user: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
enum Message {
    UserAdded(String),
    UserFailed(String),
}

// Helper function to convert SystemTime to timestamp - MAKE IT PUBLIC

pub fn system_time_to_timestamp(time: SystemTime) -> String {
    let datetime: DateTime<Utc> = time.into();
    datetime.to_rfc3339()
}

pub async fn user_result(
    Path(user_id): Path<u32>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, ApiError> {
    if user_id == 0 {
        return Err(ApiError::InvalidInputs(
            "User ID cannot be zero".to_string(),
        ));
    }

    // Create userd
    let user = User {
        user: user_id.to_string(),
        created_at: Utc::now().to_rfc3339(),
    };

    // Insert into Supabase
    match insert_user_to_supabase(&state.supabase, &user).await {
        Ok(success) => Ok(Json(json!({
            "success": success,
            "user_id": user_id,
            "message": "User Added Successfully",
        }))),
        Err(e) => Err(ApiError::Other(format!("Failed to insert user: {}", e))),
    }
}

async fn insert_user_to_supabase(
    supabase: &supabase_rs::SupabaseClient,
    user: &User,
) -> Result<bool, Box<dyn std::error::Error>> {
    let payload = serde_json::json!(user);

    println!("Payload: {:#?}", &payload);

    match supabase.insert("users", payload).await {
        Ok(body) => {
            println!("Supabase insert success: {}", body);
            Ok(true)
        }
        Err(err) => {
            // 👇 THIS is what we need to see
            println!("Supabase insert failed (raw): {:#?}", err);
            dbg!(&err);
            Err(err.into())
        }
    }
}
