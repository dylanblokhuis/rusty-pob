use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Node {
    pub skill: Option<i32>,
    pub name: Option<String>,
    pub icon: Option<String>,
    #[serde(rename = "ascendancyName")]
    pub ascendancy_name: Option<String>,
    pub stats: Option<Vec<String>>,

    pub group: Option<i32>,
    pub orbit: Option<i32>,
    #[serde(rename = "orbitIndex")]
    pub orbit_index: Option<i32>,
    pub out: Option<Vec<String>>,
    // "in" is a keyword in rust
    #[serde(rename = "in")]
    pub _in: Option<Vec<String>>,
}
