use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

/// JWT claims stored in access and refresh tokens.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub email: String,
    pub role: String,
    pub tier: String,
    pub exp: i64,
    pub iat: i64,
    /// Unique token identifier — prevents hash collisions when multiple
    /// tokens are issued for the same user within the same second.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jti: Option<String>,
}

fn jwt_secret() -> String {
    std::env::var("JWT_SECRET").expect("JWT_SECRET must be set")
}

pub fn access_token_expiry_minutes() -> i64 {
    std::env::var("JWT_ACCESS_TOKEN_EXPIRY_MINUTES")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(15)
}

pub fn refresh_token_expiry_days() -> i64 {
    std::env::var("JWT_REFRESH_TOKEN_EXPIRY_DAYS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(7)
}

pub fn create_access_token(
    user_id: i64,
    email: &str,
    role: &str,
    tier: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let claims = Claims {
        sub: user_id,
        email: email.to_string(),
        role: role.to_string(),
        tier: tier.to_string(),
        iat: now.timestamp(),
        exp: (now + Duration::minutes(access_token_expiry_minutes())).timestamp(),
        jti: Some(uuid::Uuid::new_v4().to_string()),
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret().as_bytes()),
    )
}

pub fn create_refresh_token(
    user_id: i64,
    email: &str,
    role: &str,
    tier: &str,
) -> Result<(String, chrono::DateTime<Utc>), jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let expires_at = now + Duration::days(refresh_token_expiry_days());
    let claims = Claims {
        sub: user_id,
        email: email.to_string(),
        role: role.to_string(),
        tier: tier.to_string(),
        iat: now.timestamp(),
        exp: expires_at.timestamp(),
        jti: Some(uuid::Uuid::new_v4().to_string()),
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret().as_bytes()),
    )?;
    Ok((token, expires_at))
}

pub fn validate_access_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret().as_bytes()),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_test_secret() {
        std::env::set_var("JWT_SECRET", "test-secret-key-for-jwt-unit-tests");
    }

    #[test]
    fn create_and_validate_access_token() {
        setup_test_secret();
        let token = create_access_token(42, "test@example.com", "user", "free").unwrap();
        let claims = validate_access_token(&token).unwrap();
        assert_eq!(claims.sub, 42);
        assert_eq!(claims.email, "test@example.com");
        assert_eq!(claims.role, "user");
        assert_eq!(claims.tier, "free");
    }

    #[test]
    fn expired_token_rejected() {
        setup_test_secret();
        let now = Utc::now();
        let claims = Claims {
            sub: 1,
            email: "expired@test.com".to_string(),
            role: "user".to_string(),
            tier: "free".to_string(),
            iat: (now - Duration::hours(2)).timestamp(),
            exp: (now - Duration::hours(1)).timestamp(),
            jti: None,
        };
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(jwt_secret().as_bytes()),
        )
        .unwrap();

        assert!(validate_access_token(&token).is_err());
    }

    #[test]
    fn invalid_token_rejected() {
        setup_test_secret();
        assert!(validate_access_token("not.a.valid.jwt").is_err());
        assert!(validate_access_token("").is_err());
    }

    #[test]
    fn claims_contain_correct_fields() {
        setup_test_secret();
        let token = create_access_token(99, "admin@co.com", "admin", "elite").unwrap();
        let claims = validate_access_token(&token).unwrap();
        assert_eq!(claims.sub, 99);
        assert_eq!(claims.email, "admin@co.com");
        assert_eq!(claims.role, "admin");
        assert_eq!(claims.tier, "elite");
        assert!(claims.exp > claims.iat);
    }

    #[test]
    fn refresh_token_has_later_expiry() {
        setup_test_secret();
        let access = create_access_token(1, "a@b.com", "user", "free").unwrap();
        let (refresh, _) = create_refresh_token(1, "a@b.com", "user", "free").unwrap();

        let access_claims = validate_access_token(&access).unwrap();
        let refresh_claims = validate_access_token(&refresh).unwrap();

        assert!(refresh_claims.exp > access_claims.exp);
    }
}
