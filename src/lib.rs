#![allow(unused_imports)]
use anyhow::Result;
use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use mongodb::options::FindOptions;
use mongodb::{Client, Collection, Cursor};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HashRecord {
    #[serde(rename = "_id")]
    pub id: String,
    pub hash: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HashCount {
    pub hash: String,
    pub hcount: u32,
}

#[allow(dead_code)]
// const URI: &str = "mongodb://192.168.0.128:27017";
const URI: &str = "mongodb://127.0.0.1:27017";

pub async fn get_hashrecords(dbname: &str) -> Result<()> {
    let client = Client::with_uri_str(URI).await?;
    let db = client.database(dbname);
    // let find_options = FindOptions::builder()
    //     .projection(doc! { "_id": 1, "hash": 1 })
    //     .build(); // .projection(doc! { "_id": 1, "hash": 1 " }).build();
    let articles: Collection<HashRecord> = db.collection("articles");

    let sorter1 = doc! {
        "$sort": doc! {
            "hash": -1
        }
    };

    let grouper = doc! {
        "$group": doc! {
            "_id": "$hash",
            "hash": doc! {"$first":"$hash"},
            "hcount": doc! {
                "$sum": 1
            }
        }
    };
    let sorter2 = doc! {
        "$sort": doc! {
            "hcount": -1
        }
    };
    let filter = doc! {
        "$match": doc! {
            "hash": doc! {
                "$gt": 1
            }
        }
    };
    let pipeline = vec![sorter1, grouper, sorter2, filter];
    let mut cursor = articles.aggregate(pipeline).await?;
    println!("Printing hash records for hcount > 1");
    while let Some(doc) = cursor.try_next().await? {
        println!("{:?}", doc);
    }

    Ok(())
}
