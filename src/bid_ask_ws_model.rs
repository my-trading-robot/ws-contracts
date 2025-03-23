use serde::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct BidAskWsModel {
    pub id: String,
    pub bid: f64,
    pub ask: f64,
}
