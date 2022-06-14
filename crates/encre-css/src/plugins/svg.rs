use super::Plugin;
use crate::utils::{default_colors, indent, value_matchers::*};
use crate::{config::Config, selector::Modifier};

use std::fmt::{self, Write};

pub struct FillPlugin;

impl Plugin for FillPlugin {
    fn namespace(&self) -> &str {
        "fill"
    }

    fn can_handle(&self, config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => default_colors::get(config, value).is_some(),
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
                let color = default_colors::get(config, value).unwrap();
                if color.contains("--en-opacity") {
                    writeln!(
                        buffer,
                        "fill: {};",
                        color.replace(" / var(--en-opacity)", "")
                    )?;
                } else {
                    writeln!(buffer, "fill: {color};")?;
                }
            }
            Modifier::Arbitrary { value, .. } => {
                if value.contains("--en-opacity") {
                    writeln!(
                        buffer,
                        "fill: {};",
                        value.replace(" / var(--en-opacity)", "")
                    )?;
                } else {
                    writeln!(buffer, "fill: {value};")?;
                }
            }
        }

        Ok(())
    }
}

pub struct StrokeColorPlugin;

impl Plugin for StrokeColorPlugin {
    fn namespace(&self) -> &str {
        "stroke"
    }

    fn can_handle(&self, config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => default_colors::get(config, value).is_some(),
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
                let color = default_colors::get(config, value).unwrap();
                if color.contains("--en-opacity") {
                    writeln!(
                        buffer,
                        "stroke: {};",
                        color.replace(" / var(--en-opacity)", "")
                    )?;
                } else {
                    writeln!(buffer, "stroke: {color};")?;
                }
            }
            Modifier::Arbitrary { value, .. } => {
                if value.contains("--en-opacity") {
                    writeln!(
                        buffer,
                        "stroke: {};",
                        value.replace(" / var(--en-opacity)", "")
                    )?;
                } else {
                    writeln!(buffer, "stroke: {value};")?;
                }
            }
        }

        Ok(())
    }
}

pub struct StrokeWidthPlugin;

impl Plugin for StrokeWidthPlugin {
    fn namespace(&self) -> &str {
        "stroke"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => value.parse::<usize>().is_ok(),
            Modifier::Arbitrary { hint, value } => {
                hint == "length" || is_matching_length(value) || is_matching_percentage(value)
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
        // NOTE: Not-compatible with TailwindCSS, support all values
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { value, .. } => writeln!(buffer, "stroke-width: {value}px;")?,
            Modifier::Arbitrary { value, .. } => writeln!(buffer, "stroke-width: {value};")?,
        }

        Ok(())
    }
}
