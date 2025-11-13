use anyhow::{Context, Result};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub email: Option<String>,
    pub role: Option<String>,
    pub exp: i64,
    pub iat: i64,
    pub iss: Option<String>,
}

pub struct JwtValidator {
    jwt_secret: String,
}

impl JwtValidator {
    pub fn new(jwt_secret: String) -> Self {
        Self { jwt_secret }
    }

    pub fn validate_token(&self, token: &str) -> Result<Uuid> {
        // Set up validation
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;
        validation.validate_nbf = false;
        validation.set_issuer(&["supabase"]);

        // Decode token
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &validation,
        )
        .context("Failed to decode JWT token")?;

        // Extract user_id from sub claim
        let user_id =
            Uuid::parse_str(&token_data.claims.sub).context("Invalid user_id in JWT token")?;

        Ok(user_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_validator() {
        // This is a test and won't work with real tokens
        // In production, use the actual Supabase JWT secret
        let validator = JwtValidator::new("test-secret".to_string());

        // Test with invalid token
        let result = validator.validate_token("invalid-token");
        assert!(result.is_err());
    }
}
