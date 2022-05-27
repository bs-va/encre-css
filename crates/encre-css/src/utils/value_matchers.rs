use lazy_static::lazy_static;
use regex::Regex;

pub const LENGTH_UNITS: [&str; 16] = [
    "cm", "mm", "Q", "in", "pc", "pt", "px", "em", "ex", "ch", "rem", "lh", "vw", "vh", "vmin",
    "vmax",
];
pub const LINE_WIDTHS: [&str; 3] = ["thin", "medium", "thick"];
pub const GRADIENT_TYPES: [&str; 5] = [
    "linear-gradient",
    "radial-gradient",
    "repeating-linear-gradient",
    "repeating-radial-gradient",
    "conic-gradient",
];
pub const SHADOW_KEYWORDS: [&str; 5] = ["inset", "inherit", "initial", "revert", "unset"];
pub const VALID_POSITIONS: [&str; 5] = ["center", "top", "right", "bottom", "left"];
pub const GENERIC_NAMES: [&str; 13] = [
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
pub const ABSOLUTE_SIZES: [&str; 8] = [
    "xx-small",
    "x-small",
    "small",
    "medium",
    "large",
    "x-large",
    "x-large",
    "xxx-large",
];
pub const RELATIVE_SIZES: [&str; 2] = ["larger", "smaller"];
pub const NAMED_COLORS: [&str; 148] = [
    "antiquewhite",
    "aliceblue",
    "aqua",
    "aquamarine",
    "azure",
    "beige",
    "bisque",
    "black",
    "blanchedalmond",
    "blue",
    "blueviolet",
    "brown",
    "burlywood",
    "cadetblue",
    "chartreuse",
    "chocolate",
    "coral",
    "cornflowerblue",
    "cornsilk",
    "crimson",
    "cyan",
    "darkblue",
    "darkcyan",
    "darkgoldenrod",
    "darkgray",
    "darkgreen",
    "darkgrey",
    "darkkhaki",
    "darkmagenta",
    "darkolivegreen",
    "darkorange",
    "darkorchid",
    "darkred",
    "darksalmon",
    "darkseagreen",
    "darkslateblue",
    "darkslategray",
    "darkslategrey",
    "darkturquoise",
    "darkviolet",
    "deeppink",
    "deepskyblue",
    "dimgray",
    "dimgrey",
    "dodgerblue",
    "firebrick",
    "floralwhite",
    "forestgreen",
    "fuchsia",
    "gainsboro",
    "ghostwhite",
    "gold",
    "goldenrod",
    "gray",
    "green",
    "greenyellow",
    "grey",
    "honeydew",
    "hotpink",
    "indianred",
    "indigo",
    "ivory",
    "khaki",
    "lavender",
    "lavenderblush",
    "lawngreen",
    "lemonchiffon",
    "lightblue",
    "lightcoral",
    "lightcyan",
    "lightgoldenrodyellow",
    "lightgray",
    "lightgreen",
    "lightgrey",
    "lightpink",
    "lightsalmon",
    "lightseagreen",
    "lightskyblue",
    "lightslategray",
    "lightslategrey",
    "lightsteelblue",
    "lightyellow",
    "lime",
    "limegreen",
    "linen",
    "magenta",
    "maroon",
    "mediumaquamarine",
    "mediumblue",
    "mediumorchid",
    "mediumpurple",
    "mediumseagreen",
    "mediumslateblue",
    "mediumspringgreen",
    "mediumturquoise",
    "mediumvioletred",
    "midnightblue",
    "mintcream",
    "mistyrose",
    "moccasin",
    "navajowhite",
    "navy",
    "oldlace",
    "olive",
    "olivedrab",
    "orange",
    "orangered",
    "orchid",
    "palegoldenrod",
    "palegreen",
    "paleturquoise",
    "palevioletred",
    "papayawhip",
    "peachpuff",
    "peru",
    "pink",
    "plum",
    "powderblue",
    "purple",
    "rebeccapurple",
    "red",
    "rosybrown",
    "royalblue",
    "saddlebrown",
    "salmon",
    "sandybrown",
    "seagreen",
    "seashell",
    "sienna",
    "silver",
    "skyblue",
    "slateblue",
    "slategray",
    "slategrey",
    "snow",
    "springgreen",
    "steelblue",
    "tan",
    "teal",
    "thistle",
    "tomato",
    "turquoise",
    "violet",
    "wheat",
    "white",
    "whitesmoke",
    "yellow",
    "yellowgreen",
];

lazy_static! {
    static ref COLOR_REGEX: Regex =
        Regex::new(r"^(#[a-f\d]{3}|#[a-f\d]{6}|rgba?\(([\d,_\.]+|var\(--.+\))[,_]([\d,_\.]+|var\(--.+\))[,_]([\d,_\.]+|var\(--.+\))((_+)?/(_+)?([\d_\.]+|var\(--.+\)))?\)|hsla?\(([\d_\.]+|var\(--.+\))(deg|rad|grad|turn)?[,_]([\d_\.%]+|var\(--.+\))[,_]([\d_\.%]+|var\(--.+\))([,_]([\d_\.%]+|var\(--.+\)))?((_+)?/(_+)?([\d_\.]+|var\(--.+\)))?\))$")
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
        || !val.is_empty() && NAMED_COLORS.iter().any(|c| &val == c)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_matching_url_test() {
        assert!(is_matching_url("url('/hello/world.png')"));
    }

    #[test]
    fn is_matching_var_test() {
        assert!(is_matching_var("var(--bg-blue)"));
    }

    #[test]
    fn is_matching_color_test() {
        assert!(is_matching_color("blue"));
        assert!(is_matching_color("#333"));
        assert!(is_matching_color("#121212"));
        assert!(is_matching_color("rgb(12.12,12,12)"));
        assert!(is_matching_color("rgb(12_12_12)"));
        assert!(is_matching_color("rgb(12_12_12/0.1)"));
        assert!(is_matching_color("rgb(12_12_12_/_0.1)"));
        assert!(is_matching_color("rgb(var(--blue),12,12)"));
        assert!(is_matching_color("rgb(12_12_12_/_var(--opacity))"));
        assert!(is_matching_color("rgba(12,12,12,0.12)"));
        assert!(is_matching_color("hsl(360,100%,50%)"));
        assert!(is_matching_color("hsl(3.14rad,100%,50%)"));
        assert!(is_matching_color("hsl(3.14rad_100%_50%/0.42)"));
        assert!(is_matching_color("hsl(var(--hue)_12%_42%/var(--opacity))"));
        assert!(is_matching_color("hsla(360,100%,50%,0.12)"));
    }

    #[test]
    fn is_matching_length_test() {
        assert!(is_matching_length("300px"));
        assert!(is_matching_length("50%"));
        assert!(is_matching_length("30vw"));
        assert!(is_matching_length("min(10%,10px)"));
        assert!(is_matching_length("0"));
    }

    #[test]
    fn is_matching_number_test() {
        assert!(is_matching_number("12"));
        assert!(!is_matching_number("42.12"));
    }

    #[test]
    fn is_matching_float_test() {
        assert!(is_matching_float("42.12"));
    }

    #[test]
    fn is_matching_percentage_test() {
        assert!(is_matching_percentage("10%"));
    }

    #[test]
    fn is_matching_time_test() {
        assert!(is_matching_time("0.5s"));
        assert!(is_matching_time("10ms"));
    }

    #[test]
    fn is_matching_shadow_with_functions_test() {
        assert!(is_matching_shadow("10px_10px_min(1px,2px)_10px_rgb(1,1,1)"));
        assert!(is_matching_shadow("inset_0_-3em_3em_rgba(0,0,0,0.1),0_0_0_2px_rgb(255,255,255),0.3em_0.3em_1em_rgba(0,0,0,0.3)"));
        assert!(is_matching_shadow(
            "var(--a,_0_0_1px_rgb(0,_0,_0)),_0_0_1px_rgb(0,_0,_0)"
        ));
    }

    #[test]
    fn is_matching_gradient_test() {
        assert!(is_matching_gradient("linear-gradient(45deg, blue, red);"));
    }

    #[test]
    fn is_matching_position_test() {
        assert!(is_matching_position("right"));
        assert!(is_matching_position("12px"));
        assert!(is_matching_position("42%"));
    }

    #[test]
    fn is_matching_line_width_test() {
        assert!(is_matching_line_width("thin"));
    }

    #[test]
    fn is_matching_generic_name_test() {
        assert!(is_matching_generic_name("sans-serif"));
        assert!(is_matching_generic_name("fantasy"));
    }

    #[test]
    fn is_matching_absolute_size_test() {
        assert!(is_matching_absolute_size("xx-small"));
    }

    #[test]
    fn is_matching_relative_size_test() {
        assert!(is_matching_relative_size("larger"));
    }
}
