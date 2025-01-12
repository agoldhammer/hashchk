use anyhow::Result;
use mongodb::bson::doc;
use mongodb::options::FindOptions;
use mongodb::{Client, Cursor};
use serde::{Deserialize, Serialize};
// use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct HashRecord {
    #[serde(rename = "_id")]
    pub id: String,
    pub hash: u32,
}

#[allow(dead_code)]
const URI: &str = "mongodb://192.168.0.128:27017";

pub async fn get_hashrecords(dbname: &str) -> Result<Cursor<HashRecord>> {
    let client = Client::with_uri_str(URI).await?;
    let db = client.database(dbname);
    let find_options = FindOptions::builder()
        .projection(doc! { "_id": 1, "hash": 1 })
        .build(); // .projection(doc! { "_id": 1, "hash": 1 " }).build();
    let collection = db.collection("articles");
    let cursor = collection.find(doc! {}).with_options(find_options).await?;
    Ok(cursor)
}
