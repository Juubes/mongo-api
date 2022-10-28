use serde::{Deserialize, Serialize, Serializer};
use warp::{
    hyper::{Body, Response},
    Reply,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Turtle {
    pub name: String,
    pub length: f32,
}

impl Reply for Turtle {
    fn into_response(self) -> warp::reply::Response {
        let str = serde_json::to_string(&self).unwrap();
        return Response::new(Body::from(str));
    }
}
