use axum::Router;
use shared_types::{DashboardStats, Product, User};
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};

/// OpenAPI documentation for the API.
#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::get_user,
        crate::api::list_users,
        crate::api::create_user,
    ),
    components(schemas(User, Product, DashboardStats)),
    tags(
        (name = "users", description = "User management endpoints"),
        (name = "products", description = "Product management endpoints"),
        (name = "dashboard", description = "Dashboard statistics")
    )
)]
pub struct ApiDoc;

/// Build an Axum router that serves the API docs at `/docs`.
pub fn swagger_router() -> Router {
    Router::new().merge(Scalar::with_url("/docs", ApiDoc::openapi()))
}
