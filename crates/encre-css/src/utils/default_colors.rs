use crate::{error::{Result, Error}, config::Config};

use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::Cow;

lazy_static! {
    static ref OPACITY_SUFFIX_REGEX: Regex = Regex::new(r"(?-u)/(\d*)$").unwrap();
}

pub fn hex_to_rgb(hex: &str) -> Result<[u8; 3]> {
    // Remove the useless `#` from the start of the color
    let hex = if let Some(hex) = hex.strip_prefix('#') {
        hex
    } else {
        hex
    };

    // Support the hexadecimal shorthand
    let hex = if hex.len() == 3 {
        hex.chars().map(|ch| ch.to_string().repeat(2).to_lowercase()).collect::<String>()
    } else {
        hex.to_lowercase()
    };

    // TODO: Handle errors
    let r = u8::from_str_radix(&hex[0..2], 16).map_err(|e| Error::HexToRgbConversion(hex[0..2].to_string(), e))?;
    let g = u8::from_str_radix(&hex[2..4], 16).map_err(|e| Error::HexToRgbConversion(hex[2..4].to_string(), e))?;
    let b = u8::from_str_radix(&hex[4..6], 16).map_err(|e| Error::HexToRgbConversion(hex[4..6].to_string(), e))?;

    Ok([r, g, b])
}

/// Get a color from a modifier
pub fn get(config: &Config, modifier: &str) -> Option<String> {
    // Handle the new opacity syntax (e.g. `bg-red-500/25`)
    let (mut opacity, modifier) =
        if let Some(opacity_suffix) = OPACITY_SUFFIX_REGEX.captures(modifier) {
            let new_modifier = &OPACITY_SUFFIX_REGEX.replace(modifier, "");
            (
                Some(
                    opacity_suffix
                        .get(1)
                        .unwrap()
                        .as_str()
                        .parse::<f32>()
                        .unwrap()
                        / 100.,
                ),
                new_modifier.to_string(),
            )
        } else {
            // The `current` and `inherit` modifiers cannot have their opacity changed
            if modifier == "current" {
                return Some("currentColor".to_string());
            } else if modifier == "inherit" {
                return Some("inherit".to_string());
            }

            (None, modifier.to_string())
        };

    let rgb_result = if modifier == "transparent" {
        if opacity.is_none() {
            opacity = Some(0.0);
        }

        Some([0, 0, 0])
    } else if modifier == "black" {
        Some([0, 0, 0])
    } else if modifier == "white" {
        Some([0xff, 0xff, 0xff])
    } else if let Some(hex_color) = config.theme.colors.get(&Cow::from(modifier)) {
        hex_to_rgb(hex_color).ok()
    } else {
        None
    };

    // Convert the array to a CSS color with an opacity value (if the color is found)
    rgb_result.map(|rgb_result| {
        format!(
            "rgb({} {} {} / {})",
            rgb_result[0],
            rgb_result[1],
            rgb_result[2],
            if let Some(opacity) = opacity {
                opacity.to_string()
            } else {
                "var(--en-opacity)".to_string()
            }
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn hex_to_rgb_test() {
        assert_eq!(hex_to_rgb(&Cow::from("#ff0000")).unwrap(), [255, 0, 0]);
        assert_eq!(hex_to_rgb(&Cow::from("#FF00FF")).unwrap(), [255, 0, 255]);
        assert_eq!(hex_to_rgb(&Cow::from("#332")).unwrap(), [51, 51, 34]);
        assert_eq!(hex_to_rgb(&Cow::from("#FEF")).unwrap(), [255, 238, 255]);
    }
}