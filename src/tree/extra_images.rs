use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ExtraImage {
    pub x: f32,
    pub y: f32,
    pub image: String,
}
