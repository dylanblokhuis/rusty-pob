use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Class {
    name: String,
    base_str: i32,
    base_dex: i32,
    base_int: i32,
    ascendancies: Vec<ClassAscendancy>,
}

#[derive(Serialize, Deserialize)]
struct ClassAscendancy {
    id: String,
    name: String,
    #[serde(rename = "flavourText")]
    flavour_text: Option<String>,
    #[serde(rename = "flavourTextColour")]
    flavour_text_colour: Option<String>,
    #[serde(rename = "flavourTextRect")]
    flavour_text_rect: Option<FlavourTextRect>,
}

#[derive(Serialize, Deserialize)]
struct FlavourTextRect {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}
