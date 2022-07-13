//! Define some color manipulation functions.
use crate::{
    config::{Config, BUILTIN_COLORS},
    error::{Error, Result},
};

use std::borrow::Cow;

/// Convert an hexadecimal color to an RGB one.
///
/// # Example
///
/// ```rust
/// use encre_css::utils::color::hex_to_rgb;
/// assert_eq!(hex_to_rgb("#333").unwrap(), (51, 51, 51));
/// assert_eq!(hex_to_rgb("#f1f1f1").unwrap(), (241, 241, 241));
/// ```
///
/// # Errors
///
/// Returns [`Error::HexToRgbConversion`] when the hexadecimal color is incorrect.
pub fn hex_to_rgb(mut hex: &str) -> Result<(u8, u8, u8)> {
    // Remove the useless `#` from the start of the color
    hex = hex.strip_prefix('#').unwrap_or(hex);

    if hex.len() == 3 {
        // Support the hexadecimal shorthand
        let r = u8::from_str_radix(&hex[0..1], 16)
            .map_err(|e| Error::HexToRgbConversion(hex.to_string(), e.to_string()))?;
        let g = u8::from_str_radix(&hex[1..2], 16)
            .map_err(|e| Error::HexToRgbConversion(hex.to_string(), e.to_string()))?;
        let b = u8::from_str_radix(&hex[2..3], 16)
            .map_err(|e| Error::HexToRgbConversion(hex.to_string(), e.to_string()))?;

        Ok((r + r * 16, g + g * 16, b + b * 16))
    } else if hex.len() == 6 {
        let hex = hex.to_lowercase();

        let r = u8::from_str_radix(&hex[0..2], 16)
            .map_err(|e| Error::HexToRgbConversion(hex.clone(), e.to_string()))?;
        let g = u8::from_str_radix(&hex[2..4], 16)
            .map_err(|e| Error::HexToRgbConversion(hex.clone(), e.to_string()))?;
        let b = u8::from_str_radix(&hex[4..6], 16)
            .map_err(|e| Error::HexToRgbConversion(hex, e.to_string()))?;

        Ok((r, g, b))
    } else {
        Err(Error::HexToRgbConversion(
            hex.to_string(),
            format!(
                "bad number of digits (expected 3 or 6, found {})",
                hex.len()
            ),
        ))
    }
}

/// Returns whether the modifier is matching a builtin color. The builtin colors are:
///
/// - `current`, `inherit`, `transparent`, `black`, `white`;
/// - Any key contained in the [`BUILTIN_COLORS`] list.
pub fn is_matching_builtin_color(config: &Config, mut modifier: &str) -> bool {
    if ["current", "inherit", "transparent", "black", "white"].contains(&modifier) {
        return true;
    }

    // Trim the opacity suffix, if present
    if let Some((new_modifier, _)) = modifier.split_once('/') {
        modifier = new_modifier;
    }

    BUILTIN_COLORS.iter().any(|color| color.0 == modifier) || config.theme.colors.contains(modifier)
}

/// Get a color from a modifier.
///
/// The third argument is used to set the opacity type used:
///
/// - [`Option::None`] will not use opacity (except if the opacity syntax is used, for example in `bg-red-500/25`);
/// - [`Option::Some`] contains a variable which will be added as the opacity of the color, used to
/// dynamically change the opacity.
pub fn get<'a>(
    config: &Config,
    modifier: &'a str,
    opacity: Option<&'static str>,
) -> Option<Cow<'a, str>> {
    // Handle the new opacity syntax (e.g. `bg-red-500/25`)
    let (mut opacity_from_syntax, modifier) = {
        // The `current` and `inherit` modifiers have static values
        if modifier == "current" {
            return Some(Cow::from("currentColor"));
        } else if modifier == "inherit" {
            return Some(Cow::from("inherit"));
        } else if let Some((new_modifier, opacity_suffix)) = modifier.split_once('/') {
            if let Ok(opacity_number) = opacity_suffix.parse::<usize>() {
                #[allow(clippy::cast_precision_loss)]
                (Some(opacity_number as f32 / 100.), new_modifier)
            } else {
                (None, new_modifier)
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
        hex_to_rgb(hex_color).ok()
    } else {
        BUILTIN_COLORS
            .iter()
            .find(|color| color.0 == modifier)
            .map(|color| color.1)
    };

    // Convert the array to a CSS color with an opacity value (if the color is found)
    rgb_result.map(|rgb_result| {
        Cow::from(format!(
            "rgb({} {} {}{})",
            rgb_result.0,
            rgb_result.1,
            rgb_result.2,
            if let Some(opacity_from_syntax) = opacity_from_syntax {
                Cow::from(format!(" / {}", opacity_from_syntax))
            } else if let Some(opacity) = opacity {
                Cow::from(format!(" / var({})", opacity))
            } else {
                Cow::from("")
            }
        ))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_to_rgb_test() {
        assert_eq!(hex_to_rgb(&Cow::from("#ff0000")).unwrap(), (255, 0, 0));
        assert_eq!(hex_to_rgb(&Cow::from("#FF00FF")).unwrap(), (255, 0, 255));
        assert_eq!(hex_to_rgb(&Cow::from("#332")).unwrap(), (51, 51, 34));
        assert_eq!(hex_to_rgb(&Cow::from("#FEF")).unwrap(), (255, 238, 255));
    }
}
