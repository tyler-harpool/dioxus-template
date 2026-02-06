use dioxus::prelude::*;
use shared_types::User;

#[cfg(feature = "server")]
use crate::db::get_db;

/// Get a user by ID.
#[cfg_attr(
    feature = "server",
    utoipa::path(
        get,
        path = "/api/users/{user_id}",
        params(
            ("user_id" = i64, Path, description = "User ID")
        ),
        responses(
            (status = 200, description = "User found", body = User),
            (status = 404, description = "User not found")
        ),
        tag = "users"
    )
)]
#[server]
pub async fn get_user(user_id: i64) -> Result<User, ServerFnError> {
    let db = get_db().await;
    let user = sqlx::query_as!(
        User,
        "SELECT id, username, display_name FROM users WHERE id = ?",
        user_id
    )
    .fetch_one(db)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(user)
}

/// List all users.
#[cfg_attr(
    feature = "server",
    utoipa::path(
        get,
        path = "/api/users",
        responses(
            (status = 200, description = "List of users", body = Vec<User>)
        ),
        tag = "users"
    )
)]
#[server]
pub async fn list_users() -> Result<Vec<User>, ServerFnError> {
    let db = get_db().await;
    let users = sqlx::query_as!(User, "SELECT id, username, display_name FROM users")
        .fetch_all(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(users)
}

/// Create a new user.
#[cfg_attr(
    feature = "server",
    utoipa::path(
        post,
        path = "/api/users",
        responses(
            (status = 201, description = "User created", body = User)
        ),
        tag = "users"
    )
)]
#[server]
pub async fn create_user(username: String, display_name: String) -> Result<User, ServerFnError> {
    let db = get_db().await;
    let id = sqlx::query("INSERT INTO users (username, display_name) VALUES (?, ?)")
        .bind(&username)
        .bind(&display_name)
        .execute(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .last_insert_rowid();

    Ok(User {
        id,
        username,
        display_name,
    })
}
