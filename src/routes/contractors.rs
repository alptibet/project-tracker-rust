use mongodb::bson::oid::ObjectId;
use mongodb::Database;
use rocket::serde::json::Json;
use rocket::State;

use crate::controllers::contractor;
use crate::errors::apperror::AppError;
use crate::models::contractor::Contractor;
use crate::models::contractor::ContractorInput;

#[get("/get-all")]
pub async fn get_contractors(db: &State<Database>) -> Result<Json<Vec<Contractor>>, AppError> {
    match contractor::find_contractors(&db).await {
        Ok(_contractor_doc) => Ok(Json(_contractor_doc)),
        Err(_error) => Err(AppError::build(404)),
    }
}

#[get("/<_id>")]
pub async fn get_one_contractor(
    db: &State<Database>,
    _id: String,
) -> Result<Json<Contractor>, AppError> {
    let oid = parse_oid(_id);
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

#[post("/", data = "<input>")]
pub async fn insert_one_contractor(
    db: &State<Database>,
    input: Json<ContractorInput>,
) -> Result<Json<Contractor>, AppError> {
    match contractor::insert_contractor(&db, input).await {
        Ok(_contractor_id) => Ok(Json(_contractor_id)),
        Err(_error) => Err(AppError::build(400)),
    }
}

#[delete("/<_id>")]
pub async fn delete_one_contractor(
    db: &State<Database>,
    _id: String,
) -> Result<Json<String>, AppError> {
    let oid = parse_oid(_id);
    match contractor::delete_contractor(&db, oid?).await {
        Ok(_contractor_doc) => {
            if _contractor_doc.is_none() {
                return Err(AppError::build(404));
            }
            Ok(Json(_contractor_doc.unwrap()))
        }
        Err(_error) => Err(AppError::build(404)),
    }
}

#[patch("/<_id>", data = "<input>")]
pub async fn update_one_contractor(
    db: &State<Database>,
    _id: String,
    input: Json<ContractorInput>,
) -> Result<Json<Contractor>, AppError> {
    let oid = parse_oid(_id);
    match contractor::update_contractor(&db, oid?, input).await {
        Ok(_contractor_doc) => {
            if _contractor_doc.is_none() {
                return Err(AppError::build(404));
            }
            Ok(Json(_contractor_doc.unwrap()))
        }
        Err(_error) => Err(AppError::build(404)),
    }
}

fn parse_oid(_id: String) -> Result<ObjectId, AppError> {
    let oid = ObjectId::parse_str(&_id);
    match oid {
        Ok(_oid) => Ok(_oid),
        Err(_error) => Err(AppError::build(400)),
    }
}
