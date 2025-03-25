use serde::{Deserialize, Serialize};

//tp - empty - means global

#[derive(Serialize, Deserialize, Debug)]
pub struct PriceLevelWsModel {
    pub id: String,
    pub rate: f64,
    pub tp: String,
}
