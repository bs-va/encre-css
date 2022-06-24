use crate::{
    config::{Config, BUILTIN_COLORS},
    error::{Error, Result},
};

use std::borrow::Cow;

pub fn hex_to_rgb(hex: &str) -> Result<[u8; 3]> {
    // Remove the useless `#` from the start of the color
    let hex = if let Some(hex) = hex.strip_prefix('#') {
        hex
    } else {
        hex
    };

    // Support the hexadecimal shorthand
    let hex = if hex.len() == 3 {
        hex.chars()
            .map(|ch| ch.to_string().repeat(2).to_lowercase())
            .collect::<String>()
    } else {
        hex.to_lowercase()
    };

    // TODO: Handle errors
    let r = u8::from_str_radix(&hex[0..2], 16)
        .map_err(|e| Error::HexToRgbConversion(hex[0..2].to_string(), e))?;
    let g = u8::from_str_radix(&hex[2..4], 16)
        .map_err(|e| Error::HexToRgbConversion(hex[2..4].to_string(), e))?;
    let b = u8::from_str_radix(&hex[4..6], 16)
        .map_err(|e| Error::HexToRgbConversion(hex[4..6].to_string(), e))?;

    Ok([r, g, b])
}

pub fn is_matching_basic_color(config: &Config, mut modifier: &str) -> bool {
    if ["current", "inherit", "transparent", "black", "white"].contains(&modifier) {
        return true;
    }

    // Trim the opacity suffix, if present
    if let Some((new_modifier, _)) = modifier.split_once('/') {
        modifier = new_modifier;
    }

    BUILTIN_COLORS.iter().any(|color| color.0 == modifier)
        || config.theme.colors.contains_key(modifier)
}

/// Get a color from a modifier
pub fn get<'a>(
    config: &Config,
    modifier: &'a str,
    opacity: Option<&'static str>,
) -> Option<Cow<'a, str>> {
    // Handle the new opacity syntax (e.g. `bg-red-500/25`)
    let (mut opacity_from_syntax, modifier) = {
        // The `current` and `inherit` modifiers cannot have their opacity changed
        if modifier == "current" {
            (None, "currentColor")
        } else if modifier == "inherit" {
            (None, "inherit")
        } else if let Some((new_modifier, opacity_suffix)) = modifier.split_once('/') {
            if let Ok(opacity_number) = opacity_suffix.parse::<usize>() {
                (Some(opacity_number as f32 / 100.), new_modifier)
            } else {
                (None, modifier)
            }
        } else {
            (None, modifier)
        }
    };

    let rgb_result = if modifier == "transparent" {
        if opacity_from_syntax.is_none() {
            opacity_from_syntax = Some(0.0);
        }

        Some([0, 0, 0])
    } else if modifier == "black" {
        Some([0, 0, 0])
    } else if modifier == "white" {
        Some([0xff, 0xff, 0xff])
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
            rgb_result[0],
            rgb_result[1],
            rgb_result[2],
            if let Some(opacity) = opacity {
                if let Some(opacity_from_syntax) = opacity_from_syntax {
                    Cow::from(format!(" / {}", opacity_from_syntax))
                } else {
                    Cow::from(format!(" / var({})", opacity))
                }
            } else {
                Cow::from("")
            },
        ))
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
