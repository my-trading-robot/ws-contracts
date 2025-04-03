use serde::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct NearLevelWsModel {
    pub id: String,
    pub tp: String,
}
