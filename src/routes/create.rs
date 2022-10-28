use serde::{Deserialize, Serialize};
use warp::{Filter, Rejection};

use crate::{data::data_handler, models::turtle::Turtle};

#[derive(Debug, Serialize, Deserialize)]
struct JsonData {
    data: String,
}

pub fn build_route() -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
    warp::path!("create")
        .and(
            warp::body::json()
                .map(|turtle: Turtle| async { data_handler::create(turtle).await.unwrap() }),
        )
        .then(|v| v)
}
