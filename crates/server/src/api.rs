use dioxus::prelude::*;
use shared_types::{DashboardStats, Product, User};

#[cfg(feature = "server")]
use crate::db::get_db;

/// Get a user by ID.
#[server]
pub async fn get_user(user_id: i64) -> Result<User, ServerFnError> {
    let db = get_db().await;
    let user = sqlx::query_as!(
        User,
        "SELECT id, username, display_name FROM users WHERE id = $1",
        user_id
    )
    .fetch_one(db)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(user)
}

/// List all users.
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
#[server]
pub async fn create_user(username: String, display_name: String) -> Result<User, ServerFnError> {
    let db = get_db().await;
    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (username, display_name) VALUES ($1, $2) RETURNING id, username, display_name",
        username,
        display_name
    )
    .fetch_one(db)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(user)
}

/// Update an existing user.
#[server]
pub async fn update_user(
    user_id: i64,
    username: String,
    display_name: String,
) -> Result<User, ServerFnError> {
    let db = get_db().await;
    let user = sqlx::query_as!(
        User,
        "UPDATE users SET username = $2, display_name = $3 WHERE id = $1 RETURNING id, username, display_name",
        user_id,
        username,
        display_name
    )
    .fetch_one(db)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(user)
}

/// Delete a user by ID.
#[server]
pub async fn delete_user(user_id: i64) -> Result<(), ServerFnError> {
    let db = get_db().await;
    sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
        .execute(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(())
}

/// List all products.
#[server]
pub async fn list_products() -> Result<Vec<Product>, ServerFnError> {
    let db = get_db().await;
    let rows = sqlx::query!(
        "SELECT id, name, description, price, category, status, created_at FROM products ORDER BY id DESC"
    )
    .fetch_all(db)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

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
#[server]
pub async fn create_product(
    name: String,
    description: String,
    price: f64,
    category: String,
    status: String,
) -> Result<Product, ServerFnError> {
    let db = get_db().await;
    let row = sqlx::query!(
        "INSERT INTO products (name, description, price, category, status) VALUES ($1, $2, $3, $4, $5) RETURNING id, name, description, price, category, status, created_at",
        name,
        description,
        price,
        category,
        status
    )
    .fetch_one(db)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

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
#[server]
pub async fn update_product(
    product_id: i64,
    name: String,
    description: String,
    price: f64,
    category: String,
    status: String,
) -> Result<Product, ServerFnError> {
    let db = get_db().await;
    let row = sqlx::query!(
        "UPDATE products SET name = $2, description = $3, price = $4, category = $5, status = $6 WHERE id = $1 RETURNING id, name, description, price, category, status, created_at",
        product_id,
        name,
        description,
        price,
        category,
        status
    )
    .fetch_one(db)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

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
#[server]
pub async fn delete_product(product_id: i64) -> Result<(), ServerFnError> {
    let db = get_db().await;
    sqlx::query!("DELETE FROM products WHERE id = $1", product_id)
        .execute(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(())
}

/// Get dashboard statistics.
#[server]
pub async fn get_dashboard_stats() -> Result<DashboardStats, ServerFnError> {
    let db = get_db().await;

    let user_count = sqlx::query_scalar!("SELECT COUNT(*) FROM users")
        .fetch_one(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .unwrap_or(0);

    let product_count = sqlx::query_scalar!("SELECT COUNT(*) FROM products")
        .fetch_one(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .unwrap_or(0);

    let active_count = sqlx::query_scalar!("SELECT COUNT(*) FROM products WHERE status = 'active'")
        .fetch_one(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .unwrap_or(0);

    let recent_users = sqlx::query_as!(
        User,
        "SELECT id, username, display_name FROM users ORDER BY id DESC LIMIT 5"
    )
    .fetch_all(db)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(DashboardStats {
        total_users: user_count,
        total_products: product_count,
        active_products: active_count,
        recent_users,
    })
}
