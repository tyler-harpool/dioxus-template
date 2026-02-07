use axum::Router;
use shared_types::User;
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
    components(schemas(User)),
    tags(
        (name = "users", description = "User management endpoints")
    )
)]
pub struct ApiDoc;

/// Build an Axum router that serves the API docs at `/docs`.
pub fn swagger_router() -> Router {
    Router::new().merge(Scalar::with_url("/docs", ApiDoc::openapi()))
}
