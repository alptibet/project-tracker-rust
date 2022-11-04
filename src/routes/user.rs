use mongodb::bson::oid::ObjectId;
use mongodb::Database;
use rocket::serde::json::Json;
use rocket::State;

use crate::controllers::user;
use crate::errors::apperror::AppError;
use crate::models::response::DocResponse;
use crate::models::response::MessageResponse;
use crate::models::response::UserDocVecResponse;
// use crate::models::user::UserInput;

#[get("/get-all")]
pub async fn get_users(db: &State<Database>) -> Result<Json<UserDocVecResponse>, AppError> {
    match user::find_users(&db).await {
        Ok(_user_doc) => Ok(Json(UserDocVecResponse {
            message: "success".to_string(),
            data: _user_doc,
        })),
        Err(_error) => Err(AppError::build(404)),
    }
}
