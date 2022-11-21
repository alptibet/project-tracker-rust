use bcrypt::{verify, BcryptError};
use jsonwebtoken::crypto::sign;
use jsonwebtoken::{Algorithm, EncodingKey};
use rocket::http::Cookie;
use std::env;

pub fn create_send_token<'a>(_id: &str) -> Cookie<'a> {
    Cookie::build("token", sign_token(&_id))
        .path("/")
        .secure(false)
        .http_only(true)
        .finish()
}

pub fn sign_token(_id: &str) -> String {
    let secret_key = env::var("JWT_SECRET").expect("No JWT KEY found in environment.");
    let result = sign(
        _id.as_bytes(),
        &EncodingKey::from_secret(secret_key.as_bytes()),
        Algorithm::HS256,
    )
    .unwrap();
    result
}

pub fn check_password(password: &str, hashed_password: &str) -> Result<bool, BcryptError> {
    verify(password, hashed_password)
}
