use serde::*;

use super::*;

#[derive(Serialize, Deserialize)]
pub struct GetCandlesWsResponseContract {
    pub req_id: i64,
    pub data: Vec<CandleWsModel>,
}

impl Into<WsContract> for GetCandlesWsResponseContract {
    fn into(self) -> WsContract {
        WsContract::GetCandlesResponse(self)
    }
}
