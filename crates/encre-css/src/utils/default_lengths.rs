use std::borrow::Cow;

pub fn get_basic(val: &str, is_negative: bool) -> Option<Cow<str>> {
    let absolute_result = match val {
        "px" => Some("1px"),
        "0" => Some("0px"),
        "0.5" => Some("0.125rem"),
        "1" => Some("0.25rem"),
        "1.5" => Some("0.375rem"),
        "2" => Some("0.5rem"),
        "2.5" => Some("0.625rem"),
        "3" => Some("0.75rem"),
        "3.5" => Some("0.875rem"),
        "4" => Some("1rem"),
        "5" => Some("1.25rem"),
        "6" => Some("1.5rem"),
        "7" => Some("1.75rem"),
        "8" => Some("2rem"),
        "9" => Some("2.25rem"),
        "10" => Some("2.5rem"),
        "11" => Some("2.75rem"),
        "12" => Some("3rem"),
        "14" => Some("3.5rem"),
        "16" => Some("4rem"),
        "20" => Some("5rem"),
        "24" => Some("6rem"),
        "28" => Some("7rem"),
        "32" => Some("8rem"),
        "36" => Some("9rem"),
        "40" => Some("10rem"),
        "44" => Some("11rem"),
        "48" => Some("12rem"),
        "52" => Some("13rem"),
        "56" => Some("14rem"),
        "60" => Some("15rem"),
        "64" => Some("16rem"),
        "72" => Some("18rem"),
        "80" => Some("20rem"),
        "96" => Some("24rem"),
        _ => return None,
    };

    if is_negative {
        Some(Cow::from("-".to_string() + absolute_result.unwrap()))
    } else {
        Some(Cow::from(absolute_result.unwrap()))
    }
}

pub fn get_fraction(val: &str, is_negative: bool) -> Option<Cow<str>> {
    let absolute_result = match val {
        "1/2" => Some("50%"),
        "1/3" => Some("33.333333%"),
        "2/3" => Some("66.666667%"),
        "1/4" => Some("25%"),
        "2/4" => Some("50%"),
        "3/4" => Some("75%"),
        "1/5" => Some("20%"),
        "2/5" => Some("40%"),
        "3/5" => Some("60%"),
        "4/5" => Some("80%"),
        "1/6" => Some("16.666667%"),
        "2/6" => Some("33.333333%"),
        "3/6" => Some("50%"),
        "4/6" => Some("66.666667%"),
        "5/6" => Some("83.333333%"),
        "1/12" => Some("8.333333%"),
        "2/12" => Some("16.666667%"),
        "3/12" => Some("25%"),
        "4/12" => Some("33.333333%"),
        "5/12" => Some("41.666667%"),
        "6/12" => Some("50%"),
        "7/12" => Some("58.333333%"),
        "8/12" => Some("66.666667%"),
        "9/12" => Some("75%"),
        "10/12" => Some("83.333333%"),
        "11/12" => Some("91.666667%"),
        _ => return None,
    };

    if is_negative {
        Some(Cow::from("-".to_string() + absolute_result.unwrap()))
    } else {
        Some(Cow::from(absolute_result.unwrap()))
    }
}

pub fn get_keyword_size(val: &str) -> Option<Cow<str>> {
    match val {
        "min" => Some(Cow::from("min-content")),
        "max" => Some(Cow::from("max-content")),
        _ => None,
    }
}

pub fn get_keyword(val: &str) -> Option<Cow<str>> {
    match val {
        "auto" => Some(Cow::from("auto")),
        "full" => Some(Cow::from("100%")),
        _ => None,
    }
}

pub fn get_extended(val: &str, is_negative: bool) -> Option<Cow<str>> {
    get_keyword(val)
        .or_else(|| get_basic(val, is_negative).or_else(|| get_fraction(val, is_negative)))
}

pub fn get_extended_size(val: &str, is_negative: bool) -> Option<Cow<str>> {
    get_keyword(val).or_else(|| {
        get_keyword_size(val)
            .or_else(|| get_basic(val, is_negative).or_else(|| get_fraction(val, is_negative)))
    })
}
