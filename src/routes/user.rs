use mongodb::Database;
use rocket::serde::json::Json;
use rocket::State;

use crate::controllers::user;
use crate::errors::apperror::AppError;
use crate::models::response::VecResponse;
use crate::models::user::User;

#[get("/get-all")]
pub async fn get_users(db: &State<Database>) -> Result<Json<VecResponse<User>>, AppError> {
    match user::find_users(&db).await {
        Ok(_user_doc) => Ok(Json(VecResponse {
            message: "success".to_string(),
            data: _user_doc,
        })),
        Err(_error) => Err(AppError::build(404)),
    }
}
