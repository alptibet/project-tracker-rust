use jsonwebtoken::crypto::sign;
use jsonwebtoken::{Algorithm, EncodingKey};
use std::env;

fn signToken(_id: &str) -> Result<String, ()> {
    let secret_key = env::var("JWT_SECRET").expect("No JWT KEY found in environment.");
    let result = sign(
        _id.as_bytes(),
        &EncodingKey::from_secret(secret_key.as_bytes()),
        Algorithm::HS256,
    )
    .unwrap();
    Ok(result)
}
