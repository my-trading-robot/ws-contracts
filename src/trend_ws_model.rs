use serde::*;

//tp - empty - means global

#[derive(Serialize, Deserialize, Debug)]
pub struct TrendWsModel {
    pub id: String,
    pub tp: String,
    pub value: String,
}
