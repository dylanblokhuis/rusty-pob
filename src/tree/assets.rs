use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Asset {
    #[serde(rename = "0.1246")]
    pub small: Option<String>,
    #[serde(rename = "0.2109")]
    pub medium: Option<String>,
    #[serde(rename = "0.2972")]
    pub large: Option<String>,
    #[serde(rename = "0.3835")]
    pub extra_large: Option<String>,

    #[serde(rename = "1")]
    pub full: Option<String>,
}
