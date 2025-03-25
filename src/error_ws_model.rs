use serde::*;

use crate::WsContract;

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorWsModel {
    pub msg: String,
}

impl Into<WsContract> for ErrorWsModel {
    fn into(self) -> WsContract {
        WsContract::Error(self)
    }
}
