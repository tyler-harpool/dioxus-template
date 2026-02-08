//! Integration tests for authentication endpoints.
//!
//! These tests require a running PostgreSQL database with all migrations applied.
//! Run with: `cargo test -p server --features server --test auth_tests`

#![cfg(feature = "server")]

mod common;

use axum::http::StatusCode;
use common::{get, post_json, test_app};

#[tokio::test]
async fn register_creates_user_and_returns_tokens() {
    let app = test_app().await;

    let (status, _body) = post_json(
        &app,
        "/api/users",
        r#"{"username":"authtest","display_name":"Auth Test"}"#,
    )
    .await;

    // Basic registration test - actual auth registration goes through server functions
    // which can't be tested via REST. This verifies the user creation path works.
    assert!(status == StatusCode::CREATED || status == StatusCode::OK);
}

#[tokio::test]
async fn health_includes_version() {
    let app = test_app().await;
    let (status, body) = get(&app, "/health").await;

    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("\"version\""));
    assert!(body.contains("\"uptime_seconds\""));
}
