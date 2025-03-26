use rust_extensions::sorted_vec::EntityWithKey;
use serde::*;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CandleWsModel {
    pub d: u64,
    pub o: f64,
    pub c: f64,
    pub h: f64,
    pub l: f64,
    pub v: f64,
}

impl CandleWsModel {
    pub fn merge(&mut self, other: &Self) {
        if self.h < other.h {
            self.h = other.h;
        }

        if self.l > other.l {
            self.l = other.l;
        }

        self.v += other.v;
        self.c = other.c;
    }
}

impl EntityWithKey<u64> for CandleWsModel {
    fn get_key(&self) -> &u64 {
        &self.d
    }
}
