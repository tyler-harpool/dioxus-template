use serde::{Deserialize, Serialize};

#[cfg(feature = "validation")]
use validator::Validate;

/// Request DTO for creating a user.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "validation", derive(Validate))]
pub struct CreateUserRequest {
    #[cfg_attr(
        feature = "validation",
        validate(length(min = 3, message = "Username must be at least 3 characters"))
    )]
    pub username: String,
    #[cfg_attr(
        feature = "validation",
        validate(length(min = 1, message = "Display name is required"))
    )]
    pub display_name: String,
}

/// Request DTO for updating a user.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "validation", derive(Validate))]
pub struct UpdateUserRequest {
    #[cfg_attr(
        feature = "validation",
        validate(length(min = 3, message = "Username must be at least 3 characters"))
    )]
    pub username: String,
    #[cfg_attr(
        feature = "validation",
        validate(length(min = 1, message = "Display name is required"))
    )]
    pub display_name: String,
}

/// Request DTO for creating a product.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "validation", derive(Validate))]
pub struct CreateProductRequest {
    #[cfg_attr(
        feature = "validation",
        validate(length(min = 1, message = "Product name is required"))
    )]
    pub name: String,
    pub description: String,
    #[cfg_attr(
        feature = "validation",
        validate(range(min = 0.0, message = "Price must be non-negative"))
    )]
    pub price: f64,
    #[cfg_attr(
        feature = "validation",
        validate(length(min = 1, message = "Category is required"))
    )]
    pub category: String,
    pub status: String,
}

/// Request DTO for updating a product.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "validation", derive(Validate))]
pub struct UpdateProductRequest {
    #[cfg_attr(
        feature = "validation",
        validate(length(min = 1, message = "Product name is required"))
    )]
    pub name: String,
    pub description: String,
    #[cfg_attr(
        feature = "validation",
        validate(range(min = 0.0, message = "Price must be non-negative"))
    )]
    pub price: f64,
    #[cfg_attr(
        feature = "validation",
        validate(length(min = 1, message = "Category is required"))
    )]
    pub category: String,
    pub status: String,
}

/// Request DTO for updating the current user's profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "validation", derive(Validate))]
pub struct UpdateProfileRequest {
    #[cfg_attr(
        feature = "validation",
        validate(length(min = 1, message = "Display name is required"))
    )]
    pub display_name: String,
    #[cfg_attr(
        feature = "validation",
        validate(email(message = "Valid email is required"))
    )]
    pub email: String,
}

/// Response returned after successful authentication (login or register).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct AuthResponse {
    pub user: crate::AuthUser,
    pub access_token: String,
}

/// Request DTO for updating a user's subscription tier.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct UpdateTierRequest {
    pub tier: String,
}
