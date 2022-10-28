use warp::{path, Filter, Rejection};

use crate::{data::data_handler::read, models::turtle::Turtle};

pub fn build_route() -> impl Filter<Extract = (Turtle,), Error = Rejection> + Clone {
    path!("read" / String)
        .map(|id| async { Turtle::from(read(id).await.unwrap()) })
        .then(|val| val)
}
