use color_name::Color;
use lazy_static::lazy_static;
use regex::Regex;

const AUTO_KEYWORD: &str = "auto";
const CSS_FUNCTIONS: [&str; 4] = ["min", "max", "clamp", "calc"];
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
        Regex::new(r"^(#[a-f\d]{3}|#[a-f\d]{6}|rgba?\(.+\)|hsla?\(.+\))$").unwrap();
    static ref LENGTH_REGEX: Regex =
        Regex::new(&format!("(?:{})", LENGTH_UNITS.join("|"))).unwrap();
    static ref TIME_REGEX: Regex = Regex::new(r"\d+m?s$").unwrap();
    static ref COMMA: fancy_regex::Regex = fancy_regex::Regex::new(r"\,(?![^(]*\))").unwrap();
}

// TODO: Support:
// - global values like inherit, initial, revert, revert-layer, unset
// - intrinsic sizing keywords: fill, max-content, min-content, fit-content

pub fn is_matching_all(_val: &str) -> bool {
    true
}

pub fn is_matching_auto(val: &str) -> bool {
    val == AUTO_KEYWORD
}

pub fn is_matching_url(val: &str) -> bool {
    val.starts_with("url(")
}

pub fn is_matching_var(val: &str) -> bool {
    val.starts_with("var(")
}

pub fn is_matching_color(val: &str) -> bool {
    COLOR_REGEX.is_match(val)
        || Color::val().by_string(val.to_string()).is_ok()
        || is_matching_var(val)
}

pub fn is_matching_length(val: &str) -> bool {
    val.split('_').all(|v| {
        v == "0"
            // TODO: Static regexes
            || Regex::new(&format!("{}$", *LENGTH_REGEX))
                .unwrap()
                .is_match(v)
            || CSS_FUNCTIONS.iter().any(|f| {
                Regex::new(&format!(r"^{}\(.+?{}", f, *LENGTH_REGEX))
                    .unwrap()
                    .is_match(val)
            })
    })
}

pub fn is_matching_number(val: &str) -> bool {
    val.parse::<usize>().is_ok()
        || CSS_FUNCTIONS
            .iter()
            .any(|f| Regex::new(&format!(r"^{}\(.+?", f)).unwrap().is_match(val))
}

pub fn is_matching_float(val: &str) -> bool {
    val.parse::<f32>().is_ok()
        || CSS_FUNCTIONS
            .iter()
            .any(|f| Regex::new(&format!(r"^{}\(.+?", f)).unwrap().is_match(val))
}

pub fn is_matching_percentage(val: &str) -> bool {
    val.ends_with('%')
        || CSS_FUNCTIONS
            .iter()
            .any(|f| Regex::new(&format!(r"^{}\(.+?%", f)).unwrap().is_match(val))
}

pub fn is_matching_time(val: &str) -> bool {
    TIME_REGEX.is_match(val)
}

pub fn is_matching_shadow(val: &str) -> bool {
    // TODO: \,(?![^(]*\)) -> prevent splitting rgba(12,12,12,0.2)
    val.split(',').all(|shadow| {
        let value = shadow.trim();
        let mut parts = value.split('_');
        let mut is_matching = (false, false, false, false, false);

        if let Some(part) = parts.next() {
            if SHADOW_KEYWORDS.contains(&part) {
                return true;
            } else if is_matching_length(part) {
                is_matching.0 = true;
            }
        }

        if let Some(part) = parts.next() {
            if is_matching_length(part) {
                is_matching.1 = true;
            }
        }

        if let Some(part) = parts.next() {
            if is_matching_length(part) {
                is_matching.2 = true;
            }
        }

        if let Some(part) = parts.next() {
            if is_matching_length(part) {
                is_matching.3 = true;
            }
        }

        if let Some(part) = parts.next() {
            if is_matching_color(part) {
                is_matching.4 = true;
                return true;
            }
        }

        is_matching.0 && is_matching.1 && is_matching.2 && is_matching.3 && is_matching.4
    })
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
