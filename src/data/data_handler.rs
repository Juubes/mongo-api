use mongodb::{
    bson::{doc, Document},
    Client,
};

use crate::models::turtle::Turtle;

pub async fn create(turtle: Turtle) -> Result<String, mongodb::error::Error> {
    let options = mongodb::options::ClientOptions::parse("mongodb://localhost:27017").await?;

    let client = Client::with_options(options)?;
    let collection = client
        .database("test-db")
        .collection::<Document>("test-col");

    let Turtle { length, name } = turtle;

    let result = collection
        .insert_one(doc! {"length": length, "name": name}, None)
        .await?;

    return Result::Ok(result.inserted_id.to_string());
}

pub async fn read() {}
pub async fn update() {}
pub async fn delete() {}
