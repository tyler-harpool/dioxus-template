use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, put},
    Json, Router,
};
use serde::Deserialize;
use shared_types::{DashboardStats, Product, User};

use crate::db::get_db;

/// Request body for creating or updating a user.
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct UserPayload {
    pub username: String,
    pub display_name: String,
}

/// Request body for creating or updating a product.
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct ProductPayload {
    pub name: String,
    pub description: String,
    pub price: f64,
    pub category: String,
    pub status: String,
}

// ── Users ──────────────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/users",
    responses(
        (status = 200, description = "List of users", body = Vec<User>)
    ),
    tag = "users"
)]
pub async fn list_users() -> impl IntoResponse {
    let db = get_db().await;
    match sqlx::query_as!(User, "SELECT id, username, display_name FROM users")
        .fetch_all(db)
        .await
    {
        Ok(users) => Json(users).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/api/users/{user_id}",
    params(("user_id" = i64, Path, description = "User ID")),
    responses(
        (status = 200, description = "User found", body = User),
        (status = 404, description = "User not found")
    ),
    tag = "users"
)]
pub async fn get_user(Path(user_id): Path<i64>) -> impl IntoResponse {
    let db = get_db().await;
    match sqlx::query_as!(
        User,
        "SELECT id, username, display_name FROM users WHERE id = $1",
        user_id
    )
    .fetch_optional(db)
    .await
    {
        Ok(Some(user)) => Json(user).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/api/users",
    request_body = UserPayload,
    responses(
        (status = 201, description = "User created", body = User)
    ),
    tag = "users"
)]
pub async fn create_user(Json(payload): Json<UserPayload>) -> impl IntoResponse {
    let db = get_db().await;
    match sqlx::query_as!(
        User,
        "INSERT INTO users (username, display_name) VALUES ($1, $2) RETURNING id, username, display_name",
        payload.username,
        payload.display_name
    )
    .fetch_one(db)
    .await
    {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[utoipa::path(
    put,
    path = "/api/users/{user_id}",
    params(("user_id" = i64, Path, description = "User ID")),
    request_body = UserPayload,
    responses(
        (status = 200, description = "User updated", body = User),
        (status = 404, description = "User not found")
    ),
    tag = "users"
)]
pub async fn update_user(
    Path(user_id): Path<i64>,
    Json(payload): Json<UserPayload>,
) -> impl IntoResponse {
    let db = get_db().await;
    match sqlx::query_as!(
        User,
        "UPDATE users SET username = $2, display_name = $3 WHERE id = $1 RETURNING id, username, display_name",
        user_id,
        payload.username,
        payload.display_name
    )
    .fetch_optional(db)
    .await
    {
        Ok(Some(user)) => Json(user).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/api/users/{user_id}",
    params(("user_id" = i64, Path, description = "User ID")),
    responses(
        (status = 204, description = "User deleted"),
        (status = 404, description = "User not found")
    ),
    tag = "users"
)]
pub async fn delete_user(Path(user_id): Path<i64>) -> impl IntoResponse {
    let db = get_db().await;
    match sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
        .execute(db)
        .await
    {
        Ok(result) if result.rows_affected() > 0 => StatusCode::NO_CONTENT.into_response(),
        Ok(_) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

// ── Products ───────────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/products",
    responses(
        (status = 200, description = "List of products", body = Vec<Product>)
    ),
    tag = "products"
)]
pub async fn list_products() -> impl IntoResponse {
    let db = get_db().await;
    match sqlx::query!(
        "SELECT id, name, description, price, category, status, created_at FROM products ORDER BY id DESC"
    )
    .fetch_all(db)
    .await
    {
        Ok(rows) => {
            let products: Vec<Product> = rows
                .into_iter()
                .map(|r| Product {
                    id: r.id,
                    name: r.name,
                    description: r.description,
                    price: r.price,
                    category: r.category,
                    status: r.status,
                    created_at: r.created_at.to_string(),
                })
                .collect();
            Json(products).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/api/products",
    request_body = ProductPayload,
    responses(
        (status = 201, description = "Product created", body = Product)
    ),
    tag = "products"
)]
pub async fn create_product(Json(payload): Json<ProductPayload>) -> impl IntoResponse {
    let db = get_db().await;
    match sqlx::query!(
        "INSERT INTO products (name, description, price, category, status) VALUES ($1, $2, $3, $4, $5) RETURNING id, name, description, price, category, status, created_at",
        payload.name,
        payload.description,
        payload.price,
        payload.category,
        payload.status
    )
    .fetch_one(db)
    .await
    {
        Ok(row) => {
            let product = Product {
                id: row.id,
                name: row.name,
                description: row.description,
                price: row.price,
                category: row.category,
                status: row.status,
                created_at: row.created_at.to_string(),
            };
            (StatusCode::CREATED, Json(product)).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[utoipa::path(
    put,
    path = "/api/products/{product_id}",
    params(("product_id" = i64, Path, description = "Product ID")),
    request_body = ProductPayload,
    responses(
        (status = 200, description = "Product updated", body = Product),
        (status = 404, description = "Product not found")
    ),
    tag = "products"
)]
pub async fn update_product(
    Path(product_id): Path<i64>,
    Json(payload): Json<ProductPayload>,
) -> impl IntoResponse {
    let db = get_db().await;
    match sqlx::query!(
        "UPDATE products SET name = $2, description = $3, price = $4, category = $5, status = $6 WHERE id = $1 RETURNING id, name, description, price, category, status, created_at",
        product_id,
        payload.name,
        payload.description,
        payload.price,
        payload.category,
        payload.status
    )
    .fetch_optional(db)
    .await
    {
        Ok(Some(row)) => {
            let product = Product {
                id: row.id,
                name: row.name,
                description: row.description,
                price: row.price,
                category: row.category,
                status: row.status,
                created_at: row.created_at.to_string(),
            };
            Json(product).into_response()
        }
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/api/products/{product_id}",
    params(("product_id" = i64, Path, description = "Product ID")),
    responses(
        (status = 204, description = "Product deleted"),
        (status = 404, description = "Product not found")
    ),
    tag = "products"
)]
pub async fn delete_product(Path(product_id): Path<i64>) -> impl IntoResponse {
    let db = get_db().await;
    match sqlx::query!("DELETE FROM products WHERE id = $1", product_id)
        .execute(db)
        .await
    {
        Ok(result) if result.rows_affected() > 0 => StatusCode::NO_CONTENT.into_response(),
        Ok(_) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

// ── Dashboard ──────────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/dashboard/stats",
    responses(
        (status = 200, description = "Dashboard statistics", body = DashboardStats)
    ),
    tag = "dashboard"
)]
pub async fn get_dashboard_stats() -> impl IntoResponse {
    let db = get_db().await;

    let total_users = sqlx::query_scalar!("SELECT COUNT(*) FROM users")
        .fetch_one(db)
        .await
        .unwrap_or(Some(0))
        .unwrap_or(0);

    let total_products = sqlx::query_scalar!("SELECT COUNT(*) FROM products")
        .fetch_one(db)
        .await
        .unwrap_or(Some(0))
        .unwrap_or(0);

    let active_products =
        sqlx::query_scalar!("SELECT COUNT(*) FROM products WHERE status = 'active'")
            .fetch_one(db)
            .await
            .unwrap_or(Some(0))
            .unwrap_or(0);

    let recent_users = sqlx::query_as!(
        User,
        "SELECT id, username, display_name FROM users ORDER BY id DESC LIMIT 5"
    )
    .fetch_all(db)
    .await
    .unwrap_or_default();

    Json(DashboardStats {
        total_users,
        total_products,
        active_products,
        recent_users,
    })
}

/// Build the REST API router with all resource routes.
pub fn rest_router() -> Router {
    Router::new()
        .route("/api/users", get(list_users).post(create_user))
        .route(
            "/api/users/{user_id}",
            get(get_user).put(update_user).delete(delete_user),
        )
        .route("/api/products", get(list_products).post(create_product))
        .route(
            "/api/products/{product_id}",
            put(update_product).delete(delete_product),
        )
        .route("/api/dashboard/stats", get(get_dashboard_stats))
}
