use axum::Router;
use shared_types::{
    AppError, AppErrorKind, AuthResponse, AuthUser, CreateProductRequest, CreateUserRequest,
    DashboardStats, LoginRequest, Product, RegisterRequest, UpdateProductRequest,
    UpdateProfileRequest, UpdateTierRequest, UpdateUserRequest, User, UserTier,
};
use sqlx::{Pool, Postgres};
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};

use crate::db::AppState;
use crate::health;
use crate::rest;

/// OpenAPI documentation for the API.
#[derive(OpenApi)]
#[openapi(
    paths(
        rest::list_users,
        rest::get_user,
        rest::create_user,
        rest::update_user,
        rest::delete_user,
        rest::update_user_tier,
        rest::list_products,
        rest::create_product,
        rest::update_product,
        rest::delete_product,
        rest::get_dashboard_stats,
        rest::register,
        rest::login,
        rest::logout,
        rest::upload_avatar,
        health::health_check,
    ),
    components(schemas(
        User,
        Product,
        DashboardStats,
        AppError,
        AppErrorKind,
        CreateUserRequest,
        UpdateUserRequest,
        CreateProductRequest,
        UpdateProductRequest,
        AuthUser,
        UserTier,
        LoginRequest,
        RegisterRequest,
        AuthResponse,
        UpdateProfileRequest,
        UpdateTierRequest,
        health::HealthResponse,
    )),
    tags(
        (name = "auth", description = "Authentication endpoints"),
        (name = "users", description = "User management endpoints"),
        (name = "products", description = "Product management endpoints"),
        (name = "dashboard", description = "Dashboard statistics"),
        (name = "health", description = "Health check endpoint")
    )
)]
pub struct ApiDoc;

/// Build an Axum router that serves the API docs at `/docs`
/// and the REST API at `/api/*`.
///
/// Accepts a `PgPool` to construct `AppState` and apply it via `.with_state()`.
pub fn api_router(pool: Pool<Postgres>) -> Router {
    let state = AppState { pool };

    Router::new()
        .merge(rest::rest_router())
        .route("/health", axum::routing::get(health::health_check))
        .route(
            "/auth/callback/{provider}",
            axum::routing::get(crate::auth::oauth_callback::oauth_callback),
        )
        .with_state(state)
        .merge(Scalar::with_url("/docs", ApiDoc::openapi()))
}
