use futures::stream::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, Document};
use mongodb::options::FindOneAndUpdateOptions;
use mongodb::options::ReturnDocument;
use mongodb::Database;
use rocket::serde::json::Json;

use crate::models::user::User;
use crate::models::user::UserDocument;
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
            active,
            password: password.to_string(),
            passwordChangeAt: password_change_at.to_string(),
            role: match userrole{
                UserRole::Admin => "Admin".to_string(),
                UserRole::User => "User".to_string(),
                UserRole::Superuser => "Superuser".to_string(),
            }
        };
        users.push(user_json);
    }

    Ok(users)
}
