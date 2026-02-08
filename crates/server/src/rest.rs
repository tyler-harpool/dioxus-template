use axum::{
    extract::Path,
    http::StatusCode,
    routing::{get, put},
    Json, Router,
};
use shared_types::{
    AppError, CreateProductRequest, CreateUserRequest, DashboardStats, Product,
    UpdateProductRequest, UpdateUserRequest, User,
};

use crate::db::get_db;
use crate::error_convert::{SqlxErrorExt, ValidateRequest};

// ── Users ──────────────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/users",
    responses(
        (status = 200, description = "List of users", body = Vec<User>),
        (status = 500, description = "Internal server error", body = AppError)
    ),
    tag = "users"
)]
#[tracing::instrument]
pub async fn list_users() -> Result<Json<Vec<User>>, AppError> {
    let db = get_db().await;
    let users = sqlx::query_as!(
        User,
        "SELECT id, username, display_name, role, tier FROM users"
    )
    .fetch_all(db)
    .await
    .map_err(SqlxErrorExt::into_app_error)?;
    Ok(Json(users))
}

#[utoipa::path(
    get,
    path = "/api/users/{user_id}",
    params(("user_id" = i64, Path, description = "User ID")),
    responses(
        (status = 200, description = "User found", body = User),
        (status = 404, description = "User not found", body = AppError),
        (status = 500, description = "Internal server error", body = AppError)
    ),
    tag = "users"
)]
#[tracing::instrument]
pub async fn get_user(Path(user_id): Path<i64>) -> Result<Json<User>, AppError> {
    let db = get_db().await;
    let user = sqlx::query_as!(
        User,
        "SELECT id, username, display_name, role, tier FROM users WHERE id = $1",
        user_id
    )
    .fetch_optional(db)
    .await
    .map_err(SqlxErrorExt::into_app_error)?
    .ok_or_else(|| AppError::not_found(format!("User with id {} not found", user_id)))?;
    Ok(Json(user))
}

#[utoipa::path(
    post,
    path = "/api/users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created", body = User),
        (status = 422, description = "Validation error", body = AppError),
        (status = 500, description = "Internal server error", body = AppError)
    ),
    tag = "users"
)]
#[tracing::instrument]
pub async fn create_user(
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<User>), AppError> {
    payload.validate_request()?;

    let db = get_db().await;
    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (username, display_name) VALUES ($1, $2) RETURNING id, username, display_name, role, tier",
        payload.username,
        payload.display_name
    )
    .fetch_one(db)
    .await
    .map_err(SqlxErrorExt::into_app_error)?;
    Ok((StatusCode::CREATED, Json(user)))
}

#[utoipa::path(
    put,
    path = "/api/users/{user_id}",
    params(("user_id" = i64, Path, description = "User ID")),
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "User updated", body = User),
        (status = 404, description = "User not found", body = AppError),
        (status = 422, description = "Validation error", body = AppError),
        (status = 500, description = "Internal server error", body = AppError)
    ),
    tag = "users"
)]
#[tracing::instrument]
pub async fn update_user(
    Path(user_id): Path<i64>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<User>, AppError> {
    payload.validate_request()?;

    let db = get_db().await;
    let user = sqlx::query_as!(
        User,
        "UPDATE users SET username = $2, display_name = $3 WHERE id = $1 RETURNING id, username, display_name, role, tier",
        user_id,
        payload.username,
        payload.display_name
    )
    .fetch_optional(db)
    .await
    .map_err(SqlxErrorExt::into_app_error)?
    .ok_or_else(|| AppError::not_found(format!("User with id {} not found", user_id)))?;
    Ok(Json(user))
}

#[utoipa::path(
    delete,
    path = "/api/users/{user_id}",
    params(("user_id" = i64, Path, description = "User ID")),
    responses(
        (status = 204, description = "User deleted"),
        (status = 404, description = "User not found", body = AppError),
        (status = 500, description = "Internal server error", body = AppError)
    ),
    tag = "users"
)]
#[tracing::instrument]
pub async fn delete_user(Path(user_id): Path<i64>) -> Result<StatusCode, AppError> {
    let db = get_db().await;
    let result = sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
        .execute(db)
        .await
        .map_err(SqlxErrorExt::into_app_error)?;
    if result.rows_affected() > 0 {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::not_found(format!(
            "User with id {} not found",
            user_id
        )))
    }
}

// ── Products ───────────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/products",
    responses(
        (status = 200, description = "List of products", body = Vec<Product>),
        (status = 500, description = "Internal server error", body = AppError)
    ),
    tag = "products"
)]
#[tracing::instrument]
pub async fn list_products() -> Result<Json<Vec<Product>>, AppError> {
    let db = get_db().await;
    let rows = sqlx::query!(
        "SELECT id, name, description, price, category, status, created_at FROM products ORDER BY id DESC"
    )
    .fetch_all(db)
    .await
    .map_err(SqlxErrorExt::into_app_error)?;

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
    Ok(Json(products))
}

#[utoipa::path(
    post,
    path = "/api/products",
    request_body = CreateProductRequest,
    responses(
        (status = 201, description = "Product created", body = Product),
        (status = 422, description = "Validation error", body = AppError),
        (status = 500, description = "Internal server error", body = AppError)
    ),
    tag = "products"
)]
#[tracing::instrument]
pub async fn create_product(
    Json(payload): Json<CreateProductRequest>,
) -> Result<(StatusCode, Json<Product>), AppError> {
    payload.validate_request()?;

    let db = get_db().await;
    let row = sqlx::query!(
        "INSERT INTO products (name, description, price, category, status) VALUES ($1, $2, $3, $4, $5) RETURNING id, name, description, price, category, status, created_at",
        payload.name,
        payload.description,
        payload.price,
        payload.category,
        payload.status
    )
    .fetch_one(db)
    .await
    .map_err(SqlxErrorExt::into_app_error)?;

    let product = Product {
        id: row.id,
        name: row.name,
        description: row.description,
        price: row.price,
        category: row.category,
        status: row.status,
        created_at: row.created_at.to_string(),
    };
    Ok((StatusCode::CREATED, Json(product)))
}

#[utoipa::path(
    put,
    path = "/api/products/{product_id}",
    params(("product_id" = i64, Path, description = "Product ID")),
    request_body = UpdateProductRequest,
    responses(
        (status = 200, description = "Product updated", body = Product),
        (status = 404, description = "Product not found", body = AppError),
        (status = 422, description = "Validation error", body = AppError),
        (status = 500, description = "Internal server error", body = AppError)
    ),
    tag = "products"
)]
#[tracing::instrument]
pub async fn update_product(
    Path(product_id): Path<i64>,
    Json(payload): Json<UpdateProductRequest>,
) -> Result<Json<Product>, AppError> {
    payload.validate_request()?;

    let db = get_db().await;
    let row = sqlx::query!(
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
    .map_err(SqlxErrorExt::into_app_error)?
    .ok_or_else(|| {
        AppError::not_found(format!("Product with id {} not found", product_id))
    })?;

    let product = Product {
        id: row.id,
        name: row.name,
        description: row.description,
        price: row.price,
        category: row.category,
        status: row.status,
        created_at: row.created_at.to_string(),
    };
    Ok(Json(product))
}

#[utoipa::path(
    delete,
    path = "/api/products/{product_id}",
    params(("product_id" = i64, Path, description = "Product ID")),
    responses(
        (status = 204, description = "Product deleted"),
        (status = 404, description = "Product not found", body = AppError),
        (status = 500, description = "Internal server error", body = AppError)
    ),
    tag = "products"
)]
#[tracing::instrument]
pub async fn delete_product(Path(product_id): Path<i64>) -> Result<StatusCode, AppError> {
    let db = get_db().await;
    let result = sqlx::query!("DELETE FROM products WHERE id = $1", product_id)
        .execute(db)
        .await
        .map_err(SqlxErrorExt::into_app_error)?;
    if result.rows_affected() > 0 {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::not_found(format!(
            "Product with id {} not found",
            product_id
        )))
    }
}

// ── Dashboard ──────────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/dashboard/stats",
    responses(
        (status = 200, description = "Dashboard statistics", body = DashboardStats),
        (status = 500, description = "Internal server error", body = AppError)
    ),
    tag = "dashboard"
)]
#[tracing::instrument]
pub async fn get_dashboard_stats() -> Result<Json<DashboardStats>, AppError> {
    let db = get_db().await;

    let total_users = sqlx::query_scalar!("SELECT COUNT(*) FROM users")
        .fetch_one(db)
        .await
        .map_err(SqlxErrorExt::into_app_error)?
        .unwrap_or(0);

    let total_products = sqlx::query_scalar!("SELECT COUNT(*) FROM products")
        .fetch_one(db)
        .await
        .map_err(SqlxErrorExt::into_app_error)?
        .unwrap_or(0);

    let active_products =
        sqlx::query_scalar!("SELECT COUNT(*) FROM products WHERE status = 'active'")
            .fetch_one(db)
            .await
            .map_err(SqlxErrorExt::into_app_error)?
            .unwrap_or(0);

    let recent_users = sqlx::query_as!(
        User,
        "SELECT id, username, display_name, role, tier FROM users ORDER BY id DESC LIMIT 5"
    )
    .fetch_all(db)
    .await
    .map_err(SqlxErrorExt::into_app_error)?;

    Ok(Json(DashboardStats {
        total_users,
        total_products,
        active_products,
        recent_users,
    }))
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
