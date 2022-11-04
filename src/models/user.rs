use mongodb::bson::datetime::DateTime;
use mongodb::bson::oid::ObjectId;
use rocket::serde::{Deserialize, Serialize};

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(crate = "rocket::serde")]
// pub enum UserRole{
//     User,
//     Admin,
//     Superuser,
// }

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub _id: String,
    pub name: String,
    pub surname: String,
    pub username: String,
    pub email: String,
    pub active: bool,
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
    pub role: String,
}
