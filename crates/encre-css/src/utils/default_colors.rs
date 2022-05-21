use crate::config::Config;

use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::Cow;

lazy_static! {
    static ref OPACITY_SUFFIX_REGEX: Regex = Regex::new(r"(?-u)/(\d*)$").unwrap();
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

        Some(&[0, 0, 0])
    } else if modifier == "black" {
        Some(&[0, 0, 0])
    } else if modifier == "white" {
        Some(&[0xff, 0xff, 0xff])
    } else {
        config.theme.colors.get(&Cow::from(modifier))
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
