use super::Plugin;
use crate::utils::{default_colors, indent, value_matchers::*};
use crate::{config::Config, selector::Modifier};

use lazy_static::lazy_static;
use regex::Regex;
use std::{
    borrow::Cow,
    fmt::{self, Write},
};

pub const CSS_FONT_VARIANT_NUMERIC: &str = "font-variant-numeric: var(--en-ordinal) var(--en-slashed-zero) var(--en-numeric-figure) var(--en-numeric-spacing) var(--en-numeric-fraction);";

lazy_static! {
    static ref START_WITH_INT_REGEX: Regex = Regex::new(r"(?-u)^\d").unwrap();
}

pub struct ColorPlugin;

impl Plugin for ColorPlugin {
    fn namespace(&self) -> &str {
        "text"
    }

    fn can_handle(&self, config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => {
                default_colors::get(config, value, Some("--en-text-opacity")).is_some()
            }
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
        match modifier {
            Modifier::Basic { value, .. } => {
                let color = default_colors::get(config, value, Some("--en-text-opacity")).unwrap();
                if color.contains("--en-text-opacity") {
                    writeln!(buffer, "--en-text-opacity: 1;")?;
                    indent(indentation, buffer)?;
                }

                writeln!(buffer, "color: {color};")?;
            }
            Modifier::Arbitrary { value, .. } => writeln!(buffer, "color: {value};")?,
        }

        Ok(())
    }
}

pub struct OpacityPlugin;

impl Plugin for OpacityPlugin {
    fn namespace(&self) -> &str {
        "text-opacity"
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

pub struct FontFamilyPlugin;

impl Plugin for FontFamilyPlugin {
    fn namespace(&self) -> &str {
        "font"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => ["sans", "serif", "mono"].contains(&&**value),
            Modifier::Arbitrary { value, .. } => value.split(',').all(|v| {
                if is_matching_generic_name(v) || is_matching_var(v) {
                    true
                } else {
                    !START_WITH_INT_REGEX.is_match(v)
                }
            }),
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
                "sans" => writeln!(
                    buffer,
                    r#"font-family: ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, "Noto Sans", sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji";"#
                )?,
                "serif" => writeln!(
                    buffer,
                    r#"font-family: Georgia, Cambria, "Times New Roman", Times, serif;"#
                )?,
                "mono" => writeln!(
                    buffer,
                    r#"font-family: Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;"#
                )?,
                _ => unreachable!(),
            },

            // NOTE: Not-compatible with TailwindCSS, it is not needed to add quotes to fonts
            // containing spaces, they are added later
            Modifier::Arbitrary { value, .. } => writeln!(
                buffer,
                "font-family: {maybe_quote}{value}{maybe_quote};",
                maybe_quote = if value.contains(' ') { "\"" } else { "" }
            )?,
        }

        Ok(())
    }
}

pub struct FontSizePlugin;

impl Plugin for FontSizePlugin {
    fn namespace(&self) -> &str {
        "text"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => [
                "xs", "sm", "base", "lg", "xl", "2xl", "3xl", "4xl", "5xl", "6xl", "7xl", "8xl",
                "9xl",
            ]
            .contains(&&**value),
            Modifier::Arbitrary { hint, value } => {
                hint == "length"
                    || is_matching_length(value)
                    || is_matching_absolute_size(value)
                    || is_matching_relative_size(value)
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
                "xs" => {
                    writeln!(buffer, "font-size: 0.75rem;")?;
                    indent(indentation, buffer)?;
                    writeln!(buffer, "line-height: 1rem;")?;
                }
                "sm" => {
                    writeln!(buffer, "font-size: 0.875rem;")?;
                    indent(indentation, buffer)?;
                    writeln!(buffer, "line-height: 1.25rem;")?;
                }
                "base" => {
                    writeln!(buffer, "font-size: 1rem;")?;
                    indent(indentation, buffer)?;
                    writeln!(buffer, "line-height: 1.5rem;")?;
                }
                "lg" => {
                    writeln!(buffer, "font-size: 1.125rem;")?;
                    indent(indentation, buffer)?;
                    writeln!(buffer, "line-height: 1.75rem;")?;
                }
                "xl" => {
                    writeln!(buffer, "font-size: 1.25rem;")?;
                    indent(indentation, buffer)?;
                    writeln!(buffer, "line-height: 1.75rem;")?;
                }
                "2xl" => {
                    writeln!(buffer, "font-size: 1.5rem;")?;
                    indent(indentation, buffer)?;
                    writeln!(buffer, "line-height: 2rem;")?;
                }
                "3xl" => {
                    writeln!(buffer, "font-size: 1.875rem;")?;
                    indent(indentation, buffer)?;
                    writeln!(buffer, "line-height: 2.25rem;")?;
                }
                "4xl" => {
                    writeln!(buffer, "font-size: 2.25rem;")?;
                    indent(indentation, buffer)?;
                    writeln!(buffer, "line-height: 2.5rem;")?;
                }
                "5xl" => {
                    writeln!(buffer, "font-size: 3rem;")?;
                    indent(indentation, buffer)?;
                    writeln!(buffer, "line-height: 1;")?;
                }
                "6xl" => {
                    writeln!(buffer, "font-size: 3.75rem;")?;
                    indent(indentation, buffer)?;
                    writeln!(buffer, "line-height: 1;")?;
                }
                "7xl" => {
                    writeln!(buffer, "font-size: 4.5rem;")?;
                    indent(indentation, buffer)?;
                    writeln!(buffer, "line-height: 1;")?;
                }
                "8xl" => {
                    writeln!(buffer, "font-size: 6rem;")?;
                    indent(indentation, buffer)?;
                    writeln!(buffer, "line-height: 1;")?;
                }
                "9xl" => {
                    writeln!(buffer, "font-size: 8rem;")?;
                    indent(indentation, buffer)?;
                    writeln!(buffer, "line-height: 1;")?;
                }
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => writeln!(buffer, "font-size: {value};")?,
        }

        Ok(())
    }
}

pub struct FontWeightPlugin;

impl Plugin for FontWeightPlugin {
    fn namespace(&self) -> &str {
        "font"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => [
                "thin",
                "extralight",
                "light",
                "normal",
                "medium",
                "semibold",
                "bold",
                "extrabold",
                "black",
            ]
            .contains(&&**value),
            Modifier::Arbitrary { value, .. } => {
                ["normal", "bold", "lighter", "bolder"].contains(&&**value)
                    || is_matching_number(value)
                    || is_matching_var(value)
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
                "thin" => writeln!(buffer, "font-weight: 100;")?,
                "extralight" => writeln!(buffer, "font-weight: 200;")?,
                "light" => writeln!(buffer, "font-weight: 300;")?,
                "normal" => writeln!(buffer, "font-weight: 400;")?,
                "medium" => writeln!(buffer, "font-weight: 500;")?,
                "semibold" => writeln!(buffer, "font-weight: 600;")?,
                "bold" => writeln!(buffer, "font-weight: 700;")?,
                "extrabold" => writeln!(buffer, "font-weight: 800;")?,
                "black" => writeln!(buffer, "font-weight: 900;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => writeln!(buffer, "font-weight: {value};")?,
        }

        Ok(())
    }
}

pub struct TextAlignmentPlugin;

impl Plugin for TextAlignmentPlugin {
    fn namespace(&self) -> &str {
        "text"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => {
                ["left", "center", "right", "justify"].contains(&&**value)
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
                "left" => writeln!(buffer, "text-align: left;")?,
                "center" => writeln!(buffer, "text-align: center;")?,
                "right" => writeln!(buffer, "text-align: right;")?,
                "justify" => writeln!(buffer, "text-align: justify;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

pub struct TextTransformPlugin;

impl Plugin for TextTransformPlugin {
    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => {
                ["uppercase", "lowercase", "capitalize", "normal-case"].contains(&&**value)
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
                "uppercase" => writeln!(buffer, "text-transform: uppercase;")?,
                "lowercase" => writeln!(buffer, "text-transform: lowercase;")?,
                "capitalize" => writeln!(buffer, "text-transform: capitalize;")?,
                "normal-case" => writeln!(buffer, "text-transform: none;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

pub struct TrackingPlugin;

impl Plugin for TrackingPlugin {
    fn namespace(&self) -> &str {
        "tracking"
    }
    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => {
                ["tighter", "tight", "normal", "wide", "wider", "widest"].contains(&&**value)
            }
            Modifier::Arbitrary { value, .. } => value == "normal" || is_matching_length(value),
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
                "tighter" => writeln!(buffer, "letter-spacing: -0.05em;")?,
                "tight" => writeln!(buffer, "letter-spacing: -0.025em;")?,
                "normal" => writeln!(buffer, "letter-spacing: 0;")?,
                "wide" => writeln!(buffer, "letter-spacing: 0.025em;")?,
                "wider" => writeln!(buffer, "letter-spacing: 0.05em;")?,
                "widest" => writeln!(buffer, "letter-spacing: 0.1em;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => writeln!(buffer, "letter-spacing: {value};")?,
        }

        Ok(())
    }
}

pub struct LeadingPlugin;

impl Plugin for LeadingPlugin {
    fn namespace(&self) -> &str {
        "leading"
    }
    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => [
                "none", "tight", "snug", "relaxed", "loose", "3", "4", "5", "6", "7", "8", "9",
                "10",
            ]
            .contains(&&**value),
            // https://developer.mozilla.org/en-US/docs/Web/CSS/line-height#values
            Modifier::Arbitrary { value, .. } => {
                value == "normal"
                    || is_matching_float(value)
                    || is_matching_length(value)
                    || is_matching_percentage(value)
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
                "none" => writeln!(buffer, "line-height: 1;")?,
                "tight" => writeln!(buffer, "line-height: 1.25;")?,
                "snug" => writeln!(buffer, "line-height: 1.375;")?,
                "normal" => writeln!(buffer, "line-height: 1.5;")?,
                "relaxed" => writeln!(buffer, "line-height: 1.625;")?,
                "loose" => writeln!(buffer, "line-height: 2;")?,
                "3" => writeln!(buffer, "line-height: .75rem;")?,
                "4" => writeln!(buffer, "line-height: 1rem;")?,
                "5" => writeln!(buffer, "line-height: 1.25rem;")?,
                "6" => writeln!(buffer, "line-height: 1.5rem;")?,
                "7" => writeln!(buffer, "line-height: 1.75rem;")?,
                "8" => writeln!(buffer, "line-height: 2rem;")?,
                "9" => writeln!(buffer, "line-height: 2.25rem;")?,
                "10" => writeln!(buffer, "line-height: 2.5rem;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => writeln!(buffer, "line-height: {value};")?,
        }

        Ok(())
    }
}

pub struct ItalicPlugin;

impl Plugin for ItalicPlugin {
    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => ["italic", "no-italic"].contains(&&**value),
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
                "italic" => writeln!(buffer, "font-style: italic;")?,
                "no-italic" => writeln!(buffer, "font-style: normal;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

pub struct TextDecorationPlugin;

impl Plugin for TextDecorationPlugin {
    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => {
                ["underline", "overline", "line-through", "no-underline"].contains(&value.as_str())
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
            Modifier::Basic { value, .. } => {
                writeln!(buffer, "-webkit-text-decoration-line: {value};")?;

                indent(indentation, buffer)?;
                writeln!(buffer, "text-decoration-line: {value};")?;
            }
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

pub struct TextDecorationColorPlugin;

impl Plugin for TextDecorationColorPlugin {
    fn namespace(&self) -> &str {
        "decoration"
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

        writeln!(buffer, "-webkit-text-decoration-color: {value};")?;
        indent(indentation, buffer)?;
        writeln!(buffer, "text-decoration-color: {value};")
    }
}

pub struct TextDecorationStylePlugin;

impl Plugin for TextDecorationStylePlugin {
    fn namespace(&self) -> &str {
        "decoration"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => {
                ["solid", "double", "dotted", "dashed", "wavy"].contains(&&**value)
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
            Modifier::Basic { value, .. } => writeln!(buffer, "text-decoration-style: {value};")?,
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

pub struct TextDecorationThicknessPlugin;

impl Plugin for TextDecorationThicknessPlugin {
    fn namespace(&self) -> &str {
        "decoration"
    }
    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => {
                ["auto", "from-font"].contains(&&**value) || value.parse::<usize>().is_ok()
            }
            Modifier::Arbitrary { hint, value } => {
                hint == "length"
                    || hint == "percentage"
                    || ["auto", "from-font"].contains(&&**value)
                    || is_matching_length(value)
                    || is_matching_percentage(value)
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
            Modifier::Basic { value, .. } => {
                if ["auto", "from-font"].contains(&&**value) {
                    return writeln!(buffer, "text-decoration-thickness: {value};");
                }

                // NOTE: Not-compatible with TailwindCSS, support all values
                writeln!(buffer, "text-decoration-thickness: {value}px;")?;
            }
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "text-decoration-thickness: {value};")?
            }
        }

        Ok(())
    }
}

pub struct TextDecorationOffsetPlugin;

impl Plugin for TextDecorationOffsetPlugin {
    fn namespace(&self) -> &str {
        "underline-offset"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => value.parse::<usize>().is_ok() || value == "auto",
            Modifier::Arbitrary { hint, value } => {
                hint == "length"
                    || hint == "percentage"
                    || value == "auto"
                    || is_matching_length(value)
                    || is_matching_percentage(value)
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
            Modifier::Basic { value, .. } => {
                if value == "auto" {
                    return writeln!(buffer, "text-underline-offset: auto;");
                }

                // NOTE: Not-compatible with TailwindCSS, support all values
                writeln!(buffer, "text-underline-offset: {value}px;")?;
            }
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "text-underline-offset: {value};")?
            }
        }

        Ok(())
    }
}

pub struct ContentPlugin;

impl Plugin for ContentPlugin {
    fn namespace(&self) -> &str {
        "content"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => value == "none",
            Modifier::Arbitrary { value, .. } => is_matching_all(value),
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
            Modifier::Basic { .. } => {
                writeln!(buffer, "--en-content: none;")?;
                indent(indentation, buffer)?;
                writeln!(buffer, "content: var(--en-content);")?;
            }
            Modifier::Arbitrary { value, .. } => {
                // NOTE: Not-compatible with TailwindCSS, it is not needed to add quotes to `content`
                // containing spaces, they are added later
                writeln!(buffer, "--en-content: \"{value}\";")?;
                indent(indentation, buffer)?;
                writeln!(buffer, "content: var(--en-content);")?;
            }
        }

        Ok(())
    }
}

pub struct FontVariantNumericPlugin;

impl Plugin for FontVariantNumericPlugin {
    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => [
                "normal-nums",
                "ordinal",
                "slashed-zero",
                "lining-nums",
                "oldstyle-nums",
                "proportional-nums",
                "tabular-nums",
                "diagonal-fractions",
                "stacked-fractions",
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
                "normal-nums" => return writeln!(buffer, "font-variant-numeric: normal;"),
                "ordinal" => writeln!(buffer, "--en-ordinal: ordinal;")?,
                "slashed-zero" => writeln!(buffer, "--en-slashed-zero: slashed-zero;")?,
                "lining-nums" => writeln!(buffer, "--en-numeric-figure: lining-nums;")?,
                "oldstyle-nums" => writeln!(buffer, "--en-numeric-figure: oldstyle-nums;")?,
                "proportional-nums" => {
                    writeln!(buffer, "--en-numeric-spacing: proportional-nums;")?
                }
                "tabular-nums" => writeln!(buffer, "--en-numeric-spacing: tabular-nums;")?,
                "diagonal-fractions" => {
                    writeln!(buffer, "--en-numeric-fraction: diagonal-fractions;")?
                }
                "stacked-fractions" => {
                    writeln!(buffer, "--en-numeric-fraction: stacked-fractions;")?
                }
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        indent(indentation, buffer)?;
        writeln!(buffer, "{}", CSS_FONT_VARIANT_NUMERIC)?;

        Ok(())
    }
}

pub struct FontSmoothingPlugin;

impl Plugin for FontSmoothingPlugin {
    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => {
                ["antialised", "subpixel-antialised"].contains(&&**value)
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
                "antialised" => {
                    writeln!(buffer, "-webkit-font-smoothing: antialiased;")?;
                    indent(indentation, buffer)?;
                    writeln!(buffer, "-moz-osx-font-smoothing: grayscale;")?;
                }
                "subpixel-antialised" => {
                    writeln!(buffer, "-webkit-font-smoothing: auto;")?;
                    indent(indentation, buffer)?;
                    writeln!(buffer, "-moz-osx-font-smoothing: auto;")?;
                }
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

pub struct ListStyleTypePlugin;

impl Plugin for ListStyleTypePlugin {
    fn namespace(&self) -> &str {
        "list"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => ["disc", "decimal", "none"].contains(&&**value),
            Modifier::Arbitrary { value, .. } => is_matching_all(value), // TODO: Better matching
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
            Modifier::Basic { value, .. } => writeln!(buffer, "list-style-type: {value};")?,
            Modifier::Arbitrary { value, .. } => writeln!(buffer, "list-style-type: {value};")?,
        }

        Ok(())
    }
}

pub struct ListStylePositionPlugin;

impl Plugin for ListStylePositionPlugin {
    fn namespace(&self) -> &str {
        "list"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => ["inside", "outside"].contains(&&**value),
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
            Modifier::Basic { value, .. } => writeln!(buffer, "list-style-position: {value};")?,
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

pub struct VerticalAlignPlugin;

impl Plugin for VerticalAlignPlugin {
    fn namespace(&self) -> &str {
        "align"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => [
                "baseline",
                "top",
                "middle",
                "bottom",
                "text-top",
                "text-bottom",
                "sub",
                "super",
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
            Modifier::Basic { value, .. } => writeln!(buffer, "vertical-align: {value};")?,
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

pub struct TextOverflowPlugin;

impl Plugin for TextOverflowPlugin {
    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => {
                ["truncate", "text-ellipsis", "text-clip"].contains(&&**value)
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
                "truncate" => {
                    writeln!(buffer, "overflow: hidden;")?;
                    indent(indentation, buffer)?;
                    writeln!(buffer, "text-overflow: ellipsis;")?;
                    indent(indentation, buffer)?;
                    writeln!(buffer, "white-space: nowrap;")?
                }
                "text-ellipsis" => writeln!(buffer, "text-overflow: ellipsis;")?,
                "text-clip" => writeln!(buffer, "text-overflow: clip;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

pub struct WhitespacePlugin;

impl Plugin for WhitespacePlugin {
    fn namespace(&self) -> &str {
        "whitespace"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => {
                ["normal", "nowrap", "pre", "pre-line", "pre-wrap"].contains(&&**value)
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
            Modifier::Basic { value, .. } => writeln!(buffer, "white-space: {value};")?,
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

pub struct WordBreakPlugin;

impl Plugin for WordBreakPlugin {
    fn namespace(&self) -> &str {
        "break"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => ["normal", "words", "all"].contains(&&**value),
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
                "normal" => {
                    writeln!(buffer, "overflow-wrap: normal;")?;
                    indent(indentation, buffer)?;
                    writeln!(buffer, "word-break: normal;")?;
                }
                "words" => writeln!(buffer, "overflow-wrap: break-word;")?,
                "all" => writeln!(buffer, "word-break: break-all;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
