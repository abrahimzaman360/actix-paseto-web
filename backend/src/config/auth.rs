use base64::engine::general_purpose;
use base64::Engine;
use pasetors::keys::SymmetricKey;
use pasetors::version4::V4;

pub fn paseto_key() -> SymmetricKey<V4> {
    let key_b64 = std::env::var("PASETO_KEY").expect("Missing PASETO_LOCAL_KEY in .env");
    let key_bytes = general_purpose::STANDARD
        .decode(&key_b64)
        .expect("Invalid base64 key");
    SymmetricKey::<V4>::from(&key_bytes).expect("Invalid paseto key")
}
