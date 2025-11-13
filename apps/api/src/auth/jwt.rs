use anyhow::Result;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub role: Option<String>,
}

pub struct JwtValidator {
    jwt_secret: String,
}

impl JwtValidator {
    pub fn new(jwt_secret: String) -> Self {
        Self { jwt_secret }
    }

    pub fn validate_token(&self, token: &str) -> Result<Uuid> {
        let mut validation = Validation::default();
        validation.validate_exp = true;
        validation.algorithms = vec![jsonwebtoken::Algorithm::HS256];
        // Uncomment and configure if your JWT includes iss/aud claims:
        // validation.set_issuer(&["your-auth-issuer"]);
        // validation.set_audience(&["your-api-audience"]);

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &validation,
        )?;

        let user_id = Uuid::parse_str(&token_data.claims.sub)?;
        Ok(user_id)
    }
}
