pub mod data;
pub mod models;
pub mod routes;

use models::turtle::Turtle;
use mongodb::{
    bson::{doc, Document},
    Client,
};
use warp::Filter;

#[tokio::main]
async fn main() {
    start_server().await;
}

async fn start_server() {
    let create = routes::create::build_route();
    let read = routes::read::build_route();
    let update = routes::update::build_route();
    let delete = routes::delete::build_route();

    let routes = create.or(read).or(update).or(delete);

    warp::serve(routes).run(([127, 0, 0, 1], 3000)).await
}
