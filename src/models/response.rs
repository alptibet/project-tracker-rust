use rocket::serde::{Deserialize, Serialize};

use crate::models::contractor::Contractor;
use crate::models::user::User;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct VecResponse<DocType> {
    pub message: String,
    pub data: Vec<DocType>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DocResponse<DocType> {
    pub message: String,
    pub data: DocType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct MessageResponse {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
enum DocType {
    User(User),
    Contractor(Contractor),
}
