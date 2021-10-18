use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Group {
    pub x: f32,
    pub y: f32,
    pub orbits: Vec<i32>,
    pub nodes: Vec<String>,
}
