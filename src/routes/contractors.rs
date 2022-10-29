use mongodb::bson::oid::ObjectId;
use mongodb::Database;
use rocket::serde::json::Json;
use rocket::State;

use crate::controllers::contractor;
use crate::models::contractor::Contractor;

#[get("/contractors/get-all")]
pub async fn get_contractors(db: &State<Database>) -> Result<Json<Vec<Contractor>>, &'static str> {
    match contractor::find_contractors(&db).await {
        Ok(_contractor_doc) => Ok(Json(_contractor_doc)),
        Err(_error) => Err("Error"),
    }
}

#[get("/contractors/<_id>")]
pub async fn get_one_contractor(
    db: &State<Database>,
    _id: String,
) -> Result<Json<Contractor>, &'static str> {
    let oid = ObjectId::parse_str(&_id).unwrap(); // Must handle error here parse_str returns a result
    match contractor::find_one_contractor(&db, oid).await {
        Ok(_contractor_doc) => Ok(Json((_contractor_doc).unwrap())), //Must also handle error here as doc can be none
        Err(_error) => Err("Could not get contractor"),// Also implement own error handling
    }
}