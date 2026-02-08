use dioxus::prelude::*;
use shared_types::{AuthUser, DashboardStats, Product, User};

#[cfg(feature = "server")]
use crate::db::get_db;

#[cfg(feature = "server")]
use crate::error_convert::{AppErrorExt, SqlxErrorExt, ValidateRequest};

#[cfg(feature = "server")]
use shared_types::{
    CreateProductRequest, CreateUserRequest, UpdateProductRequest, UpdateUserRequest, UserTier,
};

/// Get premium analytics data. Requires Premium tier or above.
#[cfg_attr(feature = "server", tracing::instrument)]
#[server]
pub async fn get_premium_analytics() -> Result<shared_types::PremiumAnalytics, ServerFnError> {
    use crate::auth::{cookies, jwt};
    use shared_types::AppError;

    // Extract and validate the caller's tier from the access token
    let ctx = dioxus::fullstack::FullstackContext::current();
    let headers = ctx.as_ref().map(|c| c.parts_mut().headers.clone());

    let headers = headers
        .ok_or_else(|| AppError::unauthorized("Authentication required").into_server_fn_error())?;

    let token = cookies::extract_access_token(&headers)
        .ok_or_else(|| AppError::unauthorized("Authentication required").into_server_fn_error())?;

    let claims = jwt::validate_access_token(&token)
        .map_err(|_| AppError::unauthorized("Invalid token").into_server_fn_error())?;

    let user_tier = UserTier::from_str_or_default(&claims.tier);
    if !user_tier.has_access(&UserTier::Premium) {
        return Err(
            AppError::forbidden("Premium tier required for analytics").into_server_fn_error()
        );
    }

    let db = get_db().await;

    let total_revenue = sqlx::query_scalar!(
        "SELECT COALESCE(SUM(price), 0.0) FROM products WHERE status = 'active'"
    )
    .fetch_one(db)
    .await
    .map_err(|e| e.into_app_error().into_server_fn_error())?
    .unwrap_or(0.0);

    let avg_product_price = sqlx::query_scalar!("SELECT COALESCE(AVG(price), 0.0) FROM products")
        .fetch_one(db)
        .await
        .map_err(|e| e.into_app_error().into_server_fn_error())?
        .unwrap_or(0.0);

    let category_rows = sqlx::query!(
        "SELECT category, COUNT(*) as count FROM products GROUP BY category ORDER BY count DESC"
    )
    .fetch_all(db)
    .await
    .map_err(|e| e.into_app_error().into_server_fn_error())?;

    let products_by_category: Vec<shared_types::CategoryCount> = category_rows
        .into_iter()
        .map(|r| shared_types::CategoryCount {
            category: r.category,
            count: r.count.unwrap_or(0),
        })
        .collect();

    let users_last_30_days = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM users WHERE created_at >= NOW() - INTERVAL '30 days'"
    )
    .fetch_one(db)
    .await
    .map_err(|e| e.into_app_error().into_server_fn_error())?
    .unwrap_or(0);

    Ok(shared_types::PremiumAnalytics {
        total_revenue,
        avg_product_price,
        products_by_category,
        users_last_30_days,
    })
}

/// Get a user by ID.
#[cfg_attr(feature = "server", tracing::instrument)]
#[server]
pub async fn get_user(user_id: i64) -> Result<User, ServerFnError> {
    let db = get_db().await;
    let user = sqlx::query_as!(
        User,
        "SELECT id, username, display_name, role, tier FROM users WHERE id = $1",
        user_id
    )
    .fetch_optional(db)
    .await
    .map_err(|e| e.into_app_error().into_server_fn_error())?
    .ok_or_else(|| {
        shared_types::AppError::not_found(format!("User with id {} not found", user_id))
            .into_server_fn_error()
    })?;
    Ok(user)
}

/// List all users.
#[cfg_attr(feature = "server", tracing::instrument)]
#[server]
pub async fn list_users() -> Result<Vec<User>, ServerFnError> {
    let db = get_db().await;
    let users = sqlx::query_as!(
        User,
        "SELECT id, username, display_name, role, tier FROM users"
    )
    .fetch_all(db)
    .await
    .map_err(|e| e.into_app_error().into_server_fn_error())?;
    Ok(users)
}

/// Create a new user.
#[cfg_attr(feature = "server", tracing::instrument)]
#[server]
pub async fn create_user(username: String, display_name: String) -> Result<User, ServerFnError> {
    let req = CreateUserRequest {
        username,
        display_name,
    };
    req.validate_request()
        .map_err(|e| e.into_server_fn_error())?;

    let db = get_db().await;
    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (username, display_name) VALUES ($1, $2) RETURNING id, username, display_name, role, tier",
        req.username,
        req.display_name
    )
    .fetch_one(db)
    .await
    .map_err(|e| e.into_app_error().into_server_fn_error())?;

    Ok(user)
}

/// Update an existing user.
#[cfg_attr(feature = "server", tracing::instrument)]
#[server]
pub async fn update_user(
    user_id: i64,
    username: String,
    display_name: String,
) -> Result<User, ServerFnError> {
    let req = UpdateUserRequest {
        username,
        display_name,
    };
    req.validate_request()
        .map_err(|e| e.into_server_fn_error())?;

    let db = get_db().await;
    let user = sqlx::query_as!(
        User,
        "UPDATE users SET username = $2, display_name = $3 WHERE id = $1 RETURNING id, username, display_name, role, tier",
        user_id,
        req.username,
        req.display_name
    )
    .fetch_one(db)
    .await
    .map_err(|e| e.into_app_error().into_server_fn_error())?;
    Ok(user)
}

/// Delete a user by ID.
#[cfg_attr(feature = "server", tracing::instrument)]
#[server]
pub async fn delete_user(user_id: i64) -> Result<(), ServerFnError> {
    let db = get_db().await;
    sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
        .execute(db)
        .await
        .map_err(|e| e.into_app_error().into_server_fn_error())?;
    Ok(())
}

/// Update a user's tier. Requires admin role (verified via JWT).
#[cfg_attr(feature = "server", tracing::instrument)]
#[server]
pub async fn update_user_tier(user_id: i64, tier: String) -> Result<User, ServerFnError> {
    use crate::auth::{cookies, jwt};
    use shared_types::AppError;

    // Validate tier value
    let valid_tiers = ["free", "premium", "elite"];
    let tier_lower = tier.to_lowercase();
    if !valid_tiers.contains(&tier_lower.as_str()) {
        return Err(
            AppError::validation("Invalid tier value", Default::default()).into_server_fn_error(),
        );
    }

    // Extract and validate admin role from JWT
    let ctx = dioxus::fullstack::FullstackContext::current();
    let headers = ctx.as_ref().map(|c| c.parts_mut().headers.clone());

    let headers = headers
        .ok_or_else(|| AppError::unauthorized("Authentication required").into_server_fn_error())?;

    let token = cookies::extract_access_token(&headers)
        .ok_or_else(|| AppError::unauthorized("Authentication required").into_server_fn_error())?;

    let claims = jwt::validate_access_token(&token)
        .map_err(|_| AppError::unauthorized("Invalid token").into_server_fn_error())?;

    if claims.role != "admin" {
        return Err(
            AppError::forbidden("Admin role required to change user tiers").into_server_fn_error(),
        );
    }

    let db = get_db().await;
    let user = sqlx::query_as!(
        User,
        "UPDATE users SET tier = $2 WHERE id = $1 RETURNING id, username, display_name, role, tier",
        user_id,
        tier_lower
    )
    .fetch_one(db)
    .await
    .map_err(|e| e.into_app_error().into_server_fn_error())?;

    Ok(user)
}

/// List all products.
#[cfg_attr(feature = "server", tracing::instrument)]
#[server]
pub async fn list_products() -> Result<Vec<Product>, ServerFnError> {
    let db = get_db().await;
    let rows = sqlx::query!(
        "SELECT id, name, description, price, category, status, created_at FROM products ORDER BY id DESC"
    )
    .fetch_all(db)
    .await
    .map_err(|e| e.into_app_error().into_server_fn_error())?;

    let products = rows
        .into_iter()
        .map(|r| Product {
            id: r.id,
            name: r.name,
            description: r.description,
            price: r.price,
            category: r.category,
            status: r.status,
            created_at: r.created_at.to_string(),
        })
        .collect();
    Ok(products)
}

/// Create a new product.
#[cfg_attr(feature = "server", tracing::instrument)]
#[server]
pub async fn create_product(
    name: String,
    description: String,
    price: f64,
    category: String,
    status: String,
) -> Result<Product, ServerFnError> {
    let req = CreateProductRequest {
        name,
        description,
        price,
        category,
        status,
    };
    req.validate_request()
        .map_err(|e| e.into_server_fn_error())?;

    let db = get_db().await;
    let row = sqlx::query!(
        "INSERT INTO products (name, description, price, category, status) VALUES ($1, $2, $3, $4, $5) RETURNING id, name, description, price, category, status, created_at",
        req.name,
        req.description,
        req.price,
        req.category,
        req.status
    )
    .fetch_one(db)
    .await
    .map_err(|e| e.into_app_error().into_server_fn_error())?;

    Ok(Product {
        id: row.id,
        name: row.name,
        description: row.description,
        price: row.price,
        category: row.category,
        status: row.status,
        created_at: row.created_at.to_string(),
    })
}

/// Update an existing product.
#[cfg_attr(feature = "server", tracing::instrument)]
#[server]
pub async fn update_product(
    product_id: i64,
    name: String,
    description: String,
    price: f64,
    category: String,
    status: String,
) -> Result<Product, ServerFnError> {
    let req = UpdateProductRequest {
        name,
        description,
        price,
        category,
        status,
    };
    req.validate_request()
        .map_err(|e| e.into_server_fn_error())?;

    let db = get_db().await;
    let row = sqlx::query!(
        "UPDATE products SET name = $2, description = $3, price = $4, category = $5, status = $6 WHERE id = $1 RETURNING id, name, description, price, category, status, created_at",
        product_id,
        req.name,
        req.description,
        req.price,
        req.category,
        req.status
    )
    .fetch_one(db)
    .await
    .map_err(|e| e.into_app_error().into_server_fn_error())?;

    Ok(Product {
        id: row.id,
        name: row.name,
        description: row.description,
        price: row.price,
        category: row.category,
        status: row.status,
        created_at: row.created_at.to_string(),
    })
}

/// Delete a product by ID.
#[cfg_attr(feature = "server", tracing::instrument)]
#[server]
pub async fn delete_product(product_id: i64) -> Result<(), ServerFnError> {
    let db = get_db().await;
    sqlx::query!("DELETE FROM products WHERE id = $1", product_id)
        .execute(db)
        .await
        .map_err(|e| e.into_app_error().into_server_fn_error())?;
    Ok(())
}

/// Get dashboard statistics.
#[cfg_attr(feature = "server", tracing::instrument)]
#[server]
pub async fn get_dashboard_stats() -> Result<DashboardStats, ServerFnError> {
    let db = get_db().await;

    let user_count = sqlx::query_scalar!("SELECT COUNT(*) FROM users")
        .fetch_one(db)
        .await
        .map_err(|e| e.into_app_error().into_server_fn_error())?
        .unwrap_or(0);

    let product_count = sqlx::query_scalar!("SELECT COUNT(*) FROM products")
        .fetch_one(db)
        .await
        .map_err(|e| e.into_app_error().into_server_fn_error())?
        .unwrap_or(0);

    let active_count = sqlx::query_scalar!("SELECT COUNT(*) FROM products WHERE status = 'active'")
        .fetch_one(db)
        .await
        .map_err(|e| e.into_app_error().into_server_fn_error())?
        .unwrap_or(0);

    let recent_users = sqlx::query_as!(
        User,
        "SELECT id, username, display_name, role, tier FROM users ORDER BY id DESC LIMIT 5"
    )
    .fetch_all(db)
    .await
    .map_err(|e| e.into_app_error().into_server_fn_error())?;

    Ok(DashboardStats {
        total_users: user_count,
        total_products: product_count,
        active_products: active_count,
        recent_users,
    })
}

/// Register a new user. Sets HTTP-only auth cookies on success.
#[cfg_attr(feature = "server", tracing::instrument(skip(password)))]
#[server]
pub async fn register(
    username: String,
    email: String,
    password: String,
    display_name: String,
) -> Result<AuthUser, ServerFnError> {
    use crate::auth::{cookies, jwt, password as pw};
    use shared_types::{AppError, RegisterRequest};

    let req = RegisterRequest {
        username: username.clone(),
        email: email.clone(),
        password: password.clone(),
        display_name: display_name.clone(),
    };
    req.validate_request()
        .map_err(|e| e.into_server_fn_error())?;

    let password_hash = pw::hash_password(&password)
        .map_err(|e| AppError::internal(e.to_string()).into_server_fn_error())?;

    let db = get_db().await;
    let user = sqlx::query!(
        "INSERT INTO users (username, email, password_hash, display_name) VALUES ($1, $2, $3, $4) RETURNING id, username, display_name, email, role, tier, avatar_url",
        username,
        email,
        password_hash,
        display_name
    )
    .fetch_one(db)
    .await
    .map_err(|e| e.into_app_error().into_server_fn_error())?;

    let user_email = user.email.unwrap_or_default();
    let user_role = user.role;
    let user_tier = UserTier::from_str_or_default(&user.tier);

    let access_token =
        jwt::create_access_token(user.id, &user_email, &user_role, user_tier.as_str())
            .map_err(|e| AppError::internal(e.to_string()).into_server_fn_error())?;

    let (refresh_token, expires_at) =
        jwt::create_refresh_token(user.id, &user_email, &user_role, user_tier.as_str())
            .map_err(|e| AppError::internal(e.to_string()).into_server_fn_error())?;

    // Store refresh token for later validation
    sqlx::query!(
        "INSERT INTO refresh_tokens (user_id, token_hash, expires_at) VALUES ($1, $2, $3)",
        user.id,
        refresh_token,
        expires_at
    )
    .execute(db)
    .await
    .map_err(|e| e.into_app_error().into_server_fn_error())?;

    // Schedule cookies to be set by the middleware
    cookies::schedule_auth_cookies(&access_token, &refresh_token);

    Ok(AuthUser {
        id: user.id,
        username: user.username,
        display_name: user.display_name,
        email: user_email,
        role: user_role,
        tier: user_tier,
        avatar_url: user.avatar_url,
    })
}

/// Login with email and password. Sets HTTP-only auth cookies on success.
#[cfg_attr(feature = "server", tracing::instrument(skip(password)))]
#[server]
pub async fn login(email: String, password: String) -> Result<AuthUser, ServerFnError> {
    use crate::auth::{cookies, jwt, password as pw};
    use shared_types::{AppError, LoginRequest};

    let req = LoginRequest {
        email: email.clone(),
        password: password.clone(),
    };
    req.validate_request()
        .map_err(|e| e.into_server_fn_error())?;

    let db = get_db().await;
    let user = sqlx::query!(
        "SELECT id, username, display_name, email, password_hash, role, tier, avatar_url FROM users WHERE email = $1",
        email
    )
    .fetch_optional(db)
    .await
    .map_err(|e| e.into_app_error().into_server_fn_error())?
    .ok_or_else(|| AppError::unauthorized("Invalid email or password").into_server_fn_error())?;

    let password_hash = user.password_hash.ok_or_else(|| {
        AppError::unauthorized("Invalid email or password").into_server_fn_error()
    })?;

    let valid = pw::verify_password(&password, &password_hash)
        .map_err(|e| AppError::internal(e.to_string()).into_server_fn_error())?;

    if !valid {
        return Err(AppError::unauthorized("Invalid email or password").into_server_fn_error());
    }

    let user_email = user.email.unwrap_or_default();
    let user_role = user.role;
    let user_tier = UserTier::from_str_or_default(&user.tier);

    let access_token =
        jwt::create_access_token(user.id, &user_email, &user_role, user_tier.as_str())
            .map_err(|e| AppError::internal(e.to_string()).into_server_fn_error())?;

    let (refresh_token, expires_at) =
        jwt::create_refresh_token(user.id, &user_email, &user_role, user_tier.as_str())
            .map_err(|e| AppError::internal(e.to_string()).into_server_fn_error())?;

    sqlx::query!(
        "INSERT INTO refresh_tokens (user_id, token_hash, expires_at) VALUES ($1, $2, $3)",
        user.id,
        refresh_token,
        expires_at
    )
    .execute(db)
    .await
    .map_err(|e| e.into_app_error().into_server_fn_error())?;

    // Schedule cookies to be set by the middleware
    cookies::schedule_auth_cookies(&access_token, &refresh_token);

    Ok(AuthUser {
        id: user.id,
        username: user.username,
        display_name: user.display_name,
        email: user_email,
        role: user_role,
        tier: user_tier,
        avatar_url: user.avatar_url,
    })
}

/// Get the current authenticated user from cookies. Returns None if not authenticated.
#[cfg_attr(feature = "server", tracing::instrument)]
#[server]
pub async fn get_current_user() -> Result<Option<AuthUser>, ServerFnError> {
    use crate::auth::{cookies, jwt};

    let ctx = dioxus::fullstack::FullstackContext::current();
    let headers = ctx.as_ref().map(|c| {
        let parts = c.parts_mut();
        parts.headers.clone()
    });

    let headers = match headers {
        Some(h) => h,
        None => return Ok(None),
    };

    let token = match cookies::extract_access_token(&headers) {
        Some(t) => t,
        None => return Ok(None),
    };

    let claims = match jwt::validate_access_token(&token) {
        Ok(c) => c,
        Err(_) => return Ok(None),
    };

    let db = get_db().await;
    let user = sqlx::query!(
        "SELECT id, username, display_name, email, role, tier, avatar_url FROM users WHERE id = $1",
        claims.sub
    )
    .fetch_optional(db)
    .await
    .map_err(|e| e.into_app_error().into_server_fn_error())?;

    match user {
        Some(u) => Ok(Some(AuthUser {
            id: u.id,
            username: u.username,
            display_name: u.display_name,
            email: u.email.unwrap_or_default(),
            role: u.role,
            tier: UserTier::from_str_or_default(&u.tier),
            avatar_url: u.avatar_url,
        })),
        None => Ok(None),
    }
}

/// Logout by revoking all refresh tokens and clearing auth cookies.
#[cfg_attr(feature = "server", tracing::instrument)]
#[server]
pub async fn logout() -> Result<(), ServerFnError> {
    use crate::auth::{cookies, jwt};

    if let Some(ctx) = dioxus::fullstack::FullstackContext::current() {
        let headers = ctx.parts_mut().headers.clone();
        if let Some(token) = cookies::extract_access_token(&headers) {
            if let Ok(claims) = jwt::validate_access_token(&token) {
                let db = get_db().await;
                let _ = sqlx::query!(
                    "UPDATE refresh_tokens SET revoked = TRUE WHERE user_id = $1 AND revoked = FALSE",
                    claims.sub
                )
                .execute(db)
                .await;
            }
        }
    }

    // Schedule cookie clearing via middleware
    cookies::schedule_clear_cookies();

    Ok(())
}

/// Update the current user's profile (display name and email).
#[cfg_attr(feature = "server", tracing::instrument)]
#[server]
pub async fn update_profile(
    display_name: String,
    email: String,
) -> Result<AuthUser, ServerFnError> {
    use crate::auth::{cookies, jwt};
    use shared_types::{AppError, UpdateProfileRequest};

    // Validate the request
    let req = UpdateProfileRequest {
        display_name: display_name.clone(),
        email: email.clone(),
    };
    req.validate_request()
        .map_err(|e| e.into_server_fn_error())?;

    // Extract user ID from JWT
    let ctx = dioxus::fullstack::FullstackContext::current();
    let headers = ctx.as_ref().map(|c| c.parts_mut().headers.clone());

    let headers = headers
        .ok_or_else(|| AppError::unauthorized("Authentication required").into_server_fn_error())?;

    let token = cookies::extract_access_token(&headers)
        .ok_or_else(|| AppError::unauthorized("Authentication required").into_server_fn_error())?;

    let claims = jwt::validate_access_token(&token)
        .map_err(|_| AppError::unauthorized("Invalid token").into_server_fn_error())?;

    let db = get_db().await;
    let user = sqlx::query!(
        "UPDATE users SET display_name = $2, email = $3 WHERE id = $1 RETURNING id, username, display_name, email, role, tier, avatar_url",
        claims.sub,
        display_name,
        email
    )
    .fetch_optional(db)
    .await
    .map_err(|e| e.into_app_error().into_server_fn_error())?
    .ok_or_else(|| AppError::not_found("User not found").into_server_fn_error())?;

    Ok(AuthUser {
        id: user.id,
        username: user.username,
        display_name: user.display_name,
        email: user.email.unwrap_or_default(),
        role: user.role,
        tier: UserTier::from_str_or_default(&user.tier),
        avatar_url: user.avatar_url,
    })
}

/// Upload a user avatar via base64-encoded file data.
#[cfg_attr(feature = "server", tracing::instrument(skip(file_data)))]
#[server]
pub async fn upload_user_avatar(
    file_data: String,
    content_type: String,
) -> Result<AuthUser, ServerFnError> {
    use crate::auth::{cookies, jwt};
    use shared_types::AppError;

    let allowed = ["image/jpeg", "image/png", "image/webp"];
    if !allowed.contains(&content_type.as_str()) {
        return Err(AppError::validation(
            "Only JPEG, PNG, and WebP images are allowed",
            Default::default(),
        )
        .into_server_fn_error());
    }

    let bytes = base64::Engine::decode(
        &base64::engine::general_purpose::STANDARD,
        &file_data,
    )
    .map_err(|e| {
        AppError::validation(format!("Invalid file data: {}", e), Default::default())
            .into_server_fn_error()
    })?;

    if bytes.len() > 2 * 1024 * 1024 {
        return Err(
            AppError::validation("Avatar must be under 2 MB", Default::default())
                .into_server_fn_error(),
        );
    }

    let ctx = dioxus::fullstack::FullstackContext::current();
    let headers = ctx
        .as_ref()
        .map(|c| c.parts_mut().headers.clone())
        .ok_or_else(|| AppError::unauthorized("Authentication required").into_server_fn_error())?;

    let token = cookies::extract_access_token(&headers)
        .ok_or_else(|| AppError::unauthorized("Authentication required").into_server_fn_error())?;

    let claims = jwt::validate_access_token(&token)
        .map_err(|_| AppError::unauthorized("Invalid token").into_server_fn_error())?;

    let avatar_url = crate::s3::upload_avatar(claims.sub, &content_type, &bytes)
        .await
        .map_err(|e| AppError::internal(e).into_server_fn_error())?;

    let db = get_db().await;
    let user = sqlx::query!(
        "UPDATE users SET avatar_url = $2 WHERE id = $1 RETURNING id, username, display_name, email, role, tier, avatar_url",
        claims.sub,
        avatar_url
    )
    .fetch_one(db)
    .await
    .map_err(|e| e.into_app_error().into_server_fn_error())?;

    Ok(AuthUser {
        id: user.id,
        username: user.username,
        display_name: user.display_name,
        email: user.email.unwrap_or_default(),
        role: user.role,
        tier: UserTier::from_str_or_default(&user.tier),
        avatar_url: user.avatar_url,
    })
}

/// Get the OAuth authorization URL for a given provider.
#[cfg_attr(feature = "server", tracing::instrument)]
#[server]
pub async fn oauth_authorize_url(provider: String) -> Result<String, ServerFnError> {
    use crate::auth::oauth;
    use shared_types::AppError;

    let provider = shared_types::OAuthProvider::parse_provider(&provider).ok_or_else(|| {
        AppError::validation("Unsupported OAuth provider", Default::default())
            .into_server_fn_error()
    })?;

    let url = oauth::get_authorize_url(&provider)
        .await
        .map_err(|e| AppError::internal(e).into_server_fn_error())?;

    Ok(url)
}
