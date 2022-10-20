use warp::{path, Filter, Rejection};

pub fn build_route() -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
    return path!("read" / u32).map(|id| format!("read: {}", id));
}
