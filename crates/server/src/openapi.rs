use axum::Router;
use shared_types::User;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

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

/// Build an Axum router that serves the Swagger UI at `/swagger-ui`.
pub fn swagger_router() -> Router {
    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
}
