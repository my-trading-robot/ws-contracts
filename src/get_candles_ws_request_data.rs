use serde::*;

use super::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct GetCandlesWsRequestContract {
    pub req_id: i64,
    pub instrument_id: String,
    pub amount: u64,
    pub max_key: i64,
    pub candle_type: String,
    pub is_bid: bool,
}

impl GetCandlesWsRequestContract {
    pub fn get_candle_type(&self) -> WsCandleType {
        WsCandleType::from_str(&self.candle_type)
    }
}

impl Into<WsContract> for GetCandlesWsRequestContract {
    fn into(self) -> WsContract {
        WsContract::GetCandlesRequest(self)
    }
}
