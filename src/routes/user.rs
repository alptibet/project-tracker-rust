use mongodb::Database;
use rocket::serde::json::Json;
use rocket::State;

use crate::controllers::user;
use crate::errors::apperror::AppError;
use crate::models::response::{VecResponse, DocResponse};
use crate::models::user::User;
use crate::models::user::UserInput;

#[get("/get-all")]
pub async fn get_users(db: &State<Database>) -> Result<Json<VecResponse<User>>, AppError> {
    match user::find_users(&db).await {
        Ok(_user_doc) => Ok(Json(VecResponse {
            message: "success".to_string(),
            data: _user_doc,
        })),
        Err(_error) => {
            println!("{_error}");
            Err(AppError::build(404))},
    }
}

#[post("/signup", data = "<input>")]
pub async fn signup(db: &State<Database>, input:Json<UserInput>) -> Result<Json<DocResponse<User>>, AppError> {
    match user::insert_user(&db, input).await {
        Ok(_user_doc)=> Ok(Json(DocResponse{
            message:"success".to_string(),
            data: _user_doc,
        })),
        Err(_error) => Err(AppError::build(400))
    }
}// also send jwt token
