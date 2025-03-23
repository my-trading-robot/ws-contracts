pub const MINUTE: &'static str = "1m";
pub const HOUR: &'static str = "1h";
pub const DAY: &'static str = "1d";
pub const MONTH: &'static str = "1M";
pub enum WsCandleType {
    Minute,
    Hour,
    Day,
    Month,
}

impl WsCandleType {
    pub fn as_str(&self) -> &str {
        match self {
            WsCandleType::Minute => MINUTE,
            WsCandleType::Hour => HOUR,
            WsCandleType::Day => DAY,
            WsCandleType::Month => MONTH,
        }
    }

    pub fn from_str(src: &str) -> Self {
        match src {
            MINUTE => Self::Minute,
            HOUR => Self::Hour,
            DAY => Self::Day,
            MONTH => Self::Month,
            _ => {
                panic!("Unknown candle type: {}", src);
            }
        }
    }
}
