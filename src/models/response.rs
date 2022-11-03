use rocket::serde::{Deserialize, Serialize};

use crate::models::contractor::Contractor;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DocVecResponse {
    pub message: String,
    pub data: Vec<Contractor>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DocResponse {
    pub message: String,
    pub data: Contractor,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct MessageResponse {
    pub message: String,
}
