use mongodb::{
    Client,
};

use crate::models::turtle::Turtle;

pub async fn create(turtle: Turtle) -> Result<String, mongodb::error::Error> {
    let options = mongodb::options::ClientOptions::parse("mongodb://localhost:27017").await?;

    let client = Client::with_options(options)?;
    let collection = client.database("test-db").collection::<Turtle>("test-col");

    // let Turtle { length, name } = turtle;

    let result = collection.insert_one(turtle, None).await?;

    return Ok(result.inserted_id.to_string());
}

pub async fn read(id: String) -> Result<Turtle, mongodb::error::Error> {
    // Read
    panic!("not implemented")
}

pub async fn update(id: String, turtle: Turtle) -> Result<(), mongodb::error::Error> {
    panic!("not implemented")
}

pub async fn delete(id: String) -> Result<(), mongodb::error::Error> {
    panic!("not implemented")
}
