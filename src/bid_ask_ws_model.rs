use serde::*;

#[derive(Serialize, Deserialize)]
pub struct BidAskWsModel {
    pub id: String,
    pub bid: f64,
    pub ask: f64,
}
