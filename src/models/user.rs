use mongodb::bson::datetime::DateTime;
use mongodb::bson::oid::ObjectId;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::{Deserialize, Serialize};

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
enum AuthError {
    MissingKey,
    InvalidKey,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = AuthError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        /// Returns true if `key` is a valid API key string.
        fn is_valid(key: &str) -> bool {
            true
        }

        match req.headers().get_one("token") {
            None => Outcome::Failure((Status::BadRequest, AuthError::MissingKey)),
            Some(key) if is_valid(key) => Outcome::Success(AuthenticatedUser(user)),
            Some(_) => Outcome::Failure((Status::BadRequest, AuthError::InvalidKey)),
        }
    }
}
