use warp::{path, Filter, Rejection};

pub fn build_route() -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
    path!("delete" / u32).map(|id| format!("delete: {}", id))
}
