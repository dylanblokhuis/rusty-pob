use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct SkillSprite {
    #[serde(rename = "normalActive")]
    normal_active: Vec<Sprite>,
    #[serde(rename = "notableActive")]
    notable_active: Vec<Sprite>,
    #[serde(rename = "keystoneActive")]
    keystone_active: Vec<Sprite>,
    #[serde(rename = "normalInactive")]
    normal_inactive: Vec<Sprite>,
    #[serde(rename = "notableInactive")]
    notable_inactive: Vec<Sprite>,
    #[serde(rename = "keystoneInactive")]
    keystone_inactive: Vec<Sprite>,
    #[serde(rename = "masteryConnected")]
    mastery_connected: Vec<Sprite>,
    #[serde(rename = "masteryActiveSelected")]
    mastery_active_selected: Vec<Sprite>,
    #[serde(rename = "masteryInactive")]
    mastery_inactive: Vec<Sprite>,
    #[serde(rename = "masteryActiveEffect")]
    mastery_active_effect: Vec<Sprite>,
}

#[derive(Serialize, Deserialize)]
struct Sprite {
    filename: String,
    coords: HashMap<String, Coords>,
}

#[derive(Serialize, Deserialize)]
struct Coords {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}
