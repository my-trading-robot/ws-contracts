use serde::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorWsModel {
    pub msg: String,
}
