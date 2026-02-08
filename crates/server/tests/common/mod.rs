use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware,
    Router,
};
use server::db::AppState;
use tower::ServiceExt;

#[allow(dead_code)]
/// Build a test router with the REST API routes (no auth middleware).
/// Each call creates a fresh pool bound to the current runtime.
pub async fn test_app() -> Router {
    let pool = server::db::create_pool();
    server::db::run_migrations(&pool).await;
    let state = AppState { pool };

    server::rest::rest_router()
        .route("/health", axum::routing::get(server::health::health_check))
        .with_state(state)
}

#[allow(dead_code)]
/// Build a test router with auth middleware enabled.
/// Required for endpoints that use AuthRequired/TierRequired extractors.
pub async fn test_app_with_auth() -> Router {
    let pool = server::db::create_pool();
    server::db::run_migrations(&pool).await;
    let state = AppState { pool };

    server::rest::rest_router()
        .route("/health", axum::routing::get(server::health::health_check))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            server::auth::middleware::auth_middleware,
        ))
        .with_state(state)
}

#[allow(dead_code)]
/// Register a test user via the REST API and return (status, auth_response_body).
pub async fn register_test_user(
    app: &Router,
    username: &str,
    email: &str,
    password: &str,
) -> (StatusCode, String) {
    let json = serde_json::json!({
        "username": username,
        "email": email,
        "password": password,
        "display_name": format!("Test {}", username)
    });
    post_json(app, "/api/auth/register", &json.to_string()).await
}

#[allow(dead_code)]
/// Helper to make a GET request and return (status, body).
pub async fn get(app: &Router, uri: &str) -> (StatusCode, String) {
    let response = app
        .clone()
        .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
        .await
        .unwrap();

    let status = response.status();
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    (status, String::from_utf8(body.to_vec()).unwrap())
}

#[allow(dead_code)]
/// Helper to make a POST request with JSON body.
pub async fn post_json(app: &Router, uri: &str, json: &str) -> (StatusCode, String) {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(uri)
                .header("content-type", "application/json")
                .body(Body::from(json.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    let status = response.status();
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    (status, String::from_utf8(body.to_vec()).unwrap())
}

#[allow(dead_code)]
/// Helper to make a PUT request with JSON body.
pub async fn put_json(app: &Router, uri: &str, json: &str) -> (StatusCode, String) {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(uri)
                .header("content-type", "application/json")
                .body(Body::from(json.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    let status = response.status();
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    (status, String::from_utf8(body.to_vec()).unwrap())
}

#[allow(dead_code)]
/// Helper to make a PUT request with JSON body and Bearer auth.
pub async fn put_json_with_auth(
    app: &Router,
    uri: &str,
    json: &str,
    token: &str,
) -> (StatusCode, String) {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("PUT")
                .uri(uri)
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", token))
                .body(Body::from(json.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    let status = response.status();
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    (status, String::from_utf8(body.to_vec()).unwrap())
}

#[allow(dead_code)]
/// Helper to make a POST request with JSON body and Bearer auth.
pub async fn post_json_with_auth(
    app: &Router,
    uri: &str,
    json: &str,
    token: &str,
) -> (StatusCode, String) {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(uri)
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", token))
                .body(Body::from(json.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    let status = response.status();
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    (status, String::from_utf8(body.to_vec()).unwrap())
}

#[allow(dead_code)]
/// Helper to make a DELETE request.
pub async fn delete(app: &Router, uri: &str) -> (StatusCode, String) {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(uri)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let status = response.status();
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    (status, String::from_utf8(body.to_vec()).unwrap())
}
