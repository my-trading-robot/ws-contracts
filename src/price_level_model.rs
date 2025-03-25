use serde::{Deserialize, Serialize};

use crate::WsContract;

//tp - empty - means global

#[derive(Serialize, Deserialize, Debug)]
pub struct PriceLevelWsModel {
    pub id: String,
    pub rate: f64,
    pub tp: String,
}

impl Into<WsContract> for PriceLevelWsModel {
    fn into(self) -> WsContract {
        WsContract::SetPriceLevel(self)
    }
}

impl Into<WsContract> for Vec<PriceLevelWsModel> {
    fn into(self) -> WsContract {
        WsContract::PriceLevels(self)
    }
}
