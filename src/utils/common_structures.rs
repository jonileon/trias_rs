use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct LocationRef {
    #[serde(rename = "StopPointRef")]
    pub id: String
}

#[derive(Deserialize, Debug, Clone)]
pub struct TriasString {
    #[serde(rename = "Text")]
    pub text: String,
    #[serde(rename = "Language")]
    pub lang: String
}

#[derive(Deserialize, Debug, Clone)]
pub struct Mode {
    #[serde(rename = "PtMode")]
    pub pt_mode: String,
    #[serde(rename = "Name")]
    pub name: TriasString,
}

