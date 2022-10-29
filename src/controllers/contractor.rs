use futures::stream::TryStreamExt;
use mongodb::Database;

use crate::models::contractor::Contractor;
use crate::models::contractor::ContractorDocument;

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
