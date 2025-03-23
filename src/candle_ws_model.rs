use serde::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct CandleWsModel {
    pub d: u64,
    pub o: f64,
    pub c: f64,
    pub h: f64,
    pub l: f64,
    pub v: f64,
}
