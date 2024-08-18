//! Define some color manipulation functions.
use crate::config::{Config, BUILTIN_COLORS};

use std::borrow::Cow;

/// Convert an hexadecimal color to an RGB one.
///
/// # Example
///
/// ```
/// use encre_css::utils::color::hex_to_rgb;
/// assert_eq!(hex_to_rgb("#333").unwrap(), (51, 51, 51));
/// assert_eq!(hex_to_rgb("#f1f1f1").unwrap(), (241, 241, 241));
/// ```
///
/// # Errors
///
/// Returns [`None`] when the hexadecimal color is incorrect.
///
/// [`None`]: Option::None
pub fn hex_to_rgb(mut hex: &str) -> Option<(u8, u8, u8)> {
    // Remove the useless `#` from the start of the color
    hex = hex.strip_prefix('#').unwrap_or(hex);

    if hex.len() == 3 {
        // Support the hexadecimal shorthand
        let r = u8::from_str_radix(&hex[0..1], 16).ok()?;
        let g = u8::from_str_radix(&hex[1..2], 16).ok()?;
        let b = u8::from_str_radix(&hex[2..3], 16).ok()?;

        Some((r + r * 16, g + g * 16, b + b * 16))
    } else if hex.len() == 6 {
        let hex = hex.to_lowercase();

        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;

        Some((r, g, b))
    } else {
        None
    }
}

/// Returns whether the modifier is matching a builtin color. The builtin colors are:
///
/// - `current`, `inherit`, `transparent`, `black`, `white`;
/// - Any key contained in the [`BUILTIN_COLORS`] list.
pub fn is_matching_builtin_color(config: &Config, mut modifier: &str) -> bool {
    if ["current", "inherit", "transparent"].contains(&modifier) {
        return true;
    }

    // Trim the opacity suffix, if present
    if let Some((new_modifier, opacity)) = modifier.split_once('/') {
        if opacity.parse::<usize>().is_err() {
            return false;
        }

        modifier = new_modifier;
    }

    if ["black", "white"].contains(&modifier) {
        return true;
    }

    BUILTIN_COLORS.contains_key(modifier) || config.theme.colors.contains(modifier)
}

/// Get a color from a modifier.
///
/// The third argument is used to set the opacity type used:
///
/// - [`Option::None`] won't use opacity (except if the opacity syntax is used, for example in `bg-red-500/25`);
/// - [`Option::Some`] contains a variable which will be added as the opacity of the color, used to
///   dynamically change the opacity.
pub fn get<'a>(
    config: &Config,
    modifier: &'a str,
    opacity: Option<&'static str>,
) -> Option<Cow<'a, str>> {
    // Handle the new opacity syntax (e.g. `bg-red-500/25`)
    let (mut opacity_from_syntax, modifier) = {
        // The `current` and `inherit` modifiers have static values
        if modifier == "current" {
            return Some(Cow::Borrowed("currentColor"));
        } else if modifier == "inherit" {
            return Some(Cow::Borrowed("inherit"));
        } else if let Some((new_modifier, opacity_suffix)) = modifier.split_once('/') {
            if let Ok(opacity_number) = opacity_suffix.parse::<usize>() {
                #[allow(clippy::cast_precision_loss)]
                (Some(opacity_number as f64 / 100.), new_modifier)
            } else {
                return None;
            }
        } else {
            (None, modifier)
        }
    };

    let rgb_result = if modifier == "transparent" {
        if opacity_from_syntax.is_none() {
            opacity_from_syntax = Some(0.0);
        }

        Some((0, 0, 0))
    } else if modifier == "black" {
        Some((0, 0, 0))
    } else if modifier == "white" {
        Some((0xff, 0xff, 0xff))
    } else if let Some(hex_color) = config.theme.colors.get(modifier) {
        // Custom theme values override builtin colors
        hex_to_rgb(hex_color)
    } else {
        BUILTIN_COLORS.get(modifier).copied()
    };

    // Convert the array to a CSS color with an opacity value (if the color is found)
    rgb_result.map(|rgb_result| {
        Cow::Owned(format!(
            "rgb({} {} {}{})",
            rgb_result.0,
            rgb_result.1,
            rgb_result.2,
            if let Some(opacity_from_syntax) = opacity_from_syntax {
                Cow::Owned(format!(" / {opacity_from_syntax}"))
            } else if let Some(opacity) = opacity {
                Cow::Owned(format!(" / var({opacity})"))
            } else {
                Cow::Borrowed("")
            }
        ))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::Config;

    #[test]
    fn hex_to_rgb_test() {
        assert_eq!(hex_to_rgb(&Cow::Borrowed("#ff0000")).unwrap(), (255, 0, 0));
        assert_eq!(
            hex_to_rgb(&Cow::Borrowed("#FF00FF")).unwrap(),
            (255, 0, 255)
        );
        assert_eq!(hex_to_rgb(&Cow::Borrowed("#332")).unwrap(), (51, 51, 34));
        assert_eq!(hex_to_rgb(&Cow::Borrowed("#FEF")).unwrap(), (255, 238, 255));
        assert!(hex_to_rgb(&Cow::Borrowed("#aaaaaaa")).is_none());
    }

    #[test]
    fn get_color() {
        let mut config = Config::default();
        config.theme.colors.add("rosa-600", "#f23cff");

        assert_eq!(
            get(&config, "red-400", Some("--opacity")).unwrap(),
            "rgb(248 113 113 / var(--opacity))"
        );
        assert_eq!(
            get(&config, "red-400/20", Some("--opacity")).unwrap(),
            "rgb(248 113 113 / 0.2)"
        );
        assert_eq!(get(&config, "red-400", None).unwrap(), "rgb(248 113 113)");
        assert_eq!(
            get(&config, "red-400/20", None).unwrap(),
            "rgb(248 113 113 / 0.2)"
        );
        assert!(get(&config, "red-400/aaa", Some("--opacity")).is_none());
        assert!(get(&config, "red-400/aaa", None).is_none());
        assert_eq!(get(&config, "current", None).unwrap(), "currentColor");
        assert_eq!(get(&config, "inherit", None).unwrap(), "inherit");
        assert_eq!(get(&config, "transparent", None).unwrap(), "rgb(0 0 0 / 0)");
        assert_eq!(
            get(&config, "transparent/20", None).unwrap(),
            "rgb(0 0 0 / 0.2)"
        );
        assert!(get(&config, "transparent/aaa", None).is_none());
        assert_eq!(get(&config, "rosa-600", None).unwrap(), "rgb(242 60 255)");
        assert_eq!(
            get(&config, "rosa-600/20", None).unwrap(),
            "rgb(242 60 255 / 0.2)"
        );
    }
}
