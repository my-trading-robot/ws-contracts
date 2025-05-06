use serde::*;

#[derive(Debug, Clone, Copy)]
pub enum PatternType {
    LimitBuyer,
    LimitSeller,
}

impl PatternType {
    pub fn from_str(src: &str) -> Option<Self> {
        match src {
            "LimitBuyer" => Self::LimitBuyer.into(),
            "LimitSeller" => Self::LimitSeller.into(),
            _ => None,
        }
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            PatternType::LimitBuyer => "LimitBuyer",
            PatternType::LimitSeller => "LimitSeller",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PatternWsModel {
    pub instr_id: String,
    pub tp: String,
    pub pattern: String,
}

impl PatternWsModel {
    pub fn try_get_pattern(&self) -> Option<PatternType> {
        PatternType::from_str(&self.pattern)
    }
}
