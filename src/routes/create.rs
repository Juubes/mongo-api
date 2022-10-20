use std::future;

use serde::{Deserialize, Serialize};
use tokio::{sync::futures, task::futures};
use warp::{Filter, Future, Rejection};

use crate::{data::data_handler, models::turtle::Turtle};

#[derive(Debug, Serialize, Deserialize)]
struct JsonData {
    data: String,
}

pub fn build_route() -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
    let route = warp::path!("create").and(warp::body::json().map(|turtle: Turtle| {
        let id = data_handler::create(turtle);
        println!("Turtle created");

        return id;
    }));

    route
}
