use serde::*;

#[derive(Debug, Clone, Copy)]
pub enum WsPatternType {
    SignalLimitBuyer,
    SignalLimitSeller,
}

impl WsPatternType {
    pub fn from_str(src: &str) -> Option<Self> {
        match src {
            "SignalLimitBuyer" => Self::SignalLimitBuyer.into(),
            "SignalLimitSeller" => Self::SignalLimitSeller.into(),
            _ => None,
        }
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SignalLimitBuyer => "SignalLimitBuyer",
            Self::SignalLimitSeller => "SignalLimitSeller",
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
