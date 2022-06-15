use super::Plugin;
use crate::utils::{default_colors, indent, shadow, value_matchers::*};
use crate::{config::Config, selector::Modifier};

use std::{
    borrow::Cow,
    fmt::{self, Write},
};

const CSS_SHADOW: &str = "box-shadow: var(--en-ring-offset-shadow, 0 0 #0000), var(--en-ring-shadow, 0 0 #0000), var(--en-shadow);";

pub struct MixBlendModePlugin;

impl Plugin for MixBlendModePlugin {
    fn namespace(&self) -> &str {
        "mix-blend"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
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
                "normal" => writeln!(buffer, "mix-blend-mode: normal;")?,
                "multiply" => writeln!(buffer, "mix-blend-mode: multiply;")?,
                "screen" => writeln!(buffer, "mix-blend-mode: screen;")?,
                "overlay" => writeln!(buffer, "mix-blend-mode: overlay;")?,
                "darken" => writeln!(buffer, "mix-blend-mode: darken;")?,
                "lighten" => writeln!(buffer, "mix-blend-mode: lighten;")?,
                "color-dodge" => writeln!(buffer, "mix-blend-mode: color-dodge;")?,
                "color-burn" => writeln!(buffer, "mix-blend-mode: color-burn;")?,
                "hard-light" => writeln!(buffer, "mix-blend-mode: hard-light;")?,
                "soft-light" => writeln!(buffer, "mix-blend-mode: soft-light;")?,
                "difference" => writeln!(buffer, "mix-blend-mode: difference;")?,
                "exclusion" => writeln!(buffer, "mix-blend-mode: exclusion;")?,
                "hue" => writeln!(buffer, "mix-blend-mode: hue;")?,
                "saturation" => writeln!(buffer, "mix-blend-mode: saturation;")?,
                "color" => writeln!(buffer, "mix-blend-mode: color;")?,
                "luminosity" => writeln!(buffer, "mix-blend-mode: luminosity;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

pub struct BackgroundBlendModePlugin;

impl Plugin for BackgroundBlendModePlugin {
    fn namespace(&self) -> &str {
        "bg-blend"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
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
                "normal" => writeln!(buffer, "background-blend-mode: normal;")?,
                "multiply" => writeln!(buffer, "background-blend-mode: multiply;")?,
                "screen" => writeln!(buffer, "background-blend-mode: screen;")?,
                "overlay" => writeln!(buffer, "background-blend-mode: overlay;")?,
                "darken" => writeln!(buffer, "background-blend-mode: darken;")?,
                "lighten" => writeln!(buffer, "background-blend-mode: lighten;")?,
                "color-dodge" => writeln!(buffer, "background-blend-mode: color-dodge;")?,
                "color-burn" => writeln!(buffer, "background-blend-mode: color-burn;")?,
                "hard-light" => writeln!(buffer, "background-blend-mode: hard-light;")?,
                "soft-light" => writeln!(buffer, "background-blend-mode: soft-light;")?,
                "difference" => writeln!(buffer, "background-blend-mode: difference;")?,
                "exclusion" => writeln!(buffer, "background-blend-mode: exclusion;")?,
                "hue" => writeln!(buffer, "background-blend-mode: hue;")?,
                "saturation" => writeln!(buffer, "background-blend-mode: saturation;")?,
                "color" => writeln!(buffer, "background-blend-mode: color;")?,
                "luminosity" => writeln!(buffer, "background-blend-mode: luminosity;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

pub struct BoxShadowPlugin;

impl Plugin for BoxShadowPlugin {
    fn namespace(&self) -> &str {
        "shadow"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => {
                ["", "sm", "md", "lg", "xl", "2xl", "inner", "none"].contains(&&**value)
            }
            Modifier::Arbitrary { value, .. } => is_matching_shadow(value),
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
                "" => {
                    writeln!(buffer, "--en-shadow: 0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1);")?;
                    indent(indentation, buffer)?;
                    writeln!(buffer, "--en-shadow-colored: 0 1px 3px 0 var(--en-shadow-color), 0 1px 2px -1px var(--en-shadow-color);")?;
                }
                "sm" => {
                    writeln!(buffer, "--en-shadow: 0 1px 2px 0 rgb(0 0 0 / 0.05);")?;
                    indent(indentation, buffer)?;
                    writeln!(
                        buffer,
                        "--en-shadow-colored: 0 1px 2px 0 var(--en-shadow-color);"
                    )?;
                }
                "md" => {
                    writeln!(buffer, "--en-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1);")?;
                    indent(indentation, buffer)?;
                    writeln!(buffer, "--en-shadow-colored: 0 4px 6px -1px var(--en-shadow-color), 0 2px 4px -2px var(--en-shadow-color);")?;
                }
                "lg" => {
                    writeln!(buffer, "--en-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1);")?;
                    indent(indentation, buffer)?;
                    writeln!(buffer, "--en-shadow-colored: 0 10px 15px -3px var(--en-shadow-color), 0 4px 6px -4px var(--en-shadow-color);")?;
                }
                "xl" => {
                    writeln!(buffer, "--en-shadow: 0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1);")?;
                    indent(indentation, buffer)?;
                    writeln!(buffer, "--en-shadow-colored: 0 20px 25px -5px var(--en-shadow-color), 0 8px 10px -6px var(--en-shadow-color);")?;
                }
                "2xl" => {
                    writeln!(buffer, "--en-shadow: 0 25px 50px -12px rgb(0 0 0 / 0.25);")?;
                    indent(indentation, buffer)?;
                    writeln!(
                        buffer,
                        "--en-shadow-colored: 0 25px 50px -12px var(--en-shadow-color);"
                    )?;
                }
                "inner" => {
                    writeln!(buffer, "--en-shadow: inset 0 2px 4px 0 rgb(0 0 0 / 0.05);")?;
                    indent(indentation, buffer)?;
                    writeln!(
                        buffer,
                        "--en-shadow-colored: inset 0 2px 4px 0 var(--en-shadow-color);"
                    )?;
                }
                "none" => writeln!(buffer, "box-shadow: none;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "--en-shadow: {value};")?;
                indent(indentation, buffer)?;
                let mut shadow = shadow::parse_shadow(value).unwrap();
                shadow.replace_all_colors("var(--en-shadow-color)");
                writeln!(buffer, "--en-shadow-colored: {};", shadow)?
            }
        }

        indent(indentation, buffer)?;
        writeln!(buffer, "{}", CSS_SHADOW)?;

        Ok(())
    }
}

pub struct BoxShadowColorPlugin;

impl Plugin for BoxShadowColorPlugin {
    fn namespace(&self) -> &str {
        "shadow"
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
        indent(indentation, buffer)?;
        let value = match modifier {
            Modifier::Basic { value, .. } => default_colors::get(config, value, None).unwrap(),
            Modifier::Arbitrary { value, .. } => Cow::from(&**value),
        };
        writeln!(buffer, "--en-shadow-color: {value};")?;

        indent(indentation, buffer)?;
        writeln!(buffer, "--en-shadow: var(--en-shadow-colored);")?;

        Ok(())
    }
}

pub struct OpacityPlugin;

impl Plugin for OpacityPlugin {
    fn namespace(&self) -> &str {
        "opacity"
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
                "opacity: {};",
                value.parse::<usize>().unwrap() as f32 / 100.
            )?,
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
