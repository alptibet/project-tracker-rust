use mongodb::bson::oid::ObjectId;
use mongodb::Database;
use rocket::http::{Cookie, CookieJar};
use rocket::serde::json::Json;
use rocket::State;

use crate::controllers::auth::{check_password, create_send_token};
use crate::controllers::user;
use crate::errors::apperror::AppError;
use crate::models::response::{DocResponse, MessageResponse, VecResponse};
use crate::models::user::{AuthError, AuthenticatedUser, LoginInput, User, UserId, UserInput};

#[get("/get-all")]
pub async fn get_users(
    db: &State<Database>,
    _auth_user: AuthenticatedUser,
) -> Result<Json<VecResponse<User>>, AppError> {
    match user::find_users(&db).await {
        Ok(_user_doc) => Ok(Json(VecResponse {
            message: "success".to_string(),
            data: _user_doc,
        })),
        Err(_error) => Err(AppError::build(400)),
    }
}

#[get("/<_id>")] //deneme amacli sadece id bulmak icin
pub async fn get_one_user(
    db: &State<Database>,
    _id: String,
) -> Result<Json<DocResponse<UserId>>, AppError> {
    let oid = parse_oid(_id);
    match user::match_user_id(&db, oid?).await {
        Ok(_user_doc) => {
            if _user_doc.is_none() {
                return Err(AppError::build(404));
            }
            Ok(Json(DocResponse {
                message: "success".to_string(),
                data: _user_doc.unwrap(),
            }))
        }
        Err(_error) => Err(AppError::build(404)),
    }
}

#[post("/signup", data = "<input>")]
pub async fn signup(
    db: &State<Database>,
    input: Json<UserInput>,
    cookies: &CookieJar<'_>,
) -> Result<Json<MessageResponse>, AppError> {
    match user::insert_user(&db, input).await {
        Ok(_user_doc) => {
            let token = create_send_token(&_user_doc._id);
            cookies.add(token);
            Ok(Json(MessageResponse {
                message: "success".to_string(),
            }))
        }
        Err(_error) => Err(AppError::build(400)),
    }
}

#[post("/login", data = "<input>")]
pub async fn login(
    db: &State<Database>,
    input: Json<LoginInput>,
    cookies: &CookieJar<'_>,
) -> Result<Json<MessageResponse>, AppError> {
    let auth_info = match user::find_auth_info(&db, &input.username).await {
        Ok(_auth_info) => {
            if _auth_info.is_none() {
                return Err(AppError::build(404));
            }
            Ok(_auth_info.unwrap())
        }
        Err(_error) => Err(AppError::build(400)),
    };
    let unwrapped_auth = auth_info.unwrap();
    match check_password(&input.password, &unwrapped_auth.password) {
        Ok(_match) => {
            if _match {
                cookies.add(create_send_token(&unwrapped_auth._id));
                Ok(Json(MessageResponse {
                    message: "success".to_string(),
                }))
            } else {
                return Err(AppError::build(401));
            }
        }
        Err(_error) => Err(AppError::build(500)),
    }
}

#[post("/logout")]
pub fn logout(cookies: &CookieJar<'_>) {
    cookies.remove(Cookie::named("token"));
}

fn parse_oid(_id: String) -> Result<ObjectId, AppError> {
    let oid = ObjectId::parse_str(&_id);
    match oid {
        Ok(_oid) => Ok(_oid),
        Err(_error) => Err(AppError::build(400)),
    }
}
