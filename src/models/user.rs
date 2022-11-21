use jsonwebtoken::crypto::verify;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Validation};
use mongodb::bson::datetime::DateTime;
use mongodb::bson::oid::ObjectId;
use mongodb::Database;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::{Deserialize, Serialize};
use std::env;

use crate::controllers::user;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum UserRole {
    User,
    Admin,
    Superuser,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub _id: String,
    pub name: String,
    pub surname: String,
    pub username: String,
    pub email: String,
    pub active: String,
    pub password: String,
    pub passwordChangeAt: String,
    pub role: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserDocument {
    pub _id: ObjectId,
    pub name: String,
    pub surname: String,
    pub username: String,
    pub email: String,
    pub active: bool,
    pub password: String,
    pub passwordChangeAt: DateTime,
    pub role: UserRole,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserInput {
    pub name: String,
    pub surname: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub passwordConfirm: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AuthInfo {
    pub password: String,
    pub _id: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginInput {
    pub username: String,
    pub password: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]

pub struct AuthenticatedUser {
    pub _id: String,
}

#[derive(Debug)]
pub enum AuthError {
    MissingKey,
    InvalidKey,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = AuthError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Returns true if `key` is a valid API key string.
        async fn is_valid_token(token: &str) -> bool {
            let secret_key = env::var("JWT_SECRET").expect("No JWT KEY found in environment.");
            let payload = verify(
                token,
                "oha".as_bytes(),
                &DecodingKey::from_secret(secret_key.as_bytes()),
                Algorithm::HS256,
            );
            println!("{:?}", payload.unwrap());
            //gelen token jwt ile verify etmek lazim ona gore true ya da false return
            true
        }

        let token: Option<String> = req
            .cookies()
            .get("token")
            .and_then(|cookie| cookie.value().parse().ok());

        match token {
            None => Outcome::Failure((Status::BadRequest, AuthError::MissingKey)),
            Some(token) if is_valid_token(&token).await => Outcome::Success(AuthenticatedUser {
                _id: "deneme".to_string(),
            }),
            Some(_) => Outcome::Failure((Status::Unauthorized, AuthError::InvalidKey)),
        }
    }
}
