pub fn validate_candle_type(str: &str) -> bool {
    if str == crate::consts::MINUTE {
        return true;
    }

    if str == crate::consts::HOUR {
        return true;
    }

    if str == crate::consts::MONTH {
        return true;
    }

    if str == crate::consts::DAY {
        return true;
    }

    false
}

pub fn validate_price_level_type(str: &str) -> bool {
    if str == crate::consts::MINUTE {
        return true;
    }

    if str == crate::consts::HOUR {
        return true;
    }

    if str == crate::consts::MONTH {
        return true;
    }

    if str == crate::consts::DAY {
        return true;
    }

    if str == crate::consts::GLOBAL_LEVEL {
        return true;
    }

    false
}
