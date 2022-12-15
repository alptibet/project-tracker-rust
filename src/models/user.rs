use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use mongodb::bson::datetime::DateTime;
use mongodb::bson::oid::ObjectId;
use mongodb::Database;
use rocket::http::Status;
use rocket::outcome::try_outcome;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::{Deserialize, Serialize};
use rocket::State;
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
pub struct UserId {
    pub _id: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserIdDocument {
    pub _id: ObjectId,
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

pub struct AuthenticatedUser;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<AuthenticatedUser, ()> {
        let db = try_outcome!(req.guard::<&State<Database>>().await);
        async fn is_valid_token(db: &Database, token: &str) -> bool {
            let secret_key = env::var("JWT_SECRET").expect("No JWT KEY found in environment.");
            let payload = decode::<Claims>(
                token,
                &DecodingKey::from_secret(secret_key.as_bytes()),
                &Validation::new(Algorithm::HS256),
            );

            if let Ok(_payload) = payload {
                let oid = ObjectId::parse_str(_payload.claims.sub).unwrap();

                match user::match_user_id(db, oid).await {
                    Ok(_dbuser) => {
                        if _dbuser.is_none() {
                            return false;
                        }
                        return true;
                    }
                    Err(_error) => false,
                };
                true
            } else {
                false
            }
        }

        let auth_bearer = req.headers().get_one("authorization");
        let token: Option<String>;
        if let Some(_auth_bearer) = auth_bearer {
            let bearer: Vec<&str> = auth_bearer.unwrap().split(' ').collect();
            token = Some(bearer[1].to_string());
        } else {
            token = req
                .cookies()
                .get("token")
                .and_then(|token| token.value().parse().ok());
        }

        match token {
            None => Outcome::Failure((Status::BadRequest, ())),
            Some(_token) if is_valid_token(db, &_token).await => {
                Outcome::Success(AuthenticatedUser)
            }
            Some(_) => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}
