use warp::{Filter, Rejection};

pub fn build_route() -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
    return warp::path!("create").map(|| "create".to_string());
}
