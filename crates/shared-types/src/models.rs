use serde::{Deserialize, Serialize};

/// A user in the system.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct User {
    pub id: i64,
    pub username: String,
    pub display_name: String,
}

/// A product available in the catalog.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct Product {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub category: String,
    pub status: String,
    pub created_at: String,
}

/// Aggregated dashboard statistics.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct DashboardStats {
    pub total_users: i64,
    pub total_products: i64,
    pub active_products: i64,
    pub recent_users: Vec<User>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_serialization_roundtrip() {
        let user = User {
            id: 1,
            username: "tharpool".into(),
            display_name: "Tyler".into(),
        };

        let json = serde_json::to_string(&user).unwrap();
        let deserialized: User = serde_json::from_str(&json).unwrap();

        assert_eq!(user, deserialized);
    }

    #[test]
    fn user_deserializes_from_api_json() {
        let json = r#"{"id": 42, "username": "demo", "display_name": "Demo User"}"#;
        let user: User = serde_json::from_str(json).unwrap();

        assert_eq!(user.id, 42);
        assert_eq!(user.username, "demo");
    }

    #[test]
    fn product_serialization_roundtrip() {
        let product = Product {
            id: 1,
            name: "Widget".into(),
            description: "A test widget".into(),
            price: 29.99,
            category: "Hardware".into(),
            status: "active".into(),
            created_at: "2025-01-01T00:00:00Z".into(),
        };

        let json = serde_json::to_string(&product).unwrap();
        let deserialized: Product = serde_json::from_str(&json).unwrap();

        assert_eq!(product, deserialized);
    }
}
