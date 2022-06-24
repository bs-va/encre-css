use super::{to_css_value, Plugin};
use crate::utils::{color, indent, value_matchers::*};
use crate::{config::Config, selector::Modifier};

use std::{
    borrow::Cow,
    fmt::{self, Write},
};

#[derive(Debug)]
pub struct ColorPlugin;

impl Plugin for ColorPlugin {
    fn namespace(&self) -> &str {
        "bg"
    }

    fn can_handle(&self, config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => color::is_matching_basic_color(config, value),
            Modifier::Arbitrary { hint, value } => {
                *hint == "color" || (hint.is_empty() && is_matching_color(value))
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
                let color = color::get(config, value, Some("--en-bg-opacity")).unwrap();
                if color.contains("--en-bg-opacity") {
                    writeln!(buffer, "--en-bg-opacity: 1;")?;
                    indent(indentation, buffer)?;
                }

                writeln!(buffer, "background-color: {color};")?;
            }
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "background-color: {};", to_css_value(value))?;
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct AttachmentPlugin;

impl Plugin for AttachmentPlugin {
    fn namespace(&self) -> &str {
        "bg"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => ["fixed", "local", "scroll"].contains(value),
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
            Modifier::Basic { value, .. } => match *value {
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

#[derive(Debug)]
pub struct ClipPlugin;

impl Plugin for ClipPlugin {
    fn namespace(&self) -> &str {
        "bg-clip"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => {
                ["border", "padding", "content", "text"].contains(value)
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
            Modifier::Basic { value, .. } => match *value {
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

#[derive(Debug)]
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

#[derive(Debug)]
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
                "gradient-to-bl",
                "gradient-to-l",
                "gradient-to-tl",
            ]
            .contains(value),
            Modifier::Arbitrary { value, .. } => is_matching_image(value),
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
            Modifier::Basic { value, .. } => match *value {
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
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "background-image: {};", to_css_value(value))?
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct GradientFromPlugin;

impl Plugin for GradientFromPlugin {
    fn namespace(&self) -> &str {
        "from"
    }

    fn can_handle(&self, config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => color::is_matching_basic_color(config, value),
            Modifier::Arbitrary { value, .. } => is_matching_color(value),
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
            Modifier::Basic { value, .. } => color::get(config, value, None).unwrap(),
            Modifier::Arbitrary { value, .. } => to_css_value(*value),
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

#[derive(Debug)]
pub struct GradientViaPlugin;

impl Plugin for GradientViaPlugin {
    fn namespace(&self) -> &str {
        "via"
    }

    fn can_handle(&self, config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => color::is_matching_basic_color(config, value),
            Modifier::Arbitrary { value, .. } => is_matching_color(value),
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
            Modifier::Basic { value, .. } => color::get(config, value, None).unwrap(),
            Modifier::Arbitrary { value, .. } => to_css_value(*value),
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

#[derive(Debug)]
pub struct GradientToPlugin;

impl Plugin for GradientToPlugin {
    fn namespace(&self) -> &str {
        "to"
    }

    fn can_handle(&self, config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => color::is_matching_basic_color(config, value),
            Modifier::Arbitrary { value, .. } => is_matching_color(value),
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
            Modifier::Basic { value, .. } => color::get(config, value, None).unwrap(),
            Modifier::Arbitrary { value, .. } => to_css_value(*value),
        };

        indent(indentation, buffer)?;
        writeln!(buffer, "--en-gradient-to: {value};")?;

        Ok(())
    }
}

#[derive(Debug)]
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
            .contains(value),
            Modifier::Arbitrary { value, .. } => {
                // https://developer.mozilla.org/en-US/docs/Web/CSS/background-position
                value
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
            Modifier::Basic { value, .. } => match *value {
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
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "background-position: {};", to_css_value(value))?
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
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
            .contains(value),
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
            Modifier::Basic { value, .. } => match *value {
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

#[derive(Debug)]
pub struct SizePlugin;

impl Plugin for SizePlugin {
    fn namespace(&self) -> &str {
        "bg"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => ["contain", "cover", "auto"].contains(value),
            Modifier::Arbitrary { hint, value } => {
                *hint == "length"
                    || (hint.is_empty()
                        && value.split(',').all(|v| {
                            v.split('_').all(|v| {
                                is_matching_length(v)
                                    || is_matching_percentage(v)
                                    || ["contain", "cover", "auto"].contains(&v)
                            })
                        }))
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
            Modifier::Basic { value, .. } => match *value {
                "auto" => writeln!(buffer, "background-size: auto;")?,
                "cover" => writeln!(buffer, "background-size: cover;")?,
                "contain" => writeln!(buffer, "background-size: contain;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "background-size: {};", to_css_value(value))?
            }
        }

        Ok(())
    }
}
