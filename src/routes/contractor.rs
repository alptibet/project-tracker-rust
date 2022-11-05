use mongodb::bson::oid::ObjectId;
use mongodb::Database;
use rocket::serde::json::Json;
use rocket::State;

use crate::controllers::contractor;
use crate::errors::apperror::AppError;
use crate::models::contractor::ContractorInput;
use crate::models::response::DocResponse;
use crate::models::response::MessageResponse;
use crate::models::response::VecResponse;
use crate::models::contractor::Contractor;

#[get("/get-all")]
pub async fn get_contractors(db: &State<Database>) -> Result<Json<VecResponse<Contractor>>, AppError> {
    match contractor::find_contractors(&db).await {
        Ok(_contractor_doc) => Ok(Json(VecResponse {
            message: "success".to_string(),
            data: _contractor_doc,
        })),
        Err(_error) => Err(AppError::build(404)),
    }
}

#[get("/<_id>")]
pub async fn get_one_contractor(
    db: &State<Database>,
    _id: String,
) -> Result<Json<DocResponse<Contractor>>, AppError> {
    let oid = parse_oid(_id);
    match contractor::find_one_contractor(&db, oid?).await {
        Ok(_contractor_doc) => {
            if _contractor_doc.is_none() {
                return Err(AppError::build(404));
            }
            Ok(Json(DocResponse {
                message: "success".to_string(),
                data: _contractor_doc.unwrap(),
            }))
        }
        Err(_error) => Err(AppError::build(404)),
    }
}

#[post("/", data = "<input>")]
pub async fn insert_one_contractor(
    db: &State<Database>,
    input: Json<ContractorInput>,
) -> Result<Json<DocResponse<Contractor>>, AppError> {
    match contractor::insert_contractor(&db, input).await {
        Ok(_contractor_doc) => Ok(Json(DocResponse {
            message: "success".to_string(),
            data: _contractor_doc,
        })),
        Err(_error) => {
            println!("{_error}");
            Err(AppError::build(400))
        }
    }
}

#[delete("/<_id>")]
pub async fn delete_one_contractor(
    db: &State<Database>,
    _id: String,
) -> Result<Json<MessageResponse>, AppError> {
    let oid = parse_oid(_id);
    match contractor::delete_contractor(&db, oid?).await {
        Ok(_contractor_doc) => {
            if _contractor_doc.is_none() {
                return Err(AppError::build(404));
            }
            Ok(Json(MessageResponse {
                message: "success".to_string(),
            }))
        }
        Err(_error) => Err(AppError::build(404)),
    }
}

#[patch("/<_id>", data = "<input>")]
pub async fn update_one_contractor(
    db: &State<Database>,
    _id: String,
    input: Json<ContractorInput>,
) -> Result<Json<DocResponse<Contractor>>, AppError> {
    let oid = parse_oid(_id);
    match contractor::update_contractor(&db, oid?, input).await {
        Ok(_contractor_doc) => {
            if _contractor_doc.is_none() {
                return Err(AppError::build(404));
            }
            Ok(Json(DocResponse {
                message: "success".to_string(),
                data: _contractor_doc.unwrap(),
            }))
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
