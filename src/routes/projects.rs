use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Database;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Project<'a> {
    pub name: &'a str,
    pub duration: u8,
    pub address: &'a str,
}

#[get("/projects/get-all-projects")]
pub fn get_all_projects<'a>() -> Json<Project<'a>> {
    Json(Project {
        name: "TEST",
        duration: 12,
        address: "TEST ADDRESS",
    })
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Contractor {
    pub name: String,
    pub _id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ContractorDocument {
    pub name: String,
    pub _id: ObjectId,
}

#[get("/projects/get-one")]
pub async fn get_one(db: &Database) -> mongodb::error::Result<Vec<Contractor>> {
    let collection = db.collection::<ContractorDocument>("contractors");
    let mut cursor = collection.find(None, None).await?;

    let mut contractors: Vec<Contractor> = vec![];
    while let Some(result) = cursor.try_next().await? {
        let _id = result._id;
        let name = result.name;

        let contractors_json = Contractor {
            _id: _id.to_string(),
            name: name.to_string(),
        };
        contractors.push(contractors_json);
    }

    Ok(contractors)
}
