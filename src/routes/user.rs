use mongodb::Database;
use rocket::http::CookieJar;
use rocket::serde::json::Json;
use rocket::State;

use crate::controllers::auth::create_send_token;
use crate::controllers::user;
use crate::errors::apperror::AppError;
use crate::models::response::{DocResponse, VecResponse};
use crate::models::user::AuthInfo;
use crate::models::user::LoginInput;
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
            Err(AppError::build(400))
        }
    }
}

#[post("/signup", data = "<input>")]
pub async fn signup(
    db: &State<Database>,
    input: Json<UserInput>,
    cookies: &CookieJar<'_>,
) -> Result<Json<DocResponse<User>>, AppError> {
    match user::insert_user(&db, input).await {
        Ok(_user_doc) => {
            let token = create_send_token(&_user_doc._id);
            cookies.add(token);
            Ok(Json(DocResponse {
                message: "success".to_string(),
                data: _user_doc,
            }))
        }
        Err(_error) => Err(AppError::build(400)),
    }
}

#[post("/login", data = "<input>")]
pub async fn login(db: &State<Database>, input: Json<LoginInput>, cookies: &CookieJar<'_>)
/*-> Result<Json<DocResponse<AuthInfo>>, AppError>*/
{
    //get user with user name
    let user = match user::find_auth_info(&db, &input.username).await {
        Ok(_auth_info) => {
            if _auth_info.is_none() {
                // return Err(AppError::build(404));
            }
            Ok(_auth_info.unwrap())
        }
        Err(_error) => Err(AppError::build(400)),
    };
    //get his hashed password
    let user_password = user.unwrap().password;
    //hash the input password and compare if passwords are correct
    //send cookie
}
