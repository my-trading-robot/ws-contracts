use serde::*;

#[derive(Debug, Clone, Copy)]
pub enum WsPatternType {
    LimitBuyer,
    LimitSeller,
}

impl WsPatternType {
    pub fn from_str(src: &str) -> Option<Self> {
        match src {
            "LimitBuyer" => Self::LimitBuyer.into(),
            "LimitSeller" => Self::LimitSeller.into(),
            _ => None,
        }
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::LimitBuyer => "LimitBuyer",
            Self::LimitSeller => "LimitSeller",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PatternWsModel {
    pub instr_id: String,
    pub candle_type: String,
    pub pattern: String,
}

impl PatternWsModel {
    pub fn try_get_pattern(&self) -> Option<WsPatternType> {
        WsPatternType::from_str(&self.pattern)
    }
}
