use std::future;

use warp::{Filter, Rejection};

use crate::data::data_handler;

pub fn build_route() -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
    warp::path!("delete" / String)
        .map(|id| async { data_handler::delete(id).await.unwrap() })
        .then(|ok| future::ready(String::from("OK")))
}
