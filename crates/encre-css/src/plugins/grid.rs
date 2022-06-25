use super::{to_css_value, Plugin};
use crate::utils::{indent, length, value_matchers::*};
use crate::{config::Config, selector::Modifier};

use std::fmt::{self, Write};

#[derive(Debug)]
pub struct TemplateColumnsPlugin;

impl Plugin for TemplateColumnsPlugin {
    fn namespace(&self) -> &str {
        "grid-cols"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => value.parse::<usize>().is_ok() || *value == "none",
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
            Modifier::Basic { value, .. } => {
                if *value == "none" {
                    return writeln!(buffer, "grid-template-columns: none;");
                }

                // NOTE: Not-compatible with TailwindCSS, support all values
                writeln!(
                    buffer,
                    "grid-template-columns: repeat({}, minmax(0, 1fr));",
                    value.parse::<usize>().unwrap(),
                )?;
            }
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "grid-template-columns: {};", to_css_value(value))?
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct TemplateRowsPlugin;

impl Plugin for TemplateRowsPlugin {
    fn namespace(&self) -> &str {
        "grid-rows"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => value.parse::<usize>().is_ok() || *value == "none",
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
            Modifier::Basic { value, .. } => {
                if *value == "none" {
                    return writeln!(buffer, "grid-template-rows: none;");
                }

                // NOTE: Not-compatible with TailwindCSS, support all values
                writeln!(
                    buffer,
                    "grid-template-rows: repeat({}, minmax(0, 1fr));",
                    value.parse::<usize>().unwrap(),
                )?;
            }
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "grid-template-rows: {};", to_css_value(value))?
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct StartEndSpanColumnPlugin;

impl Plugin for StartEndSpanColumnPlugin {
    fn namespace(&self) -> &str {
        "col"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => {
                *value == "auto"
                    || value
                        .strip_prefix("span-")
                        .map(|v| v == "full" || v.parse::<usize>().is_ok())
                        .unwrap_or(false)
                    || value
                        .strip_prefix("start-")
                        .map(|v| v == "auto" || v.parse::<usize>().is_ok())
                        .unwrap_or(false)
                    || value
                        .strip_prefix("end-")
                        .map(|v| v == "auto" || v.parse::<usize>().is_ok())
                        .unwrap_or(false)
            }
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
            Modifier::Basic { value, .. } => {
                if *value == "auto" {
                    return writeln!(buffer, "grid-column: auto;");
                }

                if let Some(value) = value.strip_prefix("span-") {
                    if value == "full" {
                        return writeln!(buffer, "grid-column: 1 / -1;");
                    }

                    // NOTE: Not-compatible with TailwindCSS, support all values
                    writeln!(buffer, "grid-column: span {value} / span {value};")?;
                } else if let Some(value) = value.strip_prefix("start-") {
                    if value == "auto" {
                        return writeln!(buffer, "grid-column-start: auto;");
                    }

                    // NOTE: Not-compatible with TailwindCSS, support all values
                    writeln!(buffer, "grid-column-start: {value};")?;
                } else if let Some(value) = value.strip_prefix("end-") {
                    if value == "auto" {
                        return writeln!(buffer, "grid-column-end: auto;");
                    }

                    // NOTE: Not-compatible with TailwindCSS, support all values
                    writeln!(buffer, "grid-column-end: {value};")?;
                }
            }
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "grid-column: {};", to_css_value(value))?
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct StartEndSpanRowPlugin;

impl Plugin for StartEndSpanRowPlugin {
    fn namespace(&self) -> &str {
        "row"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => {
                *value == "auto"
                    || value
                        .strip_prefix("span-")
                        .map(|v| v == "full" || v.parse::<usize>().is_ok())
                        .unwrap_or(false)
                    || value
                        .strip_prefix("start-")
                        .map(|v| v == "auto" || v.parse::<usize>().is_ok())
                        .unwrap_or(false)
                    || value
                        .strip_prefix("end-")
                        .map(|v| v == "auto" || v.parse::<usize>().is_ok())
                        .unwrap_or(false)
            }
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
            Modifier::Basic { value, .. } => {
                if *value == "auto" {
                    return writeln!(buffer, "grid-row: auto;");
                }

                if let Some(value) = value.strip_prefix("span-") {
                    if value == "full" {
                        return writeln!(buffer, "grid-row: 1 / -1;");
                    }

                    // NOTE: Not-compatible with TailwindCSS, support all values
                    writeln!(buffer, "grid-row: span {value} / span {value};")?;
                } else if let Some(value) = value.strip_prefix("start-") {
                    if value == "auto" {
                        return writeln!(buffer, "grid-row-start: auto;");
                    }

                    // NOTE: Not-compatible with TailwindCSS, support all values
                    writeln!(buffer, "grid-row-start: {value};")?;
                } else if let Some(value) = value.strip_prefix("end-") {
                    if value == "auto" {
                        return writeln!(buffer, "grid-row-end: auto;");
                    }

                    // NOTE: Not-compatible with TailwindCSS, support all values
                    writeln!(buffer, "grid-row-end: {value};")?;
                }
            }
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "grid-row: {};", to_css_value(value))?
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct AutoFlowPlugin;

impl Plugin for AutoFlowPlugin {
    fn namespace(&self) -> &str {
        "grid-flow"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => {
                ["row", "col", "row-dense", "col-dense"].contains(&&**value)
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
                "row" => writeln!(buffer, "grid-auto-flow: row;")?,
                "col" => writeln!(buffer, "grid-auto-flow: column;")?,
                "row-dense" => writeln!(buffer, "grid-auto-flow: row dense;")?,
                "col-dense" => writeln!(buffer, "grid-auto-flow: column dense;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct AutoColumnsPlugin;

impl Plugin for AutoColumnsPlugin {
    fn namespace(&self) -> &str {
        "auto-cols"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => ["auto", "min", "max", "fr"].contains(&&**value),
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
            Modifier::Basic { value, .. } => match *value {
                "auto" => writeln!(buffer, "grid-auto-columns: auto;")?,
                "min" => writeln!(buffer, "grid-auto-columns: min-content;")?,
                "max" => writeln!(buffer, "grid-auto-columns: max-content;")?,
                "fr" => writeln!(buffer, "grid-auto-columns: minmax(0, 1fr);")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "grid-auto-columns: {};", to_css_value(value))?
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct AutoRowsPlugin;

impl Plugin for AutoRowsPlugin {
    fn namespace(&self) -> &str {
        "auto-rows"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => ["auto", "min", "max", "fr"].contains(&&**value),
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
            Modifier::Basic { value, .. } => match *value {
                "auto" => writeln!(buffer, "grid-auto-rows: auto;")?,
                "min" => writeln!(buffer, "grid-auto-rows: min-content;")?,
                "max" => writeln!(buffer, "grid-auto-rows: max-content;")?,
                "fr" => writeln!(buffer, "grid-auto-rows: minmax(0, 1fr);")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "grid-auto-rows: {};", to_css_value(value))?
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct GapPlugin;

impl Plugin for GapPlugin {
    fn namespace(&self) -> &str {
        "gap"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { is_negative, value } => {
                length::get_basic(value, *is_negative).is_some()
            }
            Modifier::Arbitrary { prefix, value, .. } => {
                prefix.is_empty() && is_matching_length(value)
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
            Modifier::Basic { is_negative, value } => writeln!(
                buffer,
                "gap: {};",
                length::get_basic(value, *is_negative).unwrap()
            )?,
            Modifier::Arbitrary { value, .. } => writeln!(buffer, "gap: {};", to_css_value(value))?,
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct GapXPlugin;

impl Plugin for GapXPlugin {
    fn namespace(&self) -> &str {
        "gap-x"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { is_negative, value } => {
                length::get_basic(value, *is_negative).is_some()
            }
            Modifier::Arbitrary { value, .. } => is_matching_length(value),
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
            Modifier::Basic { is_negative, value } => writeln!(
                buffer,
                "column-gap: {};",
                length::get_basic(value, *is_negative).unwrap()
            )?,
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "column-gap: {};", to_css_value(value))?
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct GapYPlugin;

impl Plugin for GapYPlugin {
    fn namespace(&self) -> &str {
        "gap-y"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { is_negative, value } => {
                length::get_basic(value, *is_negative).is_some()
            }
            Modifier::Arbitrary { value, .. } => is_matching_length(value),
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
            Modifier::Basic { is_negative, value } => writeln!(
                buffer,
                "row-gap: {};",
                length::get_basic(value, *is_negative).unwrap()
            )?,
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "row-gap: {};", to_css_value(value))?
            }
        }

        Ok(())
    }
}
