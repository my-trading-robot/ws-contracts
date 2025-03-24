use rust_extensions::sorted_vec::EntityWithKey;
use serde::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CandleWsModel {
    pub d: u64,
    pub o: f64,
    pub c: f64,
    pub h: f64,
    pub l: f64,
    pub v: f64,
}

impl EntityWithKey<u64> for CandleWsModel {
    fn get_key(&self) -> &u64 {
        &self.d
    }
}
