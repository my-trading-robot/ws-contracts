use serde::{Deserialize, Serialize};

use crate::WsContract;

//id - empty - means global

#[derive(Serialize, Deserialize, Debug)]
pub struct SetActiveInstrumentWsModel {
    pub id: String,
}

impl Into<WsContract> for SetActiveInstrumentWsModel {
    fn into(self) -> WsContract {
        WsContract::SetActiveInstrument(self)
    }
}
