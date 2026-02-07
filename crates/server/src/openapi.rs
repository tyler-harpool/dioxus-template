use axum::Router;
use shared_types::{DashboardStats, Product, User};
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};

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
    ),
    components(schemas(
        User,
        Product,
        DashboardStats,
        rest::UserPayload,
        rest::ProductPayload,
    )),
    tags(
        (name = "users", description = "User management endpoints"),
        (name = "products", description = "Product management endpoints"),
        (name = "dashboard", description = "Dashboard statistics")
    )
)]
pub struct ApiDoc;

/// Build an Axum router that serves the API docs at `/docs`
/// and the REST API at `/api/*`.
pub fn api_router() -> Router {
    Router::new()
        .merge(rest::rest_router())
        .merge(Scalar::with_url("/docs", ApiDoc::openapi()))
}
