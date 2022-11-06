use bcrypt::hash;
use futures::stream::TryStreamExt;
use mongodb::bson::{doc, DateTime, Document};
use mongodb::Database;
use rocket::serde::json::Json;

use crate::models::user::User;
use crate::models::user::UserDocument;
use crate::models::user::UserInput;
use crate::models::user::UserRole;

pub async fn find_users(db: &Database) -> mongodb::error::Result<Vec<User>> {
    let collection = db.collection::<UserDocument>("users");
    let mut cursor = collection.find(None, None).await?;
    let mut users: Vec<User> = vec![];

    while let Some(result) = cursor.try_next().await? {
        let _id = result._id;
        let name = result.name;
        let surname = result.surname;
        let username = result.username;
        let email = result.email;
        let active = result.active;
        let password = result.password;
        let password_change_at = result.passwordChangeAt;
        let userrole = result.role;
        let user_json = User {
            _id: _id.to_string(),
            name: name.to_string(),
            surname: surname.to_string(),
            username: username.to_string(),
            email: email.to_string(),
            active: active.to_string(),
            password: password.to_string(),
            passwordChangeAt: password_change_at.to_string(),
            role: match userrole {
                UserRole::Admin => "Admin".to_string(),
                UserRole::User => "User".to_string(),
                UserRole::Superuser => "Superuser".to_string(),
            },
        };
        users.push(user_json);
    }

    Ok(users)
}

pub async fn insert_user(db: &Database, input: Json<UserInput>) -> mongodb::error::Result<User> {
    let collection = db.collection::<Document>("users");
    let password_created_at: DateTime = DateTime::now();
    let hashed_password = hash(&input.password, 12).unwrap();
    let user_document = doc! {
        "name": &input.name,
        "surname": &input.surname,
        "username": &input.username,
        "email": &input.email,
        "active": true,
        "password": &hashed_password,
        "passwordChangeAt": password_created_at,
        "role": "User".to_string(),
    };

    let insert_one_result = collection.insert_one(user_document, None).await?;

    let name = &input.name;
    let surname = &input.surname;
    let username = &input.username;
    let email = &input.email;
    let active = true;
    let password = &hashed_password;
    let role = "User".to_string();

    let user_json = User {
        _id: insert_one_result.inserted_id.to_string(),
        name: name.to_string(),
        surname: surname.to_string(),
        username: username.to_string(),
        email: email.to_string(),
        active: active.to_string(),
        password: password.to_string(),
        passwordChangeAt: password_created_at.to_string(),
        role: role.to_string(),
    };
    Ok(user_json)
}
