use axum::{Router, Json, response::IntoResponse, http::StatusCode};
use axum::routing::post;
use serde_json::json;
use std::time::Instant;
use crate::lib::auth::login::login_handler;

pub fn routes() -> Router {
    Router::new().route("/login", post(login_handler))
}
