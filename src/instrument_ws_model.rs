use serde::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct InstrumentWsModel {
    pub id: String,
    pub accuracy: i64,
}
