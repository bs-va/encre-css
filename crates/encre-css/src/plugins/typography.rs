use super::{to_css_value, Plugin};
use crate::{
    context::{ContextCanHandle, ContextHandle},
    selector::Modifier,
    utils::{color, indent, length, value_matchers::*},
};

use std::{
    borrow::Cow,
    fmt::{self, Write},
};

pub const CSS_FONT_VARIANT_NUMERIC: &str = "font-variant-numeric: var(--en-ordinal) var(--en-slashed-zero) var(--en-numeric-figure) var(--en-numeric-spacing) var(--en-numeric-fraction);";

#[derive(Debug)]
pub struct ColorPlugin;

impl Plugin for ColorPlugin {
    fn namespace(&self) -> &str {
        "text"
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
                let color = color::get(context.config, value, Some("--en-text-opacity")).unwrap();
                if color.contains("--en-text-opacity") {
                    writeln!(context.buffer, "--en-text-opacity: 1;")?;
                    indent(context.indentation, context.buffer)?;
                }

                writeln!(context.buffer, "color: {color};")?;
            }
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "color: {};", to_css_value(value))?
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct OpacityPlugin;

impl Plugin for OpacityPlugin {
    fn namespace(&self) -> &str {
        "text-opacity"
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
pub struct FontFamilyPlugin;

impl Plugin for FontFamilyPlugin {
    fn namespace(&self) -> &str {
        "font"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => ["sans", "serif", "mono"].contains(&&**value),
            Modifier::Arbitrary { value, .. } => {
                value.split(',').all(|v| v.parse::<usize>().is_err())
            }
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "sans" => writeln!(
                    context.buffer,
                    r#"font-family: ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, "Noto Sans", sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji";"#
                )?,
                "serif" => writeln!(
                    context.buffer,
                    r#"font-family: Georgia, Cambria, "Times New Roman", Times, serif;"#
                )?,
                "mono" => writeln!(
                    context.buffer,
                    r#"font-family: Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;"#
                )?,
                _ => unreachable!(),
            },

            // NOTE: Not-compatible with TailwindCSS, it is not needed to add quotes to fonts
            // containing spaces, they are added later
            Modifier::Arbitrary { value, .. } => writeln!(
                context.buffer,
                "font-family: {};",
                to_css_value(value)
                    .split(',')
                    .map(|v| if v.trim().contains(' ') {
                        Cow::from(format!(r#""{}""#, v))
                    } else {
                        Cow::from(v)
                    })
                    .collect::<Vec<Cow<str>>>()
                    .join(","),
            )?,
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct FontSizePlugin;

impl Plugin for FontSizePlugin {
    fn namespace(&self) -> &str {
        "text"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => [
                "xs", "sm", "base", "lg", "xl", "2xl", "3xl", "4xl", "5xl", "6xl", "7xl", "8xl",
                "9xl",
            ]
            .contains(&&**value),
            Modifier::Arbitrary { hint, value, .. } => {
                *hint == "length"
                    || (hint.is_empty()
                        && (is_matching_length(value)
                            || is_matching_absolute_size(value)
                            || is_matching_relative_size(value)))
            }
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "xs" => {
                    writeln!(context.buffer, "font-size: 0.75rem;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "line-height: 1rem;")?;
                }
                "sm" => {
                    writeln!(context.buffer, "font-size: 0.875rem;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "line-height: 1.25rem;")?;
                }
                "base" => {
                    writeln!(context.buffer, "font-size: 1rem;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "line-height: 1.5rem;")?;
                }
                "lg" => {
                    writeln!(context.buffer, "font-size: 1.125rem;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "line-height: 1.75rem;")?;
                }
                "xl" => {
                    writeln!(context.buffer, "font-size: 1.25rem;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "line-height: 1.75rem;")?;
                }
                "2xl" => {
                    writeln!(context.buffer, "font-size: 1.5rem;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "line-height: 2rem;")?;
                }
                "3xl" => {
                    writeln!(context.buffer, "font-size: 1.875rem;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "line-height: 2.25rem;")?;
                }
                "4xl" => {
                    writeln!(context.buffer, "font-size: 2.25rem;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "line-height: 2.5rem;")?;
                }
                "5xl" => {
                    writeln!(context.buffer, "font-size: 3rem;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "line-height: 1;")?;
                }
                "6xl" => {
                    writeln!(context.buffer, "font-size: 3.75rem;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "line-height: 1;")?;
                }
                "7xl" => {
                    writeln!(context.buffer, "font-size: 4.5rem;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "line-height: 1;")?;
                }
                "8xl" => {
                    writeln!(context.buffer, "font-size: 6rem;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "line-height: 1;")?;
                }
                "9xl" => {
                    writeln!(context.buffer, "font-size: 8rem;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "line-height: 1;")?;
                }
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "font-size: {};", to_css_value(value))?
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct FontWeightPlugin;

impl Plugin for FontWeightPlugin {
    fn namespace(&self) -> &str {
        "font"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
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

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "thin" => writeln!(context.buffer, "font-weight: 100;")?,
                "extralight" => writeln!(context.buffer, "font-weight: 200;")?,
                "light" => writeln!(context.buffer, "font-weight: 300;")?,
                "normal" => writeln!(context.buffer, "font-weight: 400;")?,
                "medium" => writeln!(context.buffer, "font-weight: 500;")?,
                "semibold" => writeln!(context.buffer, "font-weight: 600;")?,
                "bold" => writeln!(context.buffer, "font-weight: 700;")?,
                "extrabold" => writeln!(context.buffer, "font-weight: 800;")?,
                "black" => writeln!(context.buffer, "font-weight: 900;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "font-weight: {};", to_css_value(value))?
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct TextAlignmentPlugin;

impl Plugin for TextAlignmentPlugin {
    fn namespace(&self) -> &str {
        "text"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => {
                ["left", "center", "right", "justify"].contains(&&**value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "left" => writeln!(context.buffer, "text-align: left;")?,
                "center" => writeln!(context.buffer, "text-align: center;")?,
                "right" => writeln!(context.buffer, "text-align: right;")?,
                "justify" => writeln!(context.buffer, "text-align: justify;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct TextTransformPlugin;

impl Plugin for TextTransformPlugin {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => {
                ["uppercase", "lowercase", "capitalize", "normal-case"].contains(&&**value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "uppercase" => writeln!(context.buffer, "text-transform: uppercase;")?,
                "lowercase" => writeln!(context.buffer, "text-transform: lowercase;")?,
                "capitalize" => writeln!(context.buffer, "text-transform: capitalize;")?,
                "normal-case" => writeln!(context.buffer, "text-transform: none;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct TrackingPlugin;

impl Plugin for TrackingPlugin {
    fn namespace(&self) -> &str {
        "tracking"
    }
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => {
                ["tighter", "tight", "normal", "wide", "wider", "widest"].contains(&&**value)
            }
            Modifier::Arbitrary { value, .. } => *value == "normal" || is_matching_length(value),
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "tighter" => writeln!(context.buffer, "letter-spacing: -0.05em;")?,
                "tight" => writeln!(context.buffer, "letter-spacing: -0.025em;")?,
                "normal" => writeln!(context.buffer, "letter-spacing: 0;")?,
                "wide" => writeln!(context.buffer, "letter-spacing: 0.025em;")?,
                "wider" => writeln!(context.buffer, "letter-spacing: 0.05em;")?,
                "widest" => writeln!(context.buffer, "letter-spacing: 0.1em;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "letter-spacing: {};", to_css_value(value))?
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct LeadingPlugin;

impl Plugin for LeadingPlugin {
    fn namespace(&self) -> &str {
        "leading"
    }
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => [
                "none", "tight", "snug", "relaxed", "loose", "3", "4", "5", "6", "7", "8", "9",
                "10",
            ]
            .contains(&&**value),
            // https://developer.mozilla.org/en-US/docs/Web/CSS/line-height#values
            Modifier::Arbitrary { value, .. } => {
                *value == "normal"
                    || is_matching_float(value)
                    || is_matching_length(value)
                    || is_matching_percentage(value)
            }
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "none" => writeln!(context.buffer, "line-height: 1;")?,
                "tight" => writeln!(context.buffer, "line-height: 1.25;")?,
                "snug" => writeln!(context.buffer, "line-height: 1.375;")?,
                "normal" => writeln!(context.buffer, "line-height: 1.5;")?,
                "relaxed" => writeln!(context.buffer, "line-height: 1.625;")?,
                "loose" => writeln!(context.buffer, "line-height: 2;")?,
                "3" => writeln!(context.buffer, "line-height: .75rem;")?,
                "4" => writeln!(context.buffer, "line-height: 1rem;")?,
                "5" => writeln!(context.buffer, "line-height: 1.25rem;")?,
                "6" => writeln!(context.buffer, "line-height: 1.5rem;")?,
                "7" => writeln!(context.buffer, "line-height: 1.75rem;")?,
                "8" => writeln!(context.buffer, "line-height: 2rem;")?,
                "9" => writeln!(context.buffer, "line-height: 2.25rem;")?,
                "10" => writeln!(context.buffer, "line-height: 2.5rem;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "line-height: {};", to_css_value(value))?
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct ItalicPlugin;

impl Plugin for ItalicPlugin {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => ["italic", "not-italic"].contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "italic" => writeln!(context.buffer, "font-style: italic;")?,
                "not-italic" => writeln!(context.buffer, "font-style: normal;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct TextDecorationPlugin;

impl Plugin for TextDecorationPlugin {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => {
                ["underline", "overline", "line-through", "no-underline"].contains(value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => {
                writeln!(context.buffer, "-webkit-text-decoration-line: {value};")?;

                indent(context.indentation, context.buffer)?;
                writeln!(context.buffer, "text-decoration-line: {value};")?;
            }
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct TextDecorationColorPlugin;

impl Plugin for TextDecorationColorPlugin {
    fn namespace(&self) -> &str {
        "decoration"
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

        writeln!(context.buffer, "-webkit-text-decoration-color: {value};")?;
        indent(context.indentation, context.buffer)?;
        writeln!(context.buffer, "text-decoration-color: {value};")
    }
}

#[derive(Debug)]
pub struct TextDecorationStylePlugin;

impl Plugin for TextDecorationStylePlugin {
    fn namespace(&self) -> &str {
        "decoration"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => {
                ["solid", "double", "dotted", "dashed", "wavy"].contains(&&**value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => {
                writeln!(context.buffer, "text-decoration-style: {value};")?
            }
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct TextDecorationThicknessPlugin;

impl Plugin for TextDecorationThicknessPlugin {
    fn namespace(&self) -> &str {
        "decoration"
    }
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => {
                ["auto", "from-font"].contains(&&**value) || value.parse::<usize>().is_ok()
            }
            Modifier::Arbitrary { hint, value, .. } => {
                *hint == "length"
                    || (hint.is_empty()
                        && (["auto", "from-font"].contains(&&**value)
                            || is_matching_length(value)
                            || is_matching_percentage(value)))
            }
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => {
                if ["auto", "from-font"].contains(&&**value) {
                    return writeln!(context.buffer, "text-decoration-thickness: {value};");
                }

                // NOTE: Not-compatible with TailwindCSS, support all values
                writeln!(context.buffer, "text-decoration-thickness: {value}px;")?;
            }
            Modifier::Arbitrary { value, .. } => writeln!(
                context.buffer,
                "text-decoration-thickness: {};",
                to_css_value(value)
            )?,
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct TextDecorationUnderlineOffsetPlugin;

impl Plugin for TextDecorationUnderlineOffsetPlugin {
    fn namespace(&self) -> &str {
        "underline-offset"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => value.parse::<usize>().is_ok() || *value == "auto",
            Modifier::Arbitrary { value, .. } => *value == "auto" || is_matching_length(value),
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => {
                if *value == "auto" {
                    return writeln!(context.buffer, "text-underline-offset: auto;");
                }

                // NOTE: Not-compatible with TailwindCSS, support all values
                writeln!(context.buffer, "text-underline-offset: {value}px;")?;
            }
            Modifier::Arbitrary { value, .. } => writeln!(
                context.buffer,
                "text-underline-offset: {};",
                to_css_value(value)
            )?,
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct ContentPlugin;

impl Plugin for ContentPlugin {
    fn namespace(&self) -> &str {
        "content"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => *value == "none",
            Modifier::Arbitrary { value, .. } => is_matching_all(value),
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { .. } => {
                writeln!(context.buffer, "--en-content: none;")?;
                indent(context.indentation, context.buffer)?;
                writeln!(context.buffer, "content: var(--en-content);")?;
            }
            Modifier::Arbitrary { value, .. } => {
                // NOTE: Not-compatible with TailwindCSS, it is not needed to add quotes to `content`
                // containing spaces, they are added later
                writeln!(context.buffer, "--en-content: \"{}\";", to_css_value(value))?;
                indent(context.indentation, context.buffer)?;
                writeln!(context.buffer, "content: var(--en-content);")?;
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct FontVariantNumericPlugin;

impl Plugin for FontVariantNumericPlugin {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
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

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "normal-nums" => return writeln!(context.buffer, "font-variant-numeric: normal;"),
                "ordinal" => writeln!(context.buffer, "--en-ordinal: ordinal;")?,
                "slashed-zero" => writeln!(context.buffer, "--en-slashed-zero: slashed-zero;")?,
                "lining-nums" => writeln!(context.buffer, "--en-numeric-figure: lining-nums;")?,
                "oldstyle-nums" => writeln!(context.buffer, "--en-numeric-figure: oldstyle-nums;")?,
                "proportional-nums" => {
                    writeln!(context.buffer, "--en-numeric-spacing: proportional-nums;")?
                }
                "tabular-nums" => writeln!(context.buffer, "--en-numeric-spacing: tabular-nums;")?,
                "diagonal-fractions" => {
                    writeln!(context.buffer, "--en-numeric-fraction: diagonal-fractions;")?
                }
                "stacked-fractions" => {
                    writeln!(context.buffer, "--en-numeric-fraction: stacked-fractions;")?
                }
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        indent(context.indentation, context.buffer)?;
        writeln!(context.buffer, "{}", CSS_FONT_VARIANT_NUMERIC)?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct FontSmoothingPlugin;

impl Plugin for FontSmoothingPlugin {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => {
                ["antialised", "subpixel-antialised"].contains(&&**value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "antialised" => {
                    writeln!(context.buffer, "-webkit-font-smoothing: antialiased;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "-moz-osx-font-smoothing: grayscale;")?;
                }
                "subpixel-antialised" => {
                    writeln!(context.buffer, "-webkit-font-smoothing: auto;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "-moz-osx-font-smoothing: auto;")?;
                }
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct ListStyleTypePlugin;

impl Plugin for ListStyleTypePlugin {
    fn namespace(&self) -> &str {
        "list"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => ["disc", "decimal", "none"].contains(&&**value),
            Modifier::Arbitrary { value, .. } => is_matching_all(value), // TODO: Better matching
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => writeln!(context.buffer, "list-style-type: {value};")?,
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "list-style-type: {};", to_css_value(value))?
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct ListStylePositionPlugin;

impl Plugin for ListStylePositionPlugin {
    fn namespace(&self) -> &str {
        "list"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => ["inside", "outside"].contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => {
                writeln!(context.buffer, "list-style-position: {value};")?
            }
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct TextIndentPlugin;

impl Plugin for TextIndentPlugin {
    fn namespace(&self) -> &str {
        "indent"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { is_negative, value } => {
                length::get_basic(value, *is_negative).is_some()
            }
            Modifier::Arbitrary { value, .. } => is_matching_length(value),
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { is_negative, value } => {
                writeln!(
                    context.buffer,
                    "text-indent: {};",
                    length::get_basic(value, *is_negative).unwrap(),
                )?;
            }
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "text-indent: {};", to_css_value(value))?;
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct VerticalAlignPlugin;

impl Plugin for VerticalAlignPlugin {
    fn namespace(&self) -> &str {
        "align"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
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

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => writeln!(context.buffer, "vertical-align: {value};")?,
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct TextOverflowPlugin;

impl Plugin for TextOverflowPlugin {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => {
                ["truncate", "text-ellipsis", "text-clip"].contains(&&**value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "truncate" => {
                    writeln!(context.buffer, "overflow: hidden;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "text-overflow: ellipsis;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "white-space: nowrap;")?
                }
                "text-ellipsis" => writeln!(context.buffer, "text-overflow: ellipsis;")?,
                "text-clip" => writeln!(context.buffer, "text-overflow: clip;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct WhitespacePlugin;

impl Plugin for WhitespacePlugin {
    fn namespace(&self) -> &str {
        "whitespace"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => {
                ["normal", "nowrap", "pre", "pre-line", "pre-wrap"].contains(&&**value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => writeln!(context.buffer, "white-space: {value};")?,
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct WordBreakPlugin;

impl Plugin for WordBreakPlugin {
    fn namespace(&self) -> &str {
        "break"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => ["normal", "words", "all"].contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "normal" => {
                    writeln!(context.buffer, "overflow-wrap: normal;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "word-break: normal;")?;
                }
                "words" => writeln!(context.buffer, "overflow-wrap: break-word;")?,
                "all" => writeln!(context.buffer, "word-break: break-all;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
