use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

/// JWT claims stored in access tokens.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub email: String,
    pub role: String,
    pub tier: String,
    pub exp: i64,
    pub iat: i64,
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
