use serde::{Deserialize, Serialize};

use crate::WsContract;

//tp - empty - means global

#[derive(Serialize, Deserialize, Debug)]
pub struct PriceLevelWsModel {
    pub id: String,
    pub rate: f64,
    pub tp: String,
}

impl PriceLevelWsModel {
    pub fn into_set_price_level(self) -> WsContract {
        WsContract::SetPriceLevel(self)
    }

    pub fn into_delete_price_level(self) -> WsContract {
        WsContract::DeletePriceLevel(self)
    }
}

impl Into<WsContract> for Vec<PriceLevelWsModel> {
    fn into(self) -> WsContract {
        WsContract::PriceLevels(self)
    }
}
