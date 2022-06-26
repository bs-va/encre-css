use super::{to_css_value, Plugin};
use crate::{context::{ContextCanHandle, ContextHandle}, utils::{color, indent, shadow, value_matchers::*}, selector::Modifier};

use std::fmt::{self, Write};

const CSS_SHADOW: &str = "box-shadow: var(--en-ring-offset-shadow, 0 0 #0000), var(--en-ring-shadow, 0 0 #0000), var(--en-shadow);";

#[derive(Debug)]
pub struct MixBlendModePlugin;

impl Plugin for MixBlendModePlugin {
    fn namespace(&self) -> &str {
        "mix-blend"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => [
                "normal",
                "multiply",
                "screen",
                "overlay",
                "darken",
                "lighten",
                "color-dodge",
                "color-burn",
                "hard-light",
                "soft-light",
                "difference",
                "exclusion",
                "hue",
                "saturation",
                "color",
                "luminosity",
            ]
            .contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "normal" => writeln!(context.buffer, "mix-blend-mode: normal;")?,
                "multiply" => writeln!(context.buffer, "mix-blend-mode: multiply;")?,
                "screen" => writeln!(context.buffer, "mix-blend-mode: screen;")?,
                "overlay" => writeln!(context.buffer, "mix-blend-mode: overlay;")?,
                "darken" => writeln!(context.buffer, "mix-blend-mode: darken;")?,
                "lighten" => writeln!(context.buffer, "mix-blend-mode: lighten;")?,
                "color-dodge" => writeln!(context.buffer, "mix-blend-mode: color-dodge;")?,
                "color-burn" => writeln!(context.buffer, "mix-blend-mode: color-burn;")?,
                "hard-light" => writeln!(context.buffer, "mix-blend-mode: hard-light;")?,
                "soft-light" => writeln!(context.buffer, "mix-blend-mode: soft-light;")?,
                "difference" => writeln!(context.buffer, "mix-blend-mode: difference;")?,
                "exclusion" => writeln!(context.buffer, "mix-blend-mode: exclusion;")?,
                "hue" => writeln!(context.buffer, "mix-blend-mode: hue;")?,
                "saturation" => writeln!(context.buffer, "mix-blend-mode: saturation;")?,
                "color" => writeln!(context.buffer, "mix-blend-mode: color;")?,
                "luminosity" => writeln!(context.buffer, "mix-blend-mode: luminosity;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct BackgroundBlendModePlugin;

impl Plugin for BackgroundBlendModePlugin {
    fn namespace(&self) -> &str {
        "bg-blend"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => [
                "normal",
                "multiply",
                "screen",
                "overlay",
                "darken",
                "lighten",
                "color-dodge",
                "color-burn",
                "hard-light",
                "soft-light",
                "difference",
                "exclusion",
                "hue",
                "saturation",
                "color",
                "luminosity",
            ]
            .contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "normal" => writeln!(context.buffer, "background-blend-mode: normal;")?,
                "multiply" => writeln!(context.buffer, "background-blend-mode: multiply;")?,
                "screen" => writeln!(context.buffer, "background-blend-mode: screen;")?,
                "overlay" => writeln!(context.buffer, "background-blend-mode: overlay;")?,
                "darken" => writeln!(context.buffer, "background-blend-mode: darken;")?,
                "lighten" => writeln!(context.buffer, "background-blend-mode: lighten;")?,
                "color-dodge" => writeln!(context.buffer, "background-blend-mode: color-dodge;")?,
                "color-burn" => writeln!(context.buffer, "background-blend-mode: color-burn;")?,
                "hard-light" => writeln!(context.buffer, "background-blend-mode: hard-light;")?,
                "soft-light" => writeln!(context.buffer, "background-blend-mode: soft-light;")?,
                "difference" => writeln!(context.buffer, "background-blend-mode: difference;")?,
                "exclusion" => writeln!(context.buffer, "background-blend-mode: exclusion;")?,
                "hue" => writeln!(context.buffer, "background-blend-mode: hue;")?,
                "saturation" => writeln!(context.buffer, "background-blend-mode: saturation;")?,
                "color" => writeln!(context.buffer, "background-blend-mode: color;")?,
                "luminosity" => writeln!(context.buffer, "background-blend-mode: luminosity;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct BoxShadowPlugin;

impl Plugin for BoxShadowPlugin {
    fn namespace(&self) -> &str {
        "shadow"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => {
                ["", "sm", "md", "lg", "xl", "2xl", "inner", "none"].contains(&&**value)
            }
            Modifier::Arbitrary { value, .. } => is_matching_shadow(value),
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "" => {
                    writeln!(context.buffer, "--en-shadow: 0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1);")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "--en-shadow-colored: 0 1px 3px 0 var(--en-shadow-color), 0 1px 2px -1px var(--en-shadow-color);")?;
                }
                "sm" => {
                    writeln!(context.buffer, "--en-shadow: 0 1px 2px 0 rgb(0 0 0 / 0.05);")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(
                        context.buffer,
                        "--en-shadow-colored: 0 1px 2px 0 var(--en-shadow-color);"
                    )?;
                }
                "md" => {
                    writeln!(context.buffer, "--en-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1);")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "--en-shadow-colored: 0 4px 6px -1px var(--en-shadow-color), 0 2px 4px -2px var(--en-shadow-color);")?;
                }
                "lg" => {
                    writeln!(context.buffer, "--en-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1);")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "--en-shadow-colored: 0 10px 15px -3px var(--en-shadow-color), 0 4px 6px -4px var(--en-shadow-color);")?;
                }
                "xl" => {
                    writeln!(context.buffer, "--en-shadow: 0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1);")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "--en-shadow-colored: 0 20px 25px -5px var(--en-shadow-color), 0 8px 10px -6px var(--en-shadow-color);")?;
                }
                "2xl" => {
                    writeln!(context.buffer, "--en-shadow: 0 25px 50px -12px rgb(0 0 0 / 0.25);")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(
                        context.buffer,
                        "--en-shadow-colored: 0 25px 50px -12px var(--en-shadow-color);"
                    )?;
                }
                "inner" => {
                    writeln!(context.buffer, "--en-shadow: inset 0 2px 4px 0 rgb(0 0 0 / 0.05);")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(
                        context.buffer,
                        "--en-shadow-colored: inset 0 2px 4px 0 var(--en-shadow-color);"
                    )?;
                }
                "none" => writeln!(context.buffer, "box-shadow: none;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                let value = to_css_value(value);
                writeln!(context.buffer, "--en-shadow: {value};")?;
                indent(context.indentation, context.buffer)?;
                let mut shadow = shadow::parse_shadow(&value).unwrap();
                shadow.replace_all_colors("var(--en-shadow-color)");
                writeln!(context.buffer, "--en-shadow-colored: {};", shadow)?
            }
        }

        indent(context.indentation, context.buffer)?;
        writeln!(context.buffer, "{}", CSS_SHADOW)?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct BoxShadowColorPlugin;

impl Plugin for BoxShadowColorPlugin {
    fn namespace(&self) -> &str {
        "shadow"
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
        let value = match context.modifier {
            Modifier::Basic { value, .. } => color::get(context.config, value, None).unwrap(),
            Modifier::Arbitrary { value, .. } => to_css_value(*value),
        };
        writeln!(context.buffer, "--en-shadow-color: {value};")?;

        indent(context.indentation, context.buffer)?;
        writeln!(context.buffer, "--en-shadow: var(--en-shadow-colored);")?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct OpacityPlugin;

impl Plugin for OpacityPlugin {
    fn namespace(&self) -> &str {
        "opacity"
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
                "opacity: {};",
                value.parse::<usize>().unwrap() as f32 / 100.
            )?,
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
