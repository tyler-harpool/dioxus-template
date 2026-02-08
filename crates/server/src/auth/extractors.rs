use axum::{extract::FromRequestParts, http::request::Parts};
use shared_types::{AppError, UserTier};

use super::jwt::Claims;

/// Extractor that requires authentication. Returns 401 if no valid token.
pub struct AuthRequired(pub Claims);

impl<S: Send + Sync> FromRequestParts<S> for AuthRequired {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Claims>()
            .cloned()
            .map(AuthRequired)
            .ok_or_else(|| AppError::unauthorized("Authentication required"))
    }
}

/// Extractor that optionally extracts auth claims. Never fails.
pub struct MaybeAuth(pub Option<Claims>);

impl<S: Send + Sync> FromRequestParts<S> for MaybeAuth {
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        Ok(MaybeAuth(parts.extensions.get::<Claims>().cloned()))
    }
}

/// Extractor that requires authentication AND a minimum user tier.
/// Returns 401 if unauthenticated, 403 if insufficient tier.
pub struct TierRequired<const TIER: u8>(pub Claims);

impl<const TIER: u8, S: Send + Sync> FromRequestParts<S> for TierRequired<TIER> {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let claims = parts
            .extensions
            .get::<Claims>()
            .cloned()
            .ok_or_else(|| AppError::unauthorized("Authentication required"))?;

        let user_tier = UserTier::from_str_or_default(&claims.tier);
        let required_tier = match TIER {
            0 => UserTier::Free,
            1 => UserTier::Premium,
            2 => UserTier::Elite,
            _ => UserTier::Elite,
        };

        if !user_tier.has_access(&required_tier) {
            return Err(AppError::forbidden(format!(
                "{} tier or higher required",
                required_tier.as_str()
            )));
        }

        Ok(TierRequired(claims))
    }
}
