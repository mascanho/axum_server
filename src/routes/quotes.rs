use axum::Json;
use axum::response::{Html, IntoResponse};
use rand::prelude::IndexedRandom;
use rand::thread_rng;
use serde_json::json;

use crate::routes::utils;

struct Quote {
    text: String,
    date: String,
}

pub async fn get_random_quote() -> impl IntoResponse {
    let quotes = utils::data::QUOTES;
    let mut rng = thread_rng();

    let random_quote = quotes.choose(&mut rng).unwrap_or(&"No quotes available");

    let quote = Quote {
        text: random_quote.to_string(),
        date: chrono::Local::now().format("%Y-%m-%d").to_string(),
    };

    Json(json!({
        "text": quote.text,
        "date": quote.date
    }))
}
