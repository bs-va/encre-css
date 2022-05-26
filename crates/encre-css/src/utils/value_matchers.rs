use color_name::Color;
use lazy_static::lazy_static;
use regex::Regex;

const LENGTH_UNITS: [&str; 16] = [
    "cm", "mm", "Q", "in", "pc", "pt", "px", "em", "ex", "ch", "rem", "lh", "vw", "vh", "vmin",
    "vmax",
];
const LINE_WIDTHS: [&str; 3] = ["thin", "medium", "thick"];
const GRADIENT_TYPES: [&str; 5] = [
    "linear-gradient",
    "radial-gradient",
    "repeating-linear-gradient",
    "repeating-radial-gradient",
    "conic-gradient",
];
const SHADOW_KEYWORDS: [&str; 5] = ["inset", "inherit", "initial", "revert", "unset"];
const VALID_POSITIONS: [&str; 5] = ["center", "top", "right", "bottom", "left"];
const GENERIC_NAMES: [&str; 13] = [
    "serif",
    "sans-serif",
    "monospace",
    "cursive",
    "fantasy",
    "system-ui",
    "ui-serif",
    "ui-sans-serif",
    "ui-monospace",
    "ui-rounded",
    "math",
    "emoji",
    "fangsong",
];
const ABSOLUTE_SIZES: [&str; 8] = [
    "xx-small",
    "x-small",
    "small",
    "medium",
    "large",
    "x-large",
    "x-large",
    "xxx-large",
];
const RELATIVE_SIZES: [&str; 2] = ["larger", "smaller"];

lazy_static! {
    static ref COLOR_REGEX: Regex =
        Regex::new(r"(?-u)^(#[a-f\d]{3}|#[a-f\d]{6}|rgba?\(([\d,_\.]+|var\(--.+\))[,_]([\d,_\.]+|var\(--.+\))[,_]([\d,_\.]+|var\(--.+\))((_+)?/(_+)?([\d_\.]+|var\(--.+\)))?\)|hsla?\(([\d_\.]+|var\(--.+\))(deg|rad|grad|turn)?[,_]([\d_\.%]+|var\(--.+\))[,_]([\d_\.%]+|var\(--.+\))([,_]([\d_\.%]+|var\(--.+\)))?((_+)?/(_+)?([\d_\.]+|var\(--.+\)))?\))$")
            .unwrap();
    static ref LENGTH_REGEX: Regex =
        Regex::new(&format!("(?-u)(?:{})$", LENGTH_UNITS.join("|"))).unwrap();
    static ref TIME_REGEX: Regex = Regex::new(r"(?-u)\d+m?s$").unwrap();
    static ref NUMBER_CSS_FUNCTIONS_REGEXES: [Regex; 4] = [
        Regex::new(r"^min\(.+?").unwrap(),
        Regex::new(r"^max\(.+?").unwrap(),
        Regex::new(r"^clamp\(.+?").unwrap(),
        Regex::new(r"^calc\(.+?").unwrap(),
    ];
    static ref PERCENTAGE_CSS_FUNCTIONS_REGEXES: [Regex; 4] = [
        Regex::new(r"^min\(.+?%").unwrap(),
        Regex::new(r"^max\(.+?%").unwrap(),
        Regex::new(r"^clamp\(.+?%").unwrap(),
        Regex::new(r"^calc\(.+?%").unwrap(),
    ];
    static ref LENGTH_CSS_FUNCTIONS_REGEXES: [Regex; 4] = [
        Regex::new(&format!(r"^min\(.+?(?:{})", LENGTH_UNITS.join("|"))).unwrap(),
        Regex::new(&format!(r"^max\(.+?(?:{})", LENGTH_UNITS.join("|"))).unwrap(),
        Regex::new(&format!(r"^clamp\(.+?(?:{})", LENGTH_UNITS.join("|"))).unwrap(),
        Regex::new(&format!(r"^calc\(.+?(?:{})", LENGTH_UNITS.join("|"))).unwrap(),
    ];
}

// TODO: Support:
// - global values like inherit, initial, revert, revert-layer, unset
// - intrinsic sizing keywords: fill, max-content, min-content, fit-content

pub fn is_matching_all(_val: &str) -> bool {
    true
}

pub fn is_matching_url(val: &str) -> bool {
    val.starts_with("url(")
}

pub fn is_matching_var(val: &str) -> bool {
    val.starts_with("var(")
}

pub fn is_matching_color(val: &str) -> bool {
    COLOR_REGEX.is_match(val)
        || !val.is_empty() && Color::val().by_string(val.to_string()).is_ok()
        || is_matching_var(val)
}

pub fn is_matching_length(val: &str) -> bool {
    val.split('_').all(|v| {
        v == "0"
            || LENGTH_REGEX.is_match(v)
            || LENGTH_CSS_FUNCTIONS_REGEXES.iter().any(|r| r.is_match(val))
            || is_matching_percentage(val)
    })
}

pub fn is_matching_number(val: &str) -> bool {
    val.parse::<usize>().is_ok() || NUMBER_CSS_FUNCTIONS_REGEXES.iter().any(|r| r.is_match(val))
}

pub fn is_matching_float(val: &str) -> bool {
    val.parse::<f32>().is_ok() || NUMBER_CSS_FUNCTIONS_REGEXES.iter().any(|r| r.is_match(val))
}

pub fn is_matching_percentage(val: &str) -> bool {
    val.ends_with('%')
        || PERCENTAGE_CSS_FUNCTIONS_REGEXES
            .iter()
            .any(|r| r.is_match(val))
}

pub fn is_matching_time(val: &str) -> bool {
    TIME_REGEX.is_match(val)
}

pub fn is_matching_shadow(val: &str) -> bool {
    let mut in_parenthesis = false;
    let mut current_part = String::new();
    let mut part_index = 0;

    for c in val.chars() {
        match c {
            '(' => {
                current_part.push('(');
                in_parenthesis = true;
            }
            ')' => {
                current_part.push(')');
                in_parenthesis = false;
            }
            '_' if !in_parenthesis => {
                if !(part_index == 0 && SHADOW_KEYWORDS.contains(&current_part.as_str()))
                    && (part_index > 4
                        || !is_matching_length(&current_part)
                            && (part_index >= 2 && !is_matching_color(&current_part)))
                {
                    return false;
        }

                part_index += 1;
                current_part.clear();
            }
            ',' if !in_parenthesis => {
                if !(part_index == 0 && SHADOW_KEYWORDS.contains(&current_part.as_str()))
                    && (part_index > 4
                        || !is_matching_length(&current_part)
                            && (part_index >= 2 && !is_matching_color(&current_part)))
                {
                    return false;
        }

                current_part.clear();
                part_index = 0;
            }
            other => {
                current_part.push(other);
        }
            }
        }

    if !current_part.is_empty() {
        // Handle the last part (not suffixed by `_`)
        if !(part_index == 0 && SHADOW_KEYWORDS.contains(&current_part.as_str()))
            && (part_index > 4
                || !is_matching_length(&current_part)
                    && (part_index >= 2 && !is_matching_color(&current_part)))
        {
            return false;
            }
        }

    true
}

pub fn is_matching_gradient(val: &str) -> bool {
    GRADIENT_TYPES.iter().any(|t| val.starts_with(t))
}

pub fn is_matching_position(val: &str) -> bool {
    VALID_POSITIONS.contains(&val)
        || is_matching_length(val)
        || is_matching_percentage(val)
        || is_matching_var(val)
}

pub fn is_matching_line_width(val: &str) -> bool {
    LINE_WIDTHS.contains(&val)
}

pub fn is_matching_generic_name(val: &str) -> bool {
    GENERIC_NAMES.contains(&val)
}

pub fn is_matching_absolute_size(val: &str) -> bool {
    ABSOLUTE_SIZES.contains(&val)
}

pub fn is_matching_relative_size(val: &str) -> bool {
    RELATIVE_SIZES.contains(&val)
}

pub fn is_matching_image(val: &str) -> bool {
    val.split(',').all(|v| {
        is_matching_var(v)
            || is_matching_url(v)
            || is_matching_gradient(v)
            || ["element(", "image(", "cross-fade(", "image-set("]
                .iter()
                .any(|e| v.starts_with(e))
    })
}
