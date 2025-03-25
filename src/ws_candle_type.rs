#[derive(Debug, Clone, Copy)]
pub enum WsCandleType {
    Minute,
    Hour,
    Day,
    Month,
}

impl WsCandleType {
    pub fn as_str(&self) -> &str {
        match self {
            WsCandleType::Minute => crate::consts::MINUTE,
            WsCandleType::Hour => crate::consts::HOUR,
            WsCandleType::Day => crate::consts::DAY,
            WsCandleType::Month => crate::consts::MONTH,
        }
    }

    pub fn from_str(src: &str) -> Self {
        match src {
            crate::consts::MINUTE => Self::Minute,
            crate::consts::HOUR => Self::Hour,
            crate::consts::DAY => Self::Day,
            crate::consts::MONTH => Self::Month,
            _ => {
                panic!("Unknown candle type: {}", src);
            }
        }
    }
}
