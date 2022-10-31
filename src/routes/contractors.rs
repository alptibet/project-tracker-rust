use mongodb::bson::oid::ObjectId;
use mongodb::Database;
use rocket::serde::json::Json;
use rocket::State;

use crate::controllers::contractor;
use crate::errors::apperror::AppError;
use crate::models::contractor::Contractor;

#[get("/get-all")]
pub async fn get_contractors(db: &State<Database>) -> Result<Json<Vec<Contractor>>, &'static str> {
    match contractor::find_contractors(&db).await {
        Ok(_contractor_doc) => Ok(Json(_contractor_doc)),
        Err(_error) => Err("Error"),
    }
}

#[get("/<_id>")]
pub async fn get_one_contractor(
    db: &State<Database>,
    _id: String,
) -> Result<Json<Contractor>, AppError> {
    let oid = match ObjectId::parse_str(&_id) {
        Ok(_oid) => Ok(_oid),
        Err(_err) => Err(AppError::build(400)),
    };

    match contractor::find_one_contractor(&db, oid?).await {
        Ok(_contractor_doc) => {
            if _contractor_doc.is_none() {
                return Err(AppError::build(404));
            }
            Ok(Json(_contractor_doc.unwrap()))
        }
        Err(_error) => Err(AppError::build(404)),
    }
}

// #[post("/")]
