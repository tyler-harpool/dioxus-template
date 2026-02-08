use serde::{Deserialize, Serialize};

/// User subscription tier controlling feature access.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub enum UserTier {
    #[default]
    Free,
    Premium,
    Elite,
}

impl UserTier {
    /// Numeric rank for tier comparison.
    fn rank(&self) -> u8 {
        match self {
            UserTier::Free => 0,
            UserTier::Premium => 1,
            UserTier::Elite => 2,
        }
    }

    /// Check if this tier grants access to a feature requiring `required` tier.
    pub fn has_access(&self, required: &UserTier) -> bool {
        self.rank() >= required.rank()
    }

    /// Parse a tier string, defaulting to Free for unknown values.
    pub fn from_str_or_default(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "premium" => UserTier::Premium,
            "elite" => UserTier::Elite,
            _ => UserTier::Free,
        }
    }

    /// Serialize to lowercase string for database storage.
    pub fn as_str(&self) -> &'static str {
        match self {
            UserTier::Free => "free",
            UserTier::Premium => "premium",
            UserTier::Elite => "elite",
        }
    }
}

/// Supported OAuth identity providers.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub enum OAuthProvider {
    Google,
    GitHub,
}

impl OAuthProvider {
    pub fn as_str(&self) -> &'static str {
        match self {
            OAuthProvider::Google => "google",
            OAuthProvider::GitHub => "github",
        }
    }

    pub fn parse_provider(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "google" => Some(OAuthProvider::Google),
            "github" => Some(OAuthProvider::GitHub),
            _ => None,
        }
    }
}

/// Parameters received from an OAuth callback redirect.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct OAuthCallbackParams {
    pub code: String,
    pub state: String,
}

/// A user in the system.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct User {
    pub id: i64,
    pub username: String,
    pub display_name: String,
    pub role: String,
    pub tier: String,
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

/// Login request.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "validation", derive(validator::Validate))]
pub struct LoginRequest {
    #[cfg_attr(
        feature = "validation",
        validate(email(message = "Valid email is required"))
    )]
    pub email: String,
    #[cfg_attr(
        feature = "validation",
        validate(length(min = 8, message = "Password must be at least 8 characters"))
    )]
    pub password: String,
}

/// Register request.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "validation", derive(validator::Validate))]
pub struct RegisterRequest {
    #[cfg_attr(
        feature = "validation",
        validate(length(min = 3, message = "Username must be at least 3 characters"))
    )]
    pub username: String,
    #[cfg_attr(
        feature = "validation",
        validate(email(message = "Valid email is required"))
    )]
    pub email: String,
    #[cfg_attr(
        feature = "validation",
        validate(length(min = 8, message = "Password must be at least 8 characters"))
    )]
    pub password: String,
    #[cfg_attr(
        feature = "validation",
        validate(length(min = 1, message = "Display name is required"))
    )]
    pub display_name: String,
}

/// Authenticated user info (safe to send to client).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct AuthUser {
    pub id: i64,
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub role: String,
    pub tier: UserTier,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
}

/// Premium analytics data returned by the tier-gated endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct PremiumAnalytics {
    pub total_revenue: f64,
    pub avg_product_price: f64,
    pub products_by_category: Vec<CategoryCount>,
    pub users_last_30_days: i64,
}

/// Category name with a count of products in that category.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct CategoryCount {
    pub category: String,
    pub count: i64,
}

/// Refresh token request (used by REST/OpenAPI).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct RefreshRequest {
    pub refresh_token: String,
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
            role: "user".into(),
            tier: "free".into(),
        };

        let json = serde_json::to_string(&user).unwrap();
        let deserialized: User = serde_json::from_str(&json).unwrap();

        assert_eq!(user, deserialized);
    }

    #[test]
    fn user_deserializes_from_api_json() {
        let json = r#"{"id": 42, "username": "demo", "display_name": "Demo User", "role": "admin", "tier": "premium"}"#;
        let user: User = serde_json::from_str(json).unwrap();

        assert_eq!(user.id, 42);
        assert_eq!(user.username, "demo");
        assert_eq!(user.role, "admin");
        assert_eq!(user.tier, "premium");
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

    #[test]
    fn user_tier_has_access_same_tier() {
        assert!(UserTier::Free.has_access(&UserTier::Free));
        assert!(UserTier::Premium.has_access(&UserTier::Premium));
        assert!(UserTier::Elite.has_access(&UserTier::Elite));
    }

    #[test]
    fn user_tier_has_access_higher_tier() {
        assert!(UserTier::Premium.has_access(&UserTier::Free));
        assert!(UserTier::Elite.has_access(&UserTier::Free));
        assert!(UserTier::Elite.has_access(&UserTier::Premium));
    }

    #[test]
    fn user_tier_denies_lower_tier() {
        assert!(!UserTier::Free.has_access(&UserTier::Premium));
        assert!(!UserTier::Free.has_access(&UserTier::Elite));
        assert!(!UserTier::Premium.has_access(&UserTier::Elite));
    }

    #[test]
    fn user_tier_from_str_or_default_known_values() {
        assert_eq!(UserTier::from_str_or_default("premium"), UserTier::Premium);
        assert_eq!(UserTier::from_str_or_default("Premium"), UserTier::Premium);
        assert_eq!(UserTier::from_str_or_default("PREMIUM"), UserTier::Premium);
        assert_eq!(UserTier::from_str_or_default("elite"), UserTier::Elite);
        assert_eq!(UserTier::from_str_or_default("Elite"), UserTier::Elite);
        assert_eq!(UserTier::from_str_or_default("free"), UserTier::Free);
    }

    #[test]
    fn user_tier_from_str_or_default_unknown_falls_to_free() {
        assert_eq!(UserTier::from_str_or_default(""), UserTier::Free);
        assert_eq!(UserTier::from_str_or_default("gold"), UserTier::Free);
        assert_eq!(UserTier::from_str_or_default("invalid"), UserTier::Free);
    }

    #[test]
    fn user_tier_as_str_roundtrip() {
        for tier in [UserTier::Free, UserTier::Premium, UserTier::Elite] {
            let s = tier.as_str();
            let parsed = UserTier::from_str_or_default(s);
            assert_eq!(tier, parsed);
        }
    }

    #[test]
    fn oauth_provider_parse_valid() {
        assert_eq!(
            OAuthProvider::parse_provider("google"),
            Some(OAuthProvider::Google)
        );
        assert_eq!(
            OAuthProvider::parse_provider("Google"),
            Some(OAuthProvider::Google)
        );
        assert_eq!(
            OAuthProvider::parse_provider("github"),
            Some(OAuthProvider::GitHub)
        );
        assert_eq!(
            OAuthProvider::parse_provider("GitHub"),
            Some(OAuthProvider::GitHub)
        );
    }

    #[test]
    fn oauth_provider_parse_invalid_returns_none() {
        assert_eq!(OAuthProvider::parse_provider("facebook"), None);
        assert_eq!(OAuthProvider::parse_provider(""), None);
        assert_eq!(OAuthProvider::parse_provider("twitter"), None);
    }
}
