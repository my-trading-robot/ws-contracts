use rust_extensions::date_time::*;

#[derive(Debug, Clone, Copy)]
pub enum WsCandleType {
    Minute,
    Min5,
    Hour,
    Day,
    Month,
}

impl WsCandleType {
    pub fn to_date_time(&self, value: i64) -> DateTimeAsMicroseconds {
        match self {
            WsCandleType::Minute => {
                let interval: IntervalKey<MinuteKey> = value.try_into().unwrap();
                return interval.try_to_date_time().unwrap();
            }
            WsCandleType::Min5 => {
                let interval: IntervalKey<Minute5Key> = value.try_into().unwrap();
                return interval.try_to_date_time().unwrap();
            }
            WsCandleType::Hour => {
                let interval: IntervalKey<HourKey> = value.try_into().unwrap();
                return interval.try_to_date_time().unwrap();
            }
            WsCandleType::Day => {
                let interval: IntervalKey<DayKey> = value.try_into().unwrap();
                return interval.try_to_date_time().unwrap();
            }
            WsCandleType::Month => {
                let interval: IntervalKey<MonthKey> = value.try_into().unwrap();
                return interval.try_to_date_time().unwrap();
            }
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            WsCandleType::Minute => crate::consts::MINUTE,
            WsCandleType::Min5 => crate::consts::MIN_5,
            WsCandleType::Hour => crate::consts::HOUR,
            WsCandleType::Day => crate::consts::DAY,
            WsCandleType::Month => crate::consts::MONTH,
        }
    }

    pub fn from_str(src: &str) -> Self {
        match src {
            crate::consts::MINUTE => Self::Minute,
            crate::consts::MIN_5 => Self::Min5,
            crate::consts::HOUR => Self::Hour,
            crate::consts::DAY => Self::Day,
            crate::consts::MONTH => Self::Month,
            _ => {
                panic!("Unknown candle type: {}", src);
            }
        }
    }

    pub fn is_min5(&self) -> bool {
        match self {
            WsCandleType::Min5 => true,
            _ => false,
        }
    }

    pub fn is_min(&self) -> bool {
        match self {
            WsCandleType::Minute => true,
            _ => false,
        }
    }

    pub fn is_hour(&self) -> bool {
        match self {
            WsCandleType::Hour => true,
            _ => false,
        }
    }

    pub fn is_day(&self) -> bool {
        match self {
            WsCandleType::Day => true,
            _ => false,
        }
    }

    pub fn is_month(&self) -> bool {
        match self {
            WsCandleType::Month => true,
            _ => false,
        }
    }
}
