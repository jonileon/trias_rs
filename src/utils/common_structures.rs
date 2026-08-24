use serde::Serialize;

#[derive(Serialize)]
pub struct LocationRef {
    #[serde(rename = "StopPointRef")]
    pub id: String
}


