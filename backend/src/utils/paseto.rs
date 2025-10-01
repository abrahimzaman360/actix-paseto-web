use crate::config;
use pasetors::Local;
use pasetors::claims::Claims;
use pasetors::claims::ClaimsValidationRules;
use pasetors::keys::SymmetricKey;
use pasetors::token::UntrustedToken;
use pasetors::{local, version4::V4};
use std::convert::TryFrom;

pub fn generate_token(user_id: &str, email: &str) -> String {
    let key = config::auth::paseto_key();

    let mut claims = Claims::new().unwrap();
    claims.subject(user_id).unwrap();                  // sub claim
    claims.add_additional("user_id", user_id).unwrap(); // add user_id claim
    claims.add_additional("email", email).unwrap(); // add user_id claim
    claims.add_additional("role", "user").unwrap();

    local::encrypt(&key, &claims, None, None).unwrap()
}

pub fn verify_token(key: &SymmetricKey<V4>, token: &str) -> Option<Claims> {
    let rules = ClaimsValidationRules::new();
    let untrusted = UntrustedToken::<Local, V4>::try_from(token).ok()?;
    let trusted = local::decrypt(key, &untrusted, &rules, None, None).ok()?;
    trusted.payload_claims().cloned()
}
