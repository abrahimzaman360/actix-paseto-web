use crate::utils::paseto::verify_token;
use actix_web::error::ErrorInternalServerError;
use actix_web::middleware::Next;
use actix_web::{
    Error, HttpMessage,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    error::ErrorUnauthorized,
    web,
};
use pasetors::claims::Claims;
use pasetors::keys::SymmetricKey;
use pasetors::version4::V4;
use std::future::Future;
use std::pin::Pin;

// Custom claims structure - customize this to match your token payload
#[derive(Debug, Clone)]
pub struct TokenClaims {
    pub user_id: String,
    pub email: String,
    pub roles: Vec<String>,
}

// Trait for custom token verification
pub trait TokenValidator: Send + Sync {
    fn validate(
        &self,
        token: &str,
    ) -> Pin<Box<dyn Future<Output = Result<TokenClaims, String>> + Send>>;
}

pub struct CustomTokenValidator {
    key: SymmetricKey<V4>,
}

impl CustomTokenValidator {
    pub fn new(key: SymmetricKey<V4>) -> Self {
        Self { key }
    }

    // Helper function to convert PASETO Claims to TokenClaims
    fn extract_claims(claims: &Claims) -> Result<TokenClaims, String> {
        let user_id = claims
            .get_claim("user_id")
            .and_then(|v| v.as_str())
            .ok_or("Missing user_id claim")?
            .to_string();

        let email = claims
            .get_claim("email")
            .and_then(|v| v.as_str())
            .ok_or("Missing email claim")?
            .to_string();

        let roles = claims
            .get_claim("roles")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_else(|| vec!["user".to_string()]);

        Ok(TokenClaims {
            user_id,
            email,
            roles,
        })
    }
}

impl TokenValidator for CustomTokenValidator {
    fn validate(
        &self,
        token: &str,
    ) -> Pin<Box<dyn Future<Output = Result<TokenClaims, String>> + Send>> {
        let token = token.to_string();
        let key = self.key.clone();

        Box::pin(async move {
            // Verify the PASETO token
            let claims = verify_token(&key, &token).ok_or("Invalid or expired token")?;

            // Extract and convert claims to our TokenClaims structure
            Self::extract_claims(&claims)
        })
    }
}

/// Extract token from either Cookie or Authorization Bearer header
fn extract_token(req: &ServiceRequest) -> Option<String> {
    // First, try to extract from Authorization header
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                return Some(auth_str[7..].to_string());
            }
        }
    }

    // If not found in header, try to extract from cookie
    if let Some(cookie) = req.cookie("tea-token") {
        return Some(cookie.value().to_string());
    }

    None
}

/// Authentication middleware function
pub async fn auth_middleware<B>(
    req: ServiceRequest,
    next: Next<B>,
) -> Result<ServiceResponse<B>, Error>
where
    B: MessageBody,
{
    // Extract token from request
    let token =
        extract_token(&req).ok_or_else(|| ErrorUnauthorized("Missing authentication token"))?;

    // Get the symmetric key from app data
    let key = req
        .app_data::<web::Data<SymmetricKey<V4>>>()
        .ok_or_else(|| ErrorInternalServerError("Symmetric key not configured"))?;

    // Validate token using PASETO validator
    let validator = CustomTokenValidator::new(key.as_ref().clone());
    let claims = validator
        .validate(&token)
        .await
        .map_err(|e| ErrorUnauthorized(format!("Invalid token: {}", e)))?;

    // Store claims in request extensions for use in handlers
    req.extensions_mut().insert(claims);

    // Continue to next middleware or handler
    next.call(req).await
}
