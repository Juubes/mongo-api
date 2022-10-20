use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Turtle {
    pub name: String,
    pub length: f32,
}
