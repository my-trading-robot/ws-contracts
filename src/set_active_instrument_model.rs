use serde::{Deserialize, Serialize};

//id - empty - means global

#[derive(Serialize, Deserialize, Debug)]
pub struct SetActiveInstrumentWsModel {
    pub id: String,
}
