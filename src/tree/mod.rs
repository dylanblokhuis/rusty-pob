use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub mod assets;
pub mod classes;
pub mod constants;
pub mod extra_images;
pub mod groups;
pub mod nodes;
pub mod skill_sprites;

#[derive(Serialize, Deserialize)]
pub struct Tree {
    pub tree: String,
    pub classes: Vec<classes::Class>,
    pub groups: HashMap<String, groups::Group>,
    pub nodes: HashMap<String, nodes::Node>,
    #[serde(rename = "extraImages")]
    pub extra_images: HashMap<String, extra_images::ExtraImage>,
    #[serde(rename = "jewelSlots")]
    pub jewel_slots: Vec<i32>,
    pub min_x: i32,
    pub min_y: i32,
    pub max_x: i32,
    pub max_y: i32,

    pub assets: HashMap<String, assets::Asset>,
    pub constants: constants::Constant,

    #[serde(rename = "skillSprites")]
    pub skill_sprites: skill_sprites::SkillSprite,

    #[serde(rename = "imageZoomLevels")]
    pub image_zoom_levels: Vec<f32>,
}
