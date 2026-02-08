use axum::Router;
use shared_types::{
    AppError, AppErrorKind, CreateProductRequest, CreateUserRequest, DashboardStats, Product,
    UpdateProductRequest, UpdateUserRequest, User,
};
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};

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
        rest::list_products,
        rest::create_product,
        rest::update_product,
        rest::delete_product,
        rest::get_dashboard_stats,
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
        health::HealthResponse,
    )),
    tags(
        (name = "users", description = "User management endpoints"),
        (name = "products", description = "Product management endpoints"),
        (name = "dashboard", description = "Dashboard statistics"),
        (name = "health", description = "Health check endpoint")
    )
)]
pub struct ApiDoc;

/// Build an Axum router that serves the API docs at `/docs`
/// and the REST API at `/api/*`.
pub fn api_router() -> Router {
    Router::new()
        .merge(rest::rest_router())
        .route("/health", axum::routing::get(health::health_check))
        .route(
            "/auth/callback/{provider}",
            axum::routing::get(crate::auth::oauth_callback::oauth_callback),
        )
        .merge(Scalar::with_url("/docs", ApiDoc::openapi()))
}
