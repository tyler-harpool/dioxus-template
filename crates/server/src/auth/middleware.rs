use axum::{extract::Request, middleware::Next, response::Response};

use super::cookies::{self, CookieSlot, PendingCookieAction};
use super::jwt::{self, validate_access_token};
use crate::db::get_db;

/// Permissive auth middleware that handles authentication and cookie management.
///
/// On each request:
/// 1. Validates the access token from cookies (or Bearer header fallback)
/// 2. If expired, attempts transparent refresh using the refresh cookie
/// 3. Inserts a `CookieSlot` so server functions can schedule cookie changes
/// 4. After the handler runs, applies any pending cookie actions to the response
///
/// Does NOT reject unauthenticated requests — downstream handlers decide authorization.
pub async fn auth_middleware(mut req: Request, next: Next) -> Response {
    let headers = req.headers().clone();
    let mut refresh_cookies: Option<(String, String)> = None;

    // Validate access token and insert Claims into extensions
    if let Some(token) = cookies::extract_access_token(&headers) {
        match validate_access_token(&token) {
            Ok(claims) => {
                req.extensions_mut().insert(claims);
            }
            Err(_) => {
                // Access token invalid/expired — try transparent refresh
                if let Some(refresh_token) = cookies::extract_refresh_token(&headers) {
                    if let Some((new_access, new_refresh)) =
                        try_transparent_refresh(&refresh_token, &mut req).await
                    {
                        refresh_cookies = Some((new_access, new_refresh));
                    }
                }
            }
        }
    }

    // Insert a CookieSlot so server functions can schedule cookie changes
    let slot = CookieSlot::default();
    req.extensions_mut().insert(slot.clone());

    let mut response = next.run(req).await;

    // Apply cookies from transparent refresh
    if let Some((access, refresh)) = refresh_cookies {
        cookies::set_auth_cookies(response.headers_mut(), &access, &refresh);
    }

    // Apply any cookie action scheduled by server functions
    if let Some(action) = slot.0.lock().unwrap().take() {
        match action {
            PendingCookieAction::Set {
                access_token,
                refresh_token,
            } => {
                cookies::set_auth_cookies(response.headers_mut(), &access_token, &refresh_token);
            }
            PendingCookieAction::Clear => {
                cookies::clear_auth_cookies(response.headers_mut());
            }
        }
    }

    response
}

/// Attempt to transparently refresh the session using the refresh token.
/// On success: inserts new Claims into request extensions and returns
/// the new token pair for the middleware to set as cookies.
async fn try_transparent_refresh(
    refresh_token: &str,
    req: &mut Request,
) -> Option<(String, String)> {
    let claims = validate_access_token(refresh_token).ok()?;

    let db = get_db().await;

    // Verify token exists and is not revoked
    let stored = sqlx::query!(
        "SELECT id, revoked FROM refresh_tokens WHERE token_hash = $1 AND user_id = $2",
        refresh_token,
        claims.sub
    )
    .fetch_optional(db)
    .await
    .ok()
    .flatten()?;

    if stored.revoked {
        return None;
    }

    // Revoke old refresh token
    let _ = sqlx::query!(
        "UPDATE refresh_tokens SET revoked = TRUE WHERE id = $1",
        stored.id
    )
    .execute(db)
    .await;

    // Issue new tokens
    let new_access =
        jwt::create_access_token(claims.sub, &claims.email, &claims.role, &claims.tier).ok()?;
    let (new_refresh, expires_at) =
        jwt::create_refresh_token(claims.sub, &claims.email, &claims.role, &claims.tier).ok()?;

    // Store new refresh token
    let _ = sqlx::query!(
        "INSERT INTO refresh_tokens (user_id, token_hash, expires_at) VALUES ($1, $2, $3)",
        claims.sub,
        new_refresh,
        expires_at
    )
    .execute(db)
    .await;

    // Validate the new access token to get fresh claims
    let new_claims = validate_access_token(&new_access).ok()?;
    req.extensions_mut().insert(new_claims);

    Some((new_access, new_refresh))
}
