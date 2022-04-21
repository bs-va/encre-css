// TODO: Negative values
pub fn get_basic(val: &str) -> Option<String> {
    match val {
        "px" => Some("1px".to_string()),
        "0" => Some("0px".to_string()),
        "0.5" => Some("0.125rem".to_string()),
        "1" => Some("0.25rem".to_string()),
        "1.5" => Some("0.375rem".to_string()),
        "2" => Some("0.5rem".to_string()),
        "2.5" => Some("0.625rem".to_string()),
        "3" => Some("0.75rem".to_string()),
        "3.5" => Some("0.875rem".to_string()),
        "4" => Some("1rem".to_string()),
        "5" => Some("1.25rem".to_string()),
        "6" => Some("1.5rem".to_string()),
        "7" => Some("1.75rem".to_string()),
        "8" => Some("2rem".to_string()),
        "9" => Some("2.25rem".to_string()),
        "10" => Some("2.5rem".to_string()),
        "11" => Some("2.75rem".to_string()),
        "12" => Some("3rem".to_string()),
        "14" => Some("3.5rem".to_string()),
        "16" => Some("4rem".to_string()),
        "20" => Some("5rem".to_string()),
        "24" => Some("6rem".to_string()),
        "28" => Some("7rem".to_string()),
        "32" => Some("8rem".to_string()),
        "36" => Some("9rem".to_string()),
        "40" => Some("10rem".to_string()),
        "44" => Some("11rem".to_string()),
        "48" => Some("12rem".to_string()),
        "52" => Some("13rem".to_string()),
        "56" => Some("14rem".to_string()),
        "60" => Some("15rem".to_string()),
        "64" => Some("16rem".to_string()),
        "72" => Some("18rem".to_string()),
        "80" => Some("20rem".to_string()),
        "96" => Some("24rem".to_string()),
        _ => None,
    }
}

pub fn get_fraction(val: &str) -> Option<String> {
    match val {
        "1/2" => Some("50%".to_string()),
        "1/3" => Some("33.333333%".to_string()),
        "2/3" => Some("66.666667%".to_string()),
        "1/4" => Some("25%".to_string()),
        "2/4" => Some("50%".to_string()),
        "3/4" => Some("75%".to_string()),
        "1/5" => Some("20%".to_string()),
        "2/5" => Some("40%".to_string()),
        "3/5" => Some("60%".to_string()),
        "4/5" => Some("80%".to_string()),
        "1/6" => Some("16.666667%".to_string()),
        "2/6" => Some("33.333333%".to_string()),
        "3/6" => Some("50%".to_string()),
        "4/6" => Some("66.666667%".to_string()),
        "5/6" => Some("83.333333%".to_string()),
        "1/12" => Some("8.333333%".to_string()),
        "2/12" => Some("16.666667%".to_string()),
        "3/12" => Some("25%".to_string()),
        "4/12" => Some("33.333333%".to_string()),
        "5/12" => Some("41.666667%".to_string()),
        "6/12" => Some("50%".to_string()),
        "7/12" => Some("58.333333%".to_string()),
        "8/12" => Some("66.666667%".to_string()),
        "9/12" => Some("75%".to_string()),
        "10/12" => Some("83.333333%".to_string()),
        "11/12" => Some("91.666667%".to_string()),
        _ => None,
    }
}

pub fn get_keyword(val: &str) -> Option<String> {
    match val {
        "auto" => Some("auto".to_string()),
        "full" => Some("100%".to_string()),
        "screen" => Some("100vw".to_string()),
        "min" => Some("min-content".to_string()),
        "max" => Some("max-content".to_string()),
        _ => None,
    }
}

pub fn get_extended(val: &str) -> Option<String> {
    get_keyword(val).or_else(|| get_basic(val).or_else(|| get_fraction(val)))
}
