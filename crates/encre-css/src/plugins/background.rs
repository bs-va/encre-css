use super::Plugin;
use crate::utils::{default_colors, indent, value_matchers::*};
use crate::{config::Config, selector::Modifier};

use smol_str::SmolStr;
use std::{
    borrow::Cow,
    fmt::{self, Write},
};

pub struct ColorPlugin;

impl Plugin for ColorPlugin {
    fn namespace(&self) -> &str {
        "bg"
    }

    fn can_handle(&self, config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => {
                default_colors::get(config, value, Some("--en-bg-opacity")).is_some()
            }
            Modifier::Arbitrary { hint, value } => {
                hint == "color" || is_matching_color(value) || is_matching_url(value)
            }
        }
    }

    fn handle(
        &self,
        config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { value, .. } => {
                let color = default_colors::get(config, value, Some("--en-bg-opacity")).unwrap();
                if color.contains("--en-bg-opacity") {
                    writeln!(buffer, "--en-bg-opacity: 1;")?;
                    indent(indentation, buffer)?;
                }

                writeln!(buffer, "background-color: {color};")?;
            }
            Modifier::Arbitrary { value, .. } => {
                let property = if value.contains("url") {
                    "background-image"
                } else {
                    "background-color"
                };

                writeln!(buffer, "{property}: {value};")?;
            }
        }

        Ok(())
    }
}

pub struct AttachmentPlugin;

impl Plugin for AttachmentPlugin {
    fn namespace(&self) -> &str {
        "bg"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => ["fixed", "local", "scroll"].contains(&value.as_str()),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { value, .. } => match value.as_str() {
                "fixed" => writeln!(buffer, "background-attachment: fixed;")?,
                "local" => writeln!(buffer, "background-attachment: local;")?,
                "scroll" => writeln!(buffer, "background-attachment: scroll;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

pub struct ClipPlugin;

impl Plugin for ClipPlugin {
    fn namespace(&self) -> &str {
        "bg-clip"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => {
                ["border", "padding", "content", "text"].contains(&value.as_str())
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { value, .. } => match value.as_str() {
                "border" => writeln!(buffer, "background-clip: border-box;")?,
                "padding" => writeln!(buffer, "background-clip: padding-box;")?,
                "content" => writeln!(buffer, "background-clip: content-box;")?,
                "text" => writeln!(buffer, "background-clip: text;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

pub struct OpacityPlugin;

impl Plugin for OpacityPlugin {
    fn namespace(&self) -> &str {
        "bg-opacity"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => value.parse::<usize>().is_ok(),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        // NOTE: Not-compatible with TailwindCSS, support all values
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { value, .. } => writeln!(
                buffer,
                "--en-bg-opacity: {};",
                value.parse::<usize>().unwrap() as f32 / 100.
            )?,
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

pub struct ImagePlugin;

impl Plugin for ImagePlugin {
    fn namespace(&self) -> &str {
        "bg"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => [
                "none",
                "gradient-to-t",
                "gradient-to-tr",
                "gradient-to-r",
                "gradient-to-br",
                "gradient-to-b",
                "grandient-to-bl",
                "gradient-to-l",
                "gradient-to-tl",
            ]
            .contains(&value.as_str()),
            Modifier::Arbitrary { hint, value } => hint == "list" || is_matching_image(value),
        }
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { value, .. } => match value.as_str() {
                "none" => writeln!(buffer, "background-image: none;")?,
                "gradient-to-t" => writeln!(
                    buffer,
                    "background-image: linear-gradient(to top, var(--en-gradient-stops));"
                )?,
                "gradient-to-tr" => writeln!(
                    buffer,
                    "background-image: linear-gradient(to top right, var(--en-gradient-stops));"
                )?,
                "gradient-to-r" => writeln!(
                    buffer,
                    "background-image: linear-gradient(to right, var(--en-gradient-stops));"
                )?,
                "gradient-to-br" => writeln!(
                    buffer,
                    "background-image: linear-gradient(to bottom right, var(--en-gradient-stops));"
                )?,
                "gradient-to-b" => writeln!(
                    buffer,
                    "background-image: linear-gradient(to bottom, var(--en-gradient-stops));"
                )?,
                "gradient-to-bl" => writeln!(
                    buffer,
                    "background-image: linear-gradient(to bottom left, var(--en-gradient-stops));"
                )?,
                "gradient-to-l" => writeln!(
                    buffer,
                    "background-image: linear-gradient(to left, var(--en-gradient-stops));"
                )?,
                "gradient-to-tl" => writeln!(
                    buffer,
                    "background-image: linear-gradient(to top left, var(--en-gradient-stops));"
                )?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => writeln!(buffer, "background-image: {value};")?,
        }

        Ok(())
    }
}

pub struct GradientFromPlugin;

impl Plugin for GradientFromPlugin {
    fn namespace(&self) -> &str {
        "from"
    }

    fn can_handle(&self, config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => default_colors::get(config, value, None).is_some(),
            Modifier::Arbitrary { hint, value } => hint == "color" || is_matching_color(value),
        }
    }

    fn handle(
        &self,
        config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        let value = match modifier {
            Modifier::Basic { value, .. } => {
                SmolStr::from(default_colors::get(config, value, None).unwrap())
            }
            Modifier::Arbitrary { value, .. } => value.clone(),
        };

        let default_to = if value == "inherit" || value == "currentColor" {
            Cow::from("rgb(255 255 255 / 0)")
        } else {
            let mut default = value.to_string();
            default.pop(); // Remove the last `)`
            default += "/ 0)";
            Cow::from(default)
        };

        indent(indentation, buffer)?;
        writeln!(buffer, "--en-gradient-from: {value};")?;
        indent(indentation, buffer)?;
        writeln!(
            buffer,
            "--en-gradient-stops: var(--en-gradient-from), var(--en-gradient-to, {default_to});"
        )?;

        Ok(())
    }
}

pub struct GradientViaPlugin;

impl Plugin for GradientViaPlugin {
    fn namespace(&self) -> &str {
        "via"
    }

    fn can_handle(&self, config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => default_colors::get(config, value, None).is_some(),
            Modifier::Arbitrary { hint, value } => hint == "color" || is_matching_color(value),
        }
    }

    fn handle(
        &self,
        config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        let value = match modifier {
            Modifier::Basic { value, .. } => {
                SmolStr::from(default_colors::get(config, value, None).unwrap())
            }
            Modifier::Arbitrary { value, .. } => value.clone(),
        };

        let default_to = if value == "inherit" || value == "currentColor" {
            Cow::from("rgb(255 255 255 / 0)")
        } else {
            let mut default = value.to_string();
            default.pop(); // Remove the last `)`
            default += "/ 0)";
            Cow::from(default)
        };

        indent(indentation, buffer)?;
        writeln!(
            buffer,
            "--en-gradient-stops: var(--en-gradient-from), {}, var(--en-gradient-to, {});",
            value, default_to
        )?;

        Ok(())
    }
}

pub struct GradientToPlugin;

impl Plugin for GradientToPlugin {
    fn namespace(&self) -> &str {
        "to"
    }

    fn can_handle(&self, config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => default_colors::get(config, value, None).is_some(),
            Modifier::Arbitrary { hint, value } => hint == "color" || is_matching_color(value),
        }
    }

    fn handle(
        &self,
        config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        let value = match modifier {
            Modifier::Basic { value, .. } => default_colors::get(config, value, None).unwrap(),
            Modifier::Arbitrary { value, .. } => Cow::from(&**value),
        };

        indent(indentation, buffer)?;
        writeln!(buffer, "--en-gradient-to: {value};")?;

        Ok(())
    }
}

pub struct PositionPlugin;

impl Plugin for PositionPlugin {
    fn namespace(&self) -> &str {
        "bg"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => [
                "bottom",
                "center",
                "left",
                "left-bottom",
                "left-top",
                "right",
                "right-bottom",
                "right-top",
                "top",
            ]
            .contains(&value.as_str()),
            Modifier::Arbitrary { hint, value } => {
                // https://developer.mozilla.org/en-US/docs/Web/CSS/background-position
                hint == "list"
                    || value
                        .split(',')
                        .all(|v| v.split('_').all(is_matching_position))
            }
        }
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { value, .. } => match value.as_str() {
                "bottom" => writeln!(buffer, "background-position: bottom;")?,
                "center" => writeln!(buffer, "background-position: center;")?,
                "left" => writeln!(buffer, "background-position: left;")?,
                "left-bottom" => writeln!(buffer, "background-position: left-bottom;")?,
                "left-top" => writeln!(buffer, "background-position: left-top;")?,
                "right" => writeln!(buffer, "background-position: right;")?,
                "right-bottom" => writeln!(buffer, "background-position: right-bottom;")?,
                "right-top" => writeln!(buffer, "background-position: right-top;")?,
                "top" => writeln!(buffer, "background-position: top;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => writeln!(buffer, "background-position: {value};")?,
        }

        Ok(())
    }
}

pub struct RepeatPlugin;

impl Plugin for RepeatPlugin {
    fn namespace(&self) -> &str {
        "bg"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => [
                "repeat",
                "no-repeat",
                "repeat-x",
                "repeat-y",
                "repeat-round",
                "repeat-space",
            ]
            .contains(&value.as_str()),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { value, .. } => match value.as_str() {
                "repeat" => writeln!(buffer, "background-repeat: repeat;")?,
                "no-repeat" => writeln!(buffer, "background-repeat: no-repeat;")?,
                "repeat-x" => writeln!(buffer, "background-repeat: repeat-x;")?,
                "repeat-y" => writeln!(buffer, "background-repeat: repeat-y;")?,
                "repeat-round" => writeln!(buffer, "background-repeat: round;")?,
                "repeat-space" => writeln!(buffer, "background-repeat: space;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

pub struct SizePlugin;

impl Plugin for SizePlugin {
    fn namespace(&self) -> &str {
        "bg"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => ["contain", "cover", "auto"].contains(&value.as_str()),
            Modifier::Arbitrary { hint, value } => {
                hint == "length"
                    || value.split(',').all(|v| {
                        v.split('_').all(|v| {
                            is_matching_length(v)
                                || is_matching_percentage(v)
                                || ["contain", "cover", "auto"].contains(&v)
                        })
                    })
            }
        }
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { value, .. } => match value.as_str() {
                "auto" => writeln!(buffer, "background-size: auto;")?,
                "cover" => writeln!(buffer, "background-size: cover;")?,
                "contain" => writeln!(buffer, "background-size: contain;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => writeln!(buffer, "background-size: {value};")?,
        }

        Ok(())
    }
}
