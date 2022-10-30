use mongodb::bson::oid::ObjectId;
use rocket::serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Contractor {
    pub _id: String,
    pub name: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ContractorDocument {
    pub _id: ObjectId,
    pub name: String,
}
