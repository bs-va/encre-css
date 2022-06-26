use super::{to_css_value, Plugin};
use crate::{
    context::{ContextCanHandle, ContextHandle},
    selector::Modifier,
    utils::{format_negative, indent, value_matchers::*},
};

use std::fmt::{self, Write};

const CSS_FILTER: &str = "filter: var(--en-blur) var(--en-brightness) var(--en-contrast) var(--en-grayscale) var(--en-hue-rotate) var(--en-invert) var(--en-saturate) var(--en-sepia) var(--en-drop-shadow);";
const CSS_BACKDROP_FILTER_1: &str = "-webkit-backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);";
const CSS_BACKDROP_FILTER_2: &str = "backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);";

#[derive(Debug)]
pub struct FilterPlugin;

impl Plugin for FilterPlugin {
    fn namespace(&self) -> &str {
        "filter"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => ["", "none"].contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "" => writeln!(context.buffer, "{}", CSS_FILTER)?,
                "none" => writeln!(context.buffer, "filter: none;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct BlurPlugin;

impl Plugin for BlurPlugin {
    fn namespace(&self) -> &str {
        "blur"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => {
                ["", "sm", "md", "lg", "xl", "2xl", "3xl", "none"].contains(&&**value)
            }
            Modifier::Arbitrary { value, .. } => is_matching_length(value),
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "" => writeln!(context.buffer, "--en-blur: blur(8px);")?,
                "sm" => writeln!(context.buffer, "--en-blur: blur(4px);")?,
                "md" => writeln!(context.buffer, "--en-blur: blur(12px);")?,
                "lg" => writeln!(context.buffer, "--en-blur: blur(16px);")?,
                "xl" => writeln!(context.buffer, "--en-blur: blur(24px);")?,
                "2xl" => writeln!(context.buffer, "--en-blur: blur(40px);")?,
                "3xl" => writeln!(context.buffer, "--en-blur: blur(64px);")?,
                "none" => writeln!(context.buffer, "--en-blur: blur(0);")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "--en-blur: blur({});", to_css_value(value))?
            }
        }

        indent(context.indentation, context.buffer)?;
        writeln!(context.buffer, "{}", CSS_FILTER)?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct BrightnessPlugin;

impl Plugin for BrightnessPlugin {
    fn namespace(&self) -> &str {
        "brightness"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => value.parse::<usize>().is_ok(),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        // NOTE: Not-compatible with TailwindCSS, support all values
        match context.modifier {
            Modifier::Basic { value, .. } => writeln!(
                context.buffer,
                "--en-brightness: brightness({});",
                value.parse::<usize>().unwrap() as f32 / 100.
            )?,
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        indent(context.indentation, context.buffer)?;
        writeln!(context.buffer, "{}", CSS_FILTER)?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct ContrastPlugin;

impl Plugin for ContrastPlugin {
    fn namespace(&self) -> &str {
        "contrast"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => value.parse::<usize>().is_ok(),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        // NOTE: Not-compatible with TailwindCSS, support all values
        match context.modifier {
            Modifier::Basic { value, .. } => writeln!(
                context.buffer,
                "--en-contrast: contrast({});",
                value.parse::<usize>().unwrap() as f32 / 100.
            )?,
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        indent(context.indentation, context.buffer)?;
        writeln!(context.buffer, "{}", CSS_FILTER)?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct DropShadowPlugin;

impl Plugin for DropShadowPlugin {
    fn namespace(&self) -> &str {
        "drop-shadow"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => {
                ["", "sm", "md", "lg", "xl", "2xl", "none"].contains(&&**value)
            }
            Modifier::Arbitrary { value, .. } => is_matching_shadow(value),
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "" => writeln!(context.buffer, "--en-drop-shadow: drop-shadow(0 1px 2px rgb(0 0 0 / 0.1)) drop-shadow(0 1px 1px rgb(0 0 0 / 0.06));")?,
                "sm" => writeln!(context.buffer, "--en-drop-shadow: drop-shadow(0 1px 1px rgb(0 0 0 / 0.05));")?,
                "md" => writeln!(context.buffer, "--en-drop-shadow: drop-shadow(0 4px 3px rgb(0 0 0 / 0.07)) drop-shadow(0 2px 2px rgb(0 0 0 / 0.06));")?,
                "lg" => writeln!(context.buffer, "--en-drop-shadow: drop-shadow(0 10px 8px rgb(0 0 0 / 0.04)) drop-shadow(0 4px 3px rgb(0 0 0 / 0.1));")?,
                "xl" => writeln!(context.buffer, "--en-drop-shadow: drop-shadow(0 20px 13px rgb(0 0 0 / 0.03)) drop-shadow(0 8px 5px rgb(0 0 0 / 0.08));")?,
                "2xl" => writeln!(context.buffer, "--en-drop-shadow: drop-shadow(0 25px 25px rgb(0 0 0 / 0.15));")?,
                "none" => writeln!(context.buffer, "--en-drop-shadow: drop-shadow(0 0 #0000);")?,
                _ => unreachable!(),
            }
            Modifier::Arbitrary { value, .. } => writeln!(context.buffer, "--en-drop-shadow: drop-shadow({});", to_css_value(value))?,
        }

        indent(context.indentation, context.buffer)?;
        writeln!(context.buffer, "{}", CSS_FILTER)?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct GrayscalePlugin;

impl Plugin for GrayscalePlugin {
    fn namespace(&self) -> &str {
        "grayscale"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => ["", "0"].contains(&&**value),
            Modifier::Arbitrary { value, .. } => {
                is_matching_float(value) || is_matching_percentage(value)
            }
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "" => writeln!(context.buffer, "--en-grayscale: grayscale(100%);")?,
                "0" => writeln!(context.buffer, "--en-grayscale: grayscale(0);")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => writeln!(
                context.buffer,
                "--en-grayscale: grayscale({});",
                to_css_value(value)
            )?,
        }

        indent(context.indentation, context.buffer)?;
        writeln!(context.buffer, "{}", CSS_FILTER)?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct HueRotatePlugin;

impl Plugin for HueRotatePlugin {
    fn namespace(&self) -> &str {
        "hue-rotate"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => value.parse::<usize>().is_ok(),
            Modifier::Arbitrary { value, .. } => is_matching_angle(value),
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { is_negative, value } => writeln!(
                context.buffer,
                "--en-hue-rotate: hue-rotate({}{}deg);",
                format_negative(is_negative),
                value
            )?,
            Modifier::Arbitrary { value, .. } => writeln!(
                context.buffer,
                "--en-hue-rotate: hue-rotate({});",
                to_css_value(value)
            )?,
        }

        indent(context.indentation, context.buffer)?;
        writeln!(context.buffer, "{}", CSS_FILTER)?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct InvertPlugin;

impl Plugin for InvertPlugin {
    fn namespace(&self) -> &str {
        "invert"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => ["", "0"].contains(&&**value),
            Modifier::Arbitrary { value, .. } => {
                is_matching_float(value) || is_matching_percentage(value)
            }
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "" => writeln!(context.buffer, "--en-invert: invert(100%);")?,
                "0" => writeln!(context.buffer, "--en-invert: invert(0);")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "--en-invert: invert({});", to_css_value(value))?
            }
        }

        indent(context.indentation, context.buffer)?;
        writeln!(context.buffer, "{}", CSS_FILTER)?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct SaturatePlugin;

impl Plugin for SaturatePlugin {
    fn namespace(&self) -> &str {
        "saturate"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => value.parse::<usize>().is_ok(),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        // NOTE: Not-compatible with TailwindCSS, support all values
        match context.modifier {
            Modifier::Basic { value, .. } => writeln!(
                context.buffer,
                "--en-saturate: saturate({});",
                value.parse::<usize>().unwrap() as f32 / 100.
            )?,
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        indent(context.indentation, context.buffer)?;
        writeln!(context.buffer, "{}", CSS_FILTER)?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct SepiaPlugin;

impl Plugin for SepiaPlugin {
    fn namespace(&self) -> &str {
        "sepia"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => ["", "0"].contains(&&**value),
            Modifier::Arbitrary { value, .. } => {
                is_matching_float(value) || is_matching_percentage(value)
            }
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "" => writeln!(context.buffer, "--en-sepia: sepia(100%);")?,
                "0" => writeln!(context.buffer, "--en-sepia: sepia(0);")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "--en-sepia: sepia({});", to_css_value(value))?
            }
        }

        indent(context.indentation, context.buffer)?;
        writeln!(context.buffer, "{}", CSS_FILTER)?;

        Ok(())
    }
}

// Backdrop

#[derive(Debug)]
pub struct BackdropFilterPlugin;

impl Plugin for BackdropFilterPlugin {
    fn namespace(&self) -> &str {
        "backdrop-filter"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => ["", "none"].contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "" => {
                    writeln!(context.buffer, "{}", CSS_BACKDROP_FILTER_1)?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "{}", CSS_BACKDROP_FILTER_2)?;
                }
                "none" => {
                    writeln!(context.buffer, "-webkit-backdrop-filter: none;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "backdrop-filter: none;")?;
                }
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct BackdropBlurPlugin;

impl Plugin for BackdropBlurPlugin {
    fn namespace(&self) -> &str {
        "backdrop-blur"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => {
                ["", "sm", "md", "lg", "xl", "2xl", "3xl", "none"].contains(&&**value)
            }
            Modifier::Arbitrary { value, .. } => is_matching_length(value),
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "" => writeln!(context.buffer, "--en-backdrop-blur: blur(8px);")?,
                "sm" => writeln!(context.buffer, "--en-backdrop-blur: blur(4px);")?,
                "md" => writeln!(context.buffer, "--en-backdrop-blur: blur(12px);")?,
                "lg" => writeln!(context.buffer, "--en-backdrop-blur: blur(16px);")?,
                "xl" => writeln!(context.buffer, "--en-backdrop-blur: blur(24px);")?,
                "2xl" => writeln!(context.buffer, "--en-backdrop-blur: blur(40px);")?,
                "3xl" => writeln!(context.buffer, "--en-backdrop-blur: blur(64px);")?,
                "none" => writeln!(context.buffer, "--en-backdrop-blur: blur(0);")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "--en-backdrop-blur: blur({});", to_css_value(value))?
            }
        }

        indent(context.indentation, context.buffer)?;
        writeln!(context.buffer, "{}", CSS_BACKDROP_FILTER_1)?;
        indent(context.indentation, context.buffer)?;
        writeln!(context.buffer, "{}", CSS_BACKDROP_FILTER_2)?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct BackdropBrightnessPlugin;

impl Plugin for BackdropBrightnessPlugin {
    fn namespace(&self) -> &str {
        "backdrop-brightness"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => value.parse::<usize>().is_ok(),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        // NOTE: Not-compatible with TailwindCSS, support all values
        match context.modifier {
            Modifier::Basic { value, .. } => writeln!(
                context.buffer,
                "--en-backdrop-brightness: brightness({});",
                value.parse::<usize>().unwrap() as f32 / 100.
            )?,
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        indent(context.indentation, context.buffer)?;
        writeln!(context.buffer, "{}", CSS_BACKDROP_FILTER_1)?;
        indent(context.indentation, context.buffer)?;
        writeln!(context.buffer, "{}", CSS_BACKDROP_FILTER_2)?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct BackdropContrastPlugin;

impl Plugin for BackdropContrastPlugin {
    fn namespace(&self) -> &str {
        "backdrop-contrast"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => value.parse::<usize>().is_ok(),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        // NOTE: Not-compatible with TailwindCSS, support all values
        match context.modifier {
            Modifier::Basic { value, .. } => writeln!(
                context.buffer,
                "--en-backdrop-contrast: contrast({});",
                value.parse::<usize>().unwrap() as f32 / 100.
            )?,
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        indent(context.indentation, context.buffer)?;
        writeln!(context.buffer, "{}", CSS_BACKDROP_FILTER_1)?;
        indent(context.indentation, context.buffer)?;
        writeln!(context.buffer, "{}", CSS_BACKDROP_FILTER_2)?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct BackdropGrayscalePlugin;

impl Plugin for BackdropGrayscalePlugin {
    fn namespace(&self) -> &str {
        "backdrop-grayscale"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => ["", "0"].contains(&&**value),
            Modifier::Arbitrary { value, .. } => {
                is_matching_float(value) || is_matching_percentage(value)
            }
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "" => writeln!(context.buffer, "--en-backdrop-grayscale: grayscale(100%);")?,
                "0" => writeln!(context.buffer, "--en-backdrop-grayscale: grayscale(0);")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => writeln!(
                context.buffer,
                "--en-backdrop-grayscale: grayscale({});",
                to_css_value(value)
            )?,
        }

        indent(context.indentation, context.buffer)?;
        writeln!(context.buffer, "{}", CSS_BACKDROP_FILTER_1)?;
        indent(context.indentation, context.buffer)?;
        writeln!(context.buffer, "{}", CSS_BACKDROP_FILTER_2)?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct BackdropHueRotatePlugin;

impl Plugin for BackdropHueRotatePlugin {
    fn namespace(&self) -> &str {
        "backdrop-hue-rotate"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => value.parse::<usize>().is_ok(),
            Modifier::Arbitrary { value, .. } => is_matching_angle(value),
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { is_negative, value } => writeln!(
                context.buffer,
                "--en-backdrop-hue-rotate: hue-rotate({}{}deg);",
                format_negative(is_negative),
                value
            )?,
            Modifier::Arbitrary { value, .. } => writeln!(
                context.buffer,
                "--en-backdrop-hue-rotate: hue-rotate({});",
                to_css_value(value)
            )?,
        }

        indent(context.indentation, context.buffer)?;
        writeln!(context.buffer, "{}", CSS_BACKDROP_FILTER_1)?;
        indent(context.indentation, context.buffer)?;
        writeln!(context.buffer, "{}", CSS_BACKDROP_FILTER_2)?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct BackdropInvertPlugin;

impl Plugin for BackdropInvertPlugin {
    fn namespace(&self) -> &str {
        "backdrop-invert"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => ["", "0"].contains(&&**value),
            Modifier::Arbitrary { value, .. } => {
                is_matching_float(value) || is_matching_percentage(value)
            }
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "" => writeln!(context.buffer, "--en-backdrop-invert: invert(100%);")?,
                "0" => writeln!(context.buffer, "--en-backdrop-invert: invert(0);")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => writeln!(
                context.buffer,
                "--en-backdrop-invert: invert({});",
                to_css_value(value)
            )?,
        }

        indent(context.indentation, context.buffer)?;
        writeln!(context.buffer, "{}", CSS_BACKDROP_FILTER_1)?;
        indent(context.indentation, context.buffer)?;
        writeln!(context.buffer, "{}", CSS_BACKDROP_FILTER_2)?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct BackdropOpacityPlugin;

impl Plugin for BackdropOpacityPlugin {
    fn namespace(&self) -> &str {
        "backdrop-opacity"
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
                "--en-backdrop-opacity: {};",
                value.parse::<usize>().unwrap() as f32 / 100.
            )?,
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct BackdropSaturatePlugin;

impl Plugin for BackdropSaturatePlugin {
    fn namespace(&self) -> &str {
        "backdrop-saturate"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => value.parse::<usize>().is_ok(),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        // NOTE: Not-compatible with TailwindCSS, support all values
        match context.modifier {
            Modifier::Basic { value, .. } => writeln!(
                context.buffer,
                "--en-backdrop-saturate: saturate({});",
                value.parse::<usize>().unwrap() as f32 / 100.
            )?,
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        indent(context.indentation, context.buffer)?;
        writeln!(context.buffer, "{}", CSS_BACKDROP_FILTER_1)?;
        indent(context.indentation, context.buffer)?;
        writeln!(context.buffer, "{}", CSS_BACKDROP_FILTER_2)?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct BackdropSepiaPlugin;

impl Plugin for BackdropSepiaPlugin {
    fn namespace(&self) -> &str {
        "backdrop-sepia"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => ["", "0"].contains(&&**value),
            Modifier::Arbitrary { value, .. } => {
                is_matching_float(value) || is_matching_percentage(value)
            }
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "" => writeln!(context.buffer, "--en-backdrop-sepia: sepia(100%);")?,
                "0" => writeln!(context.buffer, "--en-backdrop-sepia: sepia(0);")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => writeln!(
                context.buffer,
                "--en-backdrop-sepia: sepia({});",
                to_css_value(value)
            )?,
        }

        indent(context.indentation, context.buffer)?;
        writeln!(context.buffer, "{}", CSS_BACKDROP_FILTER_1)?;
        indent(context.indentation, context.buffer)?;
        writeln!(context.buffer, "{}", CSS_BACKDROP_FILTER_2)?;

        Ok(())
    }
}
