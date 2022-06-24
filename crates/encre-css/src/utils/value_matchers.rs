pub const LENGTH_UNITS: [&str; 16] = [
    "cm", "mm", "Q", "in", "pc", "pt", "px", "em", "ex", "ch", "rem", "lh", "vw", "vh", "vmin",
    "vmax",
];
pub const LINE_WIDTHS: [&str; 3] = ["thin", "medium", "thick"];
pub const ANGLES: [&str; 4] = ["deg", "grad", "rad", "turn"];
pub const GRADIENT_TYPES: [&str; 5] = [
    "linear-gradient",
    "radial-gradient",
    "repeating-linear-gradient",
    "repeating-radial-gradient",
    "conic-gradient",
];
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

// TODO: Support:
// - global values like inherit, initial, revert, revert-layer, unset
// - intrinsic sizing keywords: fill, max-content, min-content, fit-content

pub fn is_matching_all(_value: &str) -> bool {
    true
}

pub fn is_matching_url(value: &str) -> bool {
    value.starts_with("url(")
}

pub fn is_matching_var(value: &str) -> bool {
    value.starts_with("var(")
}

pub fn is_matching_computational_css_function(value: &str) -> bool {
    value.starts_with("min")
        || value.starts_with("max")
        || value.starts_with("clamp")
        || value.starts_with("calc")
}

pub fn is_matching_color(value: &str) -> bool {
    (value.starts_with('#') && (value.len() == 4 || value.len() == 7))
        || value.starts_with("rgb")
        || value.starts_with("rgba")
        || value.starts_with("hsl")
        || value.starts_with("hsla")
        || NAMED_COLORS.iter().any(|c| &value == c)
        || is_matching_var(value)
}

pub fn is_matching_length(value: &str) -> bool {
    value.split('_').all(|v| {
        v == "0" || LENGTH_UNITS.iter().any(|u| v.ends_with(u)) || is_matching_percentage(v)
    }) || is_matching_var(value)
}

pub fn is_matching_number(value: &str) -> bool {
    value.parse::<usize>().is_ok() || is_matching_computational_css_function(value)
}

pub fn is_matching_float(value: &str) -> bool {
    value.parse::<f32>().is_ok() || is_matching_computational_css_function(value)
}

pub fn is_matching_percentage(value: &str) -> bool {
    value.ends_with('%') || is_matching_computational_css_function(value)
}

pub fn is_matching_time(value: &str) -> bool {
    value.ends_with('s') || value.ends_with("ms")
}

pub fn is_matching_shadow(value: &str) -> bool {
    super::shadow::parse_shadow(&value.replace('_', " ")).is_some()
}

pub fn is_matching_gradient(value: &str) -> bool {
    GRADIENT_TYPES.iter().any(|t| value.starts_with(t))
}

pub fn is_matching_position(value: &str) -> bool {
    VALID_POSITIONS.contains(&value) || is_matching_length(value) || is_matching_percentage(value)
}

pub fn is_matching_line_width(value: &str) -> bool {
    LINE_WIDTHS.contains(&value)
}

pub fn is_matching_angle(value: &str) -> bool {
    ANGLES.iter().any(|a| value.ends_with(a))
}

pub fn is_matching_generic_name(value: &str) -> bool {
    GENERIC_NAMES.contains(&value)
}

pub fn is_matching_absolute_size(value: &str) -> bool {
    ABSOLUTE_SIZES.contains(&value)
}

pub fn is_matching_relative_size(value: &str) -> bool {
    RELATIVE_SIZES.contains(&value)
}

pub fn is_matching_image(value: &str) -> bool {
    value.split(',').all(|v| {
        is_matching_url(v)
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
    fn is_matching_angle_test() {
        assert!(is_matching_angle("0.2turn"));
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
