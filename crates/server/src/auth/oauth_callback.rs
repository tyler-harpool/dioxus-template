use axum::{
    extract::{Path, Query},
    response::{IntoResponse, Redirect, Response},
};
use oauth2::{AuthorizationCode, TokenResponse};
use shared_types::{OAuthProvider, UserTier};

use super::{cookies, jwt, oauth, oauth_state};
use crate::db::get_db;

/// Query parameters received from the OAuth provider callback.
#[derive(Debug, serde::Deserialize)]
pub struct CallbackQuery {
    pub code: String,
    pub state: String,
}

/// Axum handler for `/auth/callback/{provider}`.
/// Exchanges the authorization code for tokens, fetches user info,
/// upserts the user, creates JWTs, sets HTTP-only cookies, and redirects to `/`.
pub async fn oauth_callback(
    Path(provider_str): Path<String>,
    Query(params): Query<CallbackQuery>,
) -> Result<Response, Response> {
    let error_redirect = |msg: &str| {
        Redirect::to(&format!("/login?error={}", urlencoding::encode(msg))).into_response()
    };

    let provider = OAuthProvider::parse_provider(&provider_str)
        .ok_or_else(|| error_redirect("Unknown OAuth provider"))?;

    // Verify CSRF state and retrieve PKCE verifier
    let verifier = oauth_state::take_verifier(&params.state)
        .await
        .ok_or_else(|| error_redirect("Invalid or expired OAuth state"))?;

    // Exchange code for access token
    let client = oauth::build_oauth_client(&provider)
        .map_err(|e| error_redirect(&format!("OAuth config error: {}", e)))?;

    let http_client = reqwest::Client::new();
    let token_response = client
        .exchange_code(AuthorizationCode::new(params.code))
        .set_pkce_verifier(verifier)
        .request_async(&http_client)
        .await
        .map_err(|e| error_redirect(&format!("Token exchange failed: {}", e)))?;

    let access_token_str = token_response.access_token().secret();

    // Fetch user info from the provider
    let user_info = match &provider {
        OAuthProvider::Google => {
            let info = oauth::fetch_google_user_info(access_token_str)
                .await
                .map_err(|e| error_redirect(&e))?;

            oauth::OAuthUserInfo {
                provider: OAuthProvider::Google,
                provider_id: info.sub,
                email: info.email.unwrap_or_default(),
                display_name: info.name.unwrap_or_else(|| "Google User".to_string()),
                avatar_url: info.picture,
            }
        }
        OAuthProvider::GitHub => {
            let info = oauth::fetch_github_user_info(access_token_str)
                .await
                .map_err(|e| error_redirect(&e))?;

            oauth::OAuthUserInfo {
                provider: OAuthProvider::GitHub,
                provider_id: info.id.to_string(),
                email: info.email.unwrap_or_default(),
                display_name: info.name.unwrap_or_else(|| info.login.clone()),
                avatar_url: info.avatar_url,
            }
        }
    };

    if user_info.email.is_empty() {
        return Err(error_redirect(
            "Could not retrieve email from OAuth provider",
        ));
    }

    // Upsert user in the database
    let db = get_db().await;
    let (user_id, role, tier_str) = oauth::upsert_oauth_user(db, &user_info)
        .await
        .map_err(|e| error_redirect(&e))?;

    let tier = UserTier::from_str_or_default(&tier_str);

    // Create JWTs
    let jwt_access = jwt::create_access_token(user_id, &user_info.email, &role, tier.as_str())
        .map_err(|e| error_redirect(&format!("JWT error: {}", e)))?;

    let (jwt_refresh, expires_at) =
        jwt::create_refresh_token(user_id, &user_info.email, &role, tier.as_str())
            .map_err(|e| error_redirect(&format!("JWT error: {}", e)))?;

    // Store refresh token
    sqlx::query!(
        "INSERT INTO refresh_tokens (user_id, token_hash, expires_at) VALUES ($1, $2, $3)",
        user_id,
        jwt_refresh,
        expires_at
    )
    .execute(db)
    .await
    .map_err(|e| error_redirect(&format!("DB error: {}", e)))?;

    // Build redirect response with auth cookies
    let mut response = Redirect::to("/").into_response();
    cookies::set_auth_cookies(response.headers_mut(), &jwt_access, &jwt_refresh);

    Ok(response)
}
