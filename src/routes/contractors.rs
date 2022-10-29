use mongodb::Database;
use rocket::serde::json::Json;
use rocket::State;

use crate::controllers::contractor;
use crate::models::contractor::Contractor;

#[get("/contractors/get-all")]
pub async fn get_contractors(db: &State<Database>) -> Result<Json<Vec<Contractor>>, &'static str> {
    match contractor::find_contractors(&db).await {
        Ok(_contractor_doc) => Ok(Json(_contractor_doc)),
        Err(_error) => Err("Error"),
    }
}
