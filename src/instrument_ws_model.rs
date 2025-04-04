use serde::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct InstrumentWsModel {
    pub id: String,
    pub accuracy: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub levels_amount: Option<usize>,
}
