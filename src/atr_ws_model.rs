use serde::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct AtrWsModel {
    pub id: String,
    pub tp: String,
    pub atr: f64,
}
