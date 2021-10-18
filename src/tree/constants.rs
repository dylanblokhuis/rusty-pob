use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Constant {
    classes: Classes,

    #[serde(rename = "characterAttributes")]
    character_attributes: CharacterAttributes,
    #[serde(rename = "PSSCentreInnerRadius")]
    psscentre_inner_radius: i32,
    #[serde(rename = "skillsPerOrbit")]
    skills_per_orbit: Vec<i32>,
    #[serde(rename = "orbitRadii")]
    orbit_radii: Vec<i32>,
}

#[derive(Serialize, Deserialize)]
struct Classes {
    #[serde(rename = "StrDexIntClass")]
    str_dex_int_class: i32,
    #[serde(rename = "StrClass")]
    str_class: i32,
    #[serde(rename = "DexClass")]
    dex_class: i32,
    #[serde(rename = "IntClass")]
    int_class: i32,
    #[serde(rename = "StrDexClass")]
    str_dex_class: i32,
    #[serde(rename = "StrIntClass")]
    str_int_class: i32,
    #[serde(rename = "DexIntClass")]
    dex_int_class: i32,
}

#[derive(Serialize, Deserialize)]
struct CharacterAttributes {
    #[serde(rename = "Strength")]
    strength: i32,
    #[serde(rename = "Dexterity")]
    dexterity: i32,
    #[serde(rename = "Intelligence")]
    intelligence: i32,
}
