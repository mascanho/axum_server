use axum::{Json, response::Html, response::IntoResponse};
use serde_json::json;

use crate::ApiError;

enum OrderStatus {
    Pending,
    Shipped,
    Delivered,
}

impl std::fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderStatus::Pending => write!(f, "Pending"),
            OrderStatus::Shipped => write!(f, "Shipped"),
            OrderStatus::Delivered => write!(f, "Delivered"),
        }
    }
}

struct Receipt {
    sum: i32,
    items: Vec<String>,
    status: String,
}

impl Receipt {
    fn new(sum: i32, items: Vec<String>) -> Self {
        Self {
            sum,
            items,
            status: OrderStatus::Pending.to_string(),
        }
    }
}

// Make some request and return the response of a page
async fn get_page_title() -> Result<String, reqwest::Error> {
    let page_title = reqwest::get("https://www.rust-lang.org/")
        .await?
        .text()
        .await?;
    Ok(page_title)
}

pub async fn hello() -> impl IntoResponse {
    Html("<h1>Welcome to RustySEO API !</h1>")
}
