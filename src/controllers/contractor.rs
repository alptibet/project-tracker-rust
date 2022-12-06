use futures::stream::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, Document};
use mongodb::options::FindOneAndUpdateOptions;
use mongodb::options::ReturnDocument;
use mongodb::Database;
use rocket::serde::json::Json;

use crate::models::contractor::Contractor;
use crate::models::contractor::ContractorDocument;
use crate::models::contractor::ContractorInput;

pub async fn find_contractors(db: &Database) -> mongodb::error::Result<Vec<Contractor>> {
    let collection = db.collection::<ContractorDocument>("contractors");
    let mut cursor = collection.find(None, None).await?;
    let mut contractors: Vec<Contractor> = vec![];

    while let Some(result) = cursor.try_next().await? {
        let _id = result._id;
        let name = result.name;
        let contractor_json = Contractor {
            _id: _id.to_string(),
            name: name.to_string(),
        };
        contractors.push(contractor_json);
    }

    Ok(contractors)
}

pub async fn find_one_contractor(
    db: &Database,
    oid: ObjectId,
) -> mongodb::error::Result<Option<Contractor>> {
    let collection = db.collection::<ContractorDocument>("contractors");

    let contractor_doc = collection.find_one(doc! {"_id":oid}, None).await?;
    if contractor_doc.is_none() {
        return Ok(None);
    }
    let unwrapped_doc = contractor_doc.unwrap();
    let contractor_json = Contractor {
        _id: unwrapped_doc._id.to_string(),
        name: unwrapped_doc.name,
    };

    Ok(Some(contractor_json))
}

pub async fn insert_contractor(
    db: &Database,
    input: Json<ContractorInput>,
) -> mongodb::error::Result<Contractor> {
    let collection = db.collection::<Document>("contractors");
    let contractor_document = doc! {"name": &input.name};
    let insert_one_result = collection.insert_one(contractor_document, None).await?;
    let contractor_name = &input.name.to_string();
    let contractor_json = Contractor {
        _id: insert_one_result.inserted_id.to_string(),
        name: contractor_name.to_string(),
    };
    Ok(contractor_json)
}

pub async fn delete_contractor(
    db: &Database,
    oid: ObjectId,
) -> mongodb::error::Result<Option<String>> {
    let collection = db.collection::<Document>("contractors");
    let contractor_doc = collection
        .find_one_and_delete(doc! {"_id": oid}, None)
        .await?;
    if contractor_doc.is_none() {
        return Ok(None);
    }
    Ok(Some("Document deleted".to_string()))
}

pub async fn update_contractor(
    db: &Database,
    oid: ObjectId,
    input: Json<ContractorInput>,
) -> mongodb::error::Result<Option<Contractor>> {
    let collection = db.collection::<ContractorDocument>("contractors");
    let update_options = FindOneAndUpdateOptions::builder()
        .return_document(ReturnDocument::After)
        .build();

    let contractor_doc = collection
        .find_one_and_update(
            doc! {"_id": oid},
            doc! {"$set": {"name": input.name.clone()}},
            update_options,
        )
        .await?;
    if contractor_doc.is_none() {
        return Ok(None);
    }

    let unwrapped_doc = contractor_doc.unwrap();
    let contractor_json = Contractor {
        _id: unwrapped_doc._id.to_string(),
        name: unwrapped_doc.name,
    };

    Ok(Some(contractor_json))
}
