use axum::{
    extract::{Multipart, Path, State},
    http::StatusCode,
    routing::{get, post, put},
    Json, Router,
};
use shared_types::{
    AppError, AuthResponse, AuthUser, CreateProductRequest, CreateUserRequest, DashboardStats,
    LoginRequest, Product, RegisterRequest, UpdateProductRequest, UpdateTierRequest,
    UpdateUserRequest, User, UserTier,
};
use sqlx::{Pool, Postgres};

use crate::auth::{extractors::AuthRequired, jwt, password as pw};
use crate::db::AppState;
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
#[tracing::instrument(skip(pool))]
pub async fn list_users(
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<Vec<User>>, AppError> {
    let users = sqlx::query_as!(
        User,
        "SELECT id, username, display_name, role, tier FROM users"
    )
    .fetch_all(&pool)
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
#[tracing::instrument(skip(pool))]
pub async fn get_user(
    State(pool): State<Pool<Postgres>>,
    Path(user_id): Path<i64>,
) -> Result<Json<User>, AppError> {
    let user = sqlx::query_as!(
        User,
        "SELECT id, username, display_name, role, tier FROM users WHERE id = $1",
        user_id
    )
    .fetch_optional(&pool)
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
#[tracing::instrument(skip(pool))]
pub async fn create_user(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<User>), AppError> {
    payload.validate_request()?;

    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (username, display_name) VALUES ($1, $2) RETURNING id, username, display_name, role, tier",
        payload.username,
        payload.display_name
    )
    .fetch_one(&pool)
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
#[tracing::instrument(skip(pool))]
pub async fn update_user(
    State(pool): State<Pool<Postgres>>,
    Path(user_id): Path<i64>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<User>, AppError> {
    payload.validate_request()?;

    let user = sqlx::query_as!(
        User,
        "UPDATE users SET username = $2, display_name = $3 WHERE id = $1 RETURNING id, username, display_name, role, tier",
        user_id,
        payload.username,
        payload.display_name
    )
    .fetch_optional(&pool)
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
#[tracing::instrument(skip(pool))]
pub async fn delete_user(
    State(pool): State<Pool<Postgres>>,
    Path(user_id): Path<i64>,
) -> Result<StatusCode, AppError> {
    let result = sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
        .execute(&pool)
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
#[tracing::instrument(skip(pool))]
pub async fn list_products(
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<Vec<Product>>, AppError> {
    let rows = sqlx::query!(
        "SELECT id, name, description, price, category, status, created_at FROM products ORDER BY id DESC"
    )
    .fetch_all(&pool)
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
#[tracing::instrument(skip(pool))]
pub async fn create_product(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<CreateProductRequest>,
) -> Result<(StatusCode, Json<Product>), AppError> {
    payload.validate_request()?;

    let row = sqlx::query!(
        "INSERT INTO products (name, description, price, category, status) VALUES ($1, $2, $3, $4, $5) RETURNING id, name, description, price, category, status, created_at",
        payload.name,
        payload.description,
        payload.price,
        payload.category,
        payload.status
    )
    .fetch_one(&pool)
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
#[tracing::instrument(skip(pool))]
pub async fn update_product(
    State(pool): State<Pool<Postgres>>,
    Path(product_id): Path<i64>,
    Json(payload): Json<UpdateProductRequest>,
) -> Result<Json<Product>, AppError> {
    payload.validate_request()?;

    let row = sqlx::query!(
        "UPDATE products SET name = $2, description = $3, price = $4, category = $5, status = $6 WHERE id = $1 RETURNING id, name, description, price, category, status, created_at",
        product_id,
        payload.name,
        payload.description,
        payload.price,
        payload.category,
        payload.status
    )
    .fetch_optional(&pool)
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
#[tracing::instrument(skip(pool))]
pub async fn delete_product(
    State(pool): State<Pool<Postgres>>,
    Path(product_id): Path<i64>,
) -> Result<StatusCode, AppError> {
    let result = sqlx::query!("DELETE FROM products WHERE id = $1", product_id)
        .execute(&pool)
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
#[tracing::instrument(skip(pool))]
pub async fn get_dashboard_stats(
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<DashboardStats>, AppError> {
    let total_users = sqlx::query_scalar!("SELECT COUNT(*) FROM users")
        .fetch_one(&pool)
        .await
        .map_err(SqlxErrorExt::into_app_error)?
        .unwrap_or(0);

    let total_products = sqlx::query_scalar!("SELECT COUNT(*) FROM products")
        .fetch_one(&pool)
        .await
        .map_err(SqlxErrorExt::into_app_error)?
        .unwrap_or(0);

    let active_products =
        sqlx::query_scalar!("SELECT COUNT(*) FROM products WHERE status = 'active'")
            .fetch_one(&pool)
            .await
            .map_err(SqlxErrorExt::into_app_error)?
            .unwrap_or(0);

    let recent_users = sqlx::query_as!(
        User,
        "SELECT id, username, display_name, role, tier FROM users ORDER BY id DESC LIMIT 5"
    )
    .fetch_all(&pool)
    .await
    .map_err(SqlxErrorExt::into_app_error)?;

    Ok(Json(DashboardStats {
        total_users,
        total_products,
        active_products,
        recent_users,
    }))
}

// ── Auth ───────────────────────────────────────────────

#[utoipa::path(
    post,
    path = "/api/auth/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User registered", body = AuthResponse),
        (status = 422, description = "Validation error (e.g. duplicate email)", body = AppError),
        (status = 500, description = "Internal server error", body = AppError)
    ),
    tag = "auth"
)]
#[tracing::instrument(skip(pool, payload))]
pub async fn register(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), AppError> {
    let password_hash = pw::hash_password(&payload.password)
        .map_err(|e| AppError::internal(e.to_string()))?;

    let user = sqlx::query!(
        "INSERT INTO users (username, email, password_hash, display_name) VALUES ($1, $2, $3, $4) RETURNING id, username, display_name, email, role, tier, avatar_url",
        payload.username,
        payload.email,
        password_hash,
        payload.display_name
    )
    .fetch_one(&pool)
    .await
    .map_err(SqlxErrorExt::into_app_error)?;

    let user_email = user.email.unwrap_or_default();
    let user_tier = UserTier::from_str_or_default(&user.tier);

    let access_token =
        jwt::create_access_token(user.id, &user_email, &user.role, user_tier.as_str())
            .map_err(|e| AppError::internal(e.to_string()))?;

    let (refresh_token, expires_at) =
        jwt::create_refresh_token(user.id, &user_email, &user.role, user_tier.as_str())
            .map_err(|e| AppError::internal(e.to_string()))?;

    sqlx::query!(
        "INSERT INTO refresh_tokens (user_id, token_hash, expires_at) VALUES ($1, $2, $3)",
        user.id,
        refresh_token,
        expires_at
    )
    .execute(&pool)
    .await
    .map_err(SqlxErrorExt::into_app_error)?;

    let auth_user = AuthUser {
        id: user.id,
        username: user.username,
        display_name: user.display_name,
        email: user_email,
        role: user.role,
        tier: user_tier,
        avatar_url: user.avatar_url,
    };

    Ok((
        StatusCode::CREATED,
        Json(AuthResponse {
            user: auth_user,
            access_token,
        }),
    ))
}

#[utoipa::path(
    post,
    path = "/api/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = AuthResponse),
        (status = 401, description = "Invalid credentials", body = AppError),
        (status = 500, description = "Internal server error", body = AppError)
    ),
    tag = "auth"
)]
#[tracing::instrument(skip(pool, payload))]
pub async fn login(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let user = sqlx::query!(
        "SELECT id, username, display_name, email, password_hash, role, tier, avatar_url FROM users WHERE email = $1",
        payload.email
    )
    .fetch_optional(&pool)
    .await
    .map_err(SqlxErrorExt::into_app_error)?
    .ok_or_else(|| AppError::unauthorized("Invalid email or password"))?;

    let password_hash = user
        .password_hash
        .ok_or_else(|| AppError::unauthorized("Invalid email or password"))?;

    let valid = pw::verify_password(&payload.password, &password_hash)
        .map_err(|e| AppError::internal(e.to_string()))?;

    if !valid {
        return Err(AppError::unauthorized("Invalid email or password"));
    }

    let user_email = user.email.unwrap_or_default();
    let user_tier = UserTier::from_str_or_default(&user.tier);

    let access_token =
        jwt::create_access_token(user.id, &user_email, &user.role, user_tier.as_str())
            .map_err(|e| AppError::internal(e.to_string()))?;

    let (refresh_token, expires_at) =
        jwt::create_refresh_token(user.id, &user_email, &user.role, user_tier.as_str())
            .map_err(|e| AppError::internal(e.to_string()))?;

    sqlx::query!(
        "INSERT INTO refresh_tokens (user_id, token_hash, expires_at) VALUES ($1, $2, $3)",
        user.id,
        refresh_token,
        expires_at
    )
    .execute(&pool)
    .await
    .map_err(SqlxErrorExt::into_app_error)?;

    let auth_user = AuthUser {
        id: user.id,
        username: user.username,
        display_name: user.display_name,
        email: user_email,
        role: user.role,
        tier: user_tier,
        avatar_url: user.avatar_url,
    };

    Ok(Json(AuthResponse {
        user: auth_user,
        access_token,
    }))
}

#[utoipa::path(
    post,
    path = "/api/auth/logout",
    responses(
        (status = 204, description = "Logged out"),
        (status = 401, description = "Not authenticated", body = AppError),
        (status = 500, description = "Internal server error", body = AppError)
    ),
    tag = "auth",
    security(("bearer_auth" = []))
)]
#[tracing::instrument(skip(pool, auth))]
pub async fn logout(
    State(pool): State<Pool<Postgres>>,
    auth: AuthRequired,
) -> Result<StatusCode, AppError> {
    sqlx::query!(
        "UPDATE refresh_tokens SET revoked = TRUE WHERE user_id = $1 AND revoked = FALSE",
        auth.0.sub
    )
    .execute(&pool)
    .await
    .map_err(SqlxErrorExt::into_app_error)?;

    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    put,
    path = "/api/users/{user_id}/tier",
    params(("user_id" = i64, Path, description = "User ID")),
    request_body = UpdateTierRequest,
    responses(
        (status = 200, description = "Tier updated", body = User),
        (status = 401, description = "Not authenticated", body = AppError),
        (status = 403, description = "Forbidden — admin role required", body = AppError),
        (status = 404, description = "User not found", body = AppError),
        (status = 422, description = "Invalid tier value", body = AppError),
        (status = 500, description = "Internal server error", body = AppError)
    ),
    tag = "users",
    security(("bearer_auth" = []))
)]
#[tracing::instrument(skip(pool, auth))]
pub async fn update_user_tier(
    State(pool): State<Pool<Postgres>>,
    auth: AuthRequired,
    Path(user_id): Path<i64>,
    Json(payload): Json<UpdateTierRequest>,
) -> Result<Json<User>, AppError> {
    if auth.0.role != "admin" {
        return Err(AppError::forbidden(
            "Admin role required to change user tiers",
        ));
    }

    let valid_tiers = ["free", "premium", "elite"];
    let tier_lower = payload.tier.to_lowercase();
    if !valid_tiers.contains(&tier_lower.as_str()) {
        return Err(AppError::validation(
            "Invalid tier value",
            Default::default(),
        ));
    }

    let user = sqlx::query_as!(
        User,
        "UPDATE users SET tier = $2 WHERE id = $1 RETURNING id, username, display_name, role, tier",
        user_id,
        tier_lower
    )
    .fetch_optional(&pool)
    .await
    .map_err(SqlxErrorExt::into_app_error)?
    .ok_or_else(|| AppError::not_found(format!("User with id {} not found", user_id)))?;

    Ok(Json(user))
}

// ── Avatar Upload ───────────────────────────────────────

const MAX_AVATAR_SIZE: usize = 2 * 1024 * 1024; // 2 MB

#[utoipa::path(
    post,
    path = "/api/users/me/avatar",
    responses(
        (status = 200, description = "Avatar uploaded", body = AuthUser),
        (status = 401, description = "Not authenticated", body = AppError),
        (status = 422, description = "Validation error", body = AppError),
        (status = 500, description = "Internal server error", body = AppError)
    ),
    tag = "users",
    security(("bearer_auth" = []))
)]
#[tracing::instrument(skip(pool, auth, multipart))]
pub async fn upload_avatar(
    State(pool): State<Pool<Postgres>>,
    auth: AuthRequired,
    mut multipart: Multipart,
) -> Result<Json<AuthUser>, AppError> {
    let mut file_bytes: Option<Vec<u8>> = None;
    let mut content_type: Option<String> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::validation(e.to_string(), Default::default()))?
    {
        let ct = field
            .content_type()
            .unwrap_or("application/octet-stream")
            .to_string();

        let allowed = ["image/jpeg", "image/png", "image/webp"];
        if !allowed.contains(&ct.as_str()) {
            return Err(AppError::validation(
                "Only JPEG, PNG, and WebP images are allowed",
                Default::default(),
            ));
        }

        let data = field
            .bytes()
            .await
            .map_err(|e| AppError::internal(e.to_string()))?;

        if data.len() > MAX_AVATAR_SIZE {
            return Err(AppError::validation(
                "Avatar must be under 2 MB",
                Default::default(),
            ));
        }

        content_type = Some(ct);
        file_bytes = Some(data.to_vec());
        break;
    }

    let bytes = file_bytes.ok_or_else(|| {
        AppError::validation("No file provided", Default::default())
    })?;
    let ct = content_type.unwrap_or_default();

    let avatar_url = crate::s3::upload_avatar(auth.0.sub, &ct, &bytes)
        .await
        .map_err(|e| AppError::internal(e))?;

    let user = sqlx::query!(
        "UPDATE users SET avatar_url = $2 WHERE id = $1 RETURNING id, username, display_name, email, role, tier, avatar_url",
        auth.0.sub,
        avatar_url
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        use crate::error_convert::SqlxErrorExt;
        e.into_app_error()
    })?;

    Ok(Json(AuthUser {
        id: user.id,
        username: user.username,
        display_name: user.display_name,
        email: user.email.unwrap_or_default(),
        role: user.role,
        tier: UserTier::from_str_or_default(&user.tier),
        avatar_url: user.avatar_url,
    }))
}

/// Build the REST API router with all resource routes.
pub fn rest_router() -> Router<AppState> {
    Router::new()
        .route("/api/users", get(list_users).post(create_user))
        .route(
            "/api/users/{user_id}",
            get(get_user).put(update_user).delete(delete_user),
        )
        .route("/api/users/{user_id}/tier", put(update_user_tier))
        .route("/api/products", get(list_products).post(create_product))
        .route(
            "/api/products/{product_id}",
            put(update_product).delete(delete_product),
        )
        .route("/api/dashboard/stats", get(get_dashboard_stats))
        .route("/api/users/me/avatar", post(upload_avatar))
        .route("/api/auth/register", post(register))
        .route("/api/auth/login", post(login))
        .route("/api/auth/logout", post(logout))
}
