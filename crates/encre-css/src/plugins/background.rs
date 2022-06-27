use super::{to_css_value, Plugin};
use crate::{
    context::{ContextCanHandle, ContextHandle},
    selector::Modifier,
    utils::{color, indent, value_matchers::*},
};

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

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => color::is_matching_basic_color(context.config, value),
            Modifier::Arbitrary { hint, value, .. } => {
                *hint == "color" || (hint.is_empty() && is_matching_color(value))
            }
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => {
                let color = color::get(context.config, value, Some("--en-bg-opacity")).unwrap();
                if color.contains("--en-bg-opacity") {
                    writeln!(context.buffer, "--en-bg-opacity: 1;")?;
                    indent(context.indentation, context.buffer)?;
                }

                writeln!(context.buffer, "background-color: {color};")?;
            }
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "background-color: {};", to_css_value(value))?;
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

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => ["fixed", "local", "scroll"].contains(value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "fixed" => writeln!(context.buffer, "background-attachment: fixed;")?,
                "local" => writeln!(context.buffer, "background-attachment: local;")?,
                "scroll" => writeln!(context.buffer, "background-attachment: scroll;")?,
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

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => {
                ["border", "padding", "content", "text"].contains(value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "border" => writeln!(context.buffer, "background-clip: border-box;")?,
                "padding" => writeln!(context.buffer, "background-clip: padding-box;")?,
                "content" => writeln!(context.buffer, "background-clip: content-box;")?,
                "text" => writeln!(context.buffer, "background-clip: text;")?,
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

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => value.parse::<usize>().is_ok(),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        // NOTE: Not-compatible with TailwindCSS, support all values
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => writeln!(
                context.buffer,
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

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
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

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "none" => writeln!(context.buffer, "background-image: none;")?,
                "gradient-to-t" => writeln!(
                    context.buffer,
                    "background-image: linear-gradient(to top, var(--en-gradient-stops));"
                )?,
                "gradient-to-tr" => writeln!(
                    context.buffer,
                    "background-image: linear-gradient(to top right, var(--en-gradient-stops));"
                )?,
                "gradient-to-r" => writeln!(
                    context.buffer,
                    "background-image: linear-gradient(to right, var(--en-gradient-stops));"
                )?,
                "gradient-to-br" => writeln!(
                    context.buffer,
                    "background-image: linear-gradient(to bottom right, var(--en-gradient-stops));"
                )?,
                "gradient-to-b" => writeln!(
                    context.buffer,
                    "background-image: linear-gradient(to bottom, var(--en-gradient-stops));"
                )?,
                "gradient-to-bl" => writeln!(
                    context.buffer,
                    "background-image: linear-gradient(to bottom left, var(--en-gradient-stops));"
                )?,
                "gradient-to-l" => writeln!(
                    context.buffer,
                    "background-image: linear-gradient(to left, var(--en-gradient-stops));"
                )?,
                "gradient-to-tl" => writeln!(
                    context.buffer,
                    "background-image: linear-gradient(to top left, var(--en-gradient-stops));"
                )?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "background-image: {};", to_css_value(value))?
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

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => color::is_matching_basic_color(context.config, value),
            Modifier::Arbitrary { value, .. } => is_matching_color(value),
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        let value = match context.modifier {
            Modifier::Basic { value, .. } => color::get(context.config, value, None).unwrap(),
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

        indent(context.indentation, context.buffer)?;
        writeln!(context.buffer, "--en-gradient-from: {value};")?;
        indent(context.indentation, context.buffer)?;
        writeln!(
            context.buffer,
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

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => color::is_matching_basic_color(context.config, value),
            Modifier::Arbitrary { value, .. } => is_matching_color(value),
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        let value = match context.modifier {
            Modifier::Basic { value, .. } => color::get(context.config, value, None).unwrap(),
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

        indent(context.indentation, context.buffer)?;
        writeln!(
            context.buffer,
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

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => color::is_matching_basic_color(context.config, value),
            Modifier::Arbitrary { value, .. } => is_matching_color(value),
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        let value = match context.modifier {
            Modifier::Basic { value, .. } => color::get(context.config, value, None).unwrap(),
            Modifier::Arbitrary { value, .. } => to_css_value(*value),
        };

        indent(context.indentation, context.buffer)?;
        writeln!(context.buffer, "--en-gradient-to: {value};")?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct PositionPlugin;

impl Plugin for PositionPlugin {
    fn namespace(&self) -> &str {
        "bg"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
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

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "bottom" => writeln!(context.buffer, "background-position: bottom;")?,
                "center" => writeln!(context.buffer, "background-position: center;")?,
                "left" => writeln!(context.buffer, "background-position: left;")?,
                "left-bottom" => writeln!(context.buffer, "background-position: left-bottom;")?,
                "left-top" => writeln!(context.buffer, "background-position: left-top;")?,
                "right" => writeln!(context.buffer, "background-position: right;")?,
                "right-bottom" => writeln!(context.buffer, "background-position: right-bottom;")?,
                "right-top" => writeln!(context.buffer, "background-position: right-top;")?,
                "top" => writeln!(context.buffer, "background-position: top;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => writeln!(
                context.buffer,
                "background-position: {};",
                to_css_value(value)
            )?,
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

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
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

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "repeat" => writeln!(context.buffer, "background-repeat: repeat;")?,
                "no-repeat" => writeln!(context.buffer, "background-repeat: no-repeat;")?,
                "repeat-x" => writeln!(context.buffer, "background-repeat: repeat-x;")?,
                "repeat-y" => writeln!(context.buffer, "background-repeat: repeat-y;")?,
                "repeat-round" => writeln!(context.buffer, "background-repeat: round;")?,
                "repeat-space" => writeln!(context.buffer, "background-repeat: space;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct OriginPlugin;

impl Plugin for OriginPlugin {
    fn namespace(&self) -> &str {
        "bg-origin"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => ["border", "padding", "content"].contains(value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => {
                writeln!(context.buffer, "background-origin: {value}-box;")?
            }
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

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => ["contain", "cover", "auto"].contains(value),
            Modifier::Arbitrary { hint, value, .. } => {
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

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "auto" => writeln!(context.buffer, "background-size: auto;")?,
                "cover" => writeln!(context.buffer, "background-size: cover;")?,
                "contain" => writeln!(context.buffer, "background-size: contain;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "background-size: {};", to_css_value(value))?
            }
        }

        Ok(())
    }
}
