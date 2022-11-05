use rocket::serde::{Deserialize, Serialize};

use crate::models::contractor::Contractor;

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(crate = "rocket::serde")]
// pub struct DocVecResponse {
//     pub message: String,
//     pub data: Vec<Contractor>,
// }

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct VecResponse<T> {
    pub message: String,
    pub data: Vec<T>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DocResponse<T> {
    pub message: String,
    pub data: T,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct MessageResponse {
    pub message: String,
}
