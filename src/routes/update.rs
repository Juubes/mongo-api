use warp::{path, Filter, Rejection};

pub fn build_route() -> impl Filter<Error = Rejection, Extract = (String,)> + Clone {
    return path!("create").map(|| format!(""));
}
