use super::Plugin;
use crate::utils::{indent, value_matchers::*};
use crate::{config::Config, selector::Modifier};

use std::fmt::{self, Write};

pub struct OrderPlugin;

impl Plugin for OrderPlugin {
    fn namespace(&self) -> &str {
        "order"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => ["first", "last", "none"].contains(&&**value),
            Modifier::Arbitrary { value, .. } => is_matching_number(value),
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
                "first" => return writeln!(buffer, "order: -9999;"),
                "last" => return writeln!(buffer, "order: 9999;"),
                "none" => return writeln!(buffer, "order: 0;"),
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => writeln!(buffer, "order: {value};")?,
        }

        Ok(())
    }
}

pub struct DirectionPlugin;

impl Plugin for DirectionPlugin {
    fn namespace(&self) -> &str {
        "flex"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => {
                ["row", "row-reverse", "col", "col-reverse"].contains(&&**value)
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
                "row" => writeln!(buffer, "flex-direction: row;")?,
                "row-reverse" => writeln!(buffer, "flex-direction: row-reverse;")?,
                "col" => writeln!(buffer, "flex-direction: column;")?,
                "col-reverse" => writeln!(buffer, "flex-direction: column-reverse;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

pub struct WrapPlugin;

impl Plugin for WrapPlugin {
    fn namespace(&self) -> &str {
        "flex"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => ["nowrap", "wrap", "wrap-reverse"].contains(&&**value),
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
                "nowrap" => writeln!(buffer, "flex-wrap: nowrap;")?,
                "wrap" => writeln!(buffer, "flex-wrap: wrap;")?,
                "wrap-reverse" => writeln!(buffer, "flex-wrap: wrap-reverse;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

pub struct GrowShrinkBasisPlugin;

impl Plugin for GrowShrinkBasisPlugin {
    fn namespace(&self) -> &str {
        "flex"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => [
                "1", "auto", "initial", "grow", "grow-0", "shrink", "shrink-0", "none",
            ]
            .contains(&&**value),
            Modifier::Arbitrary { hint, value } => {
                if hint == "list" {
                    return true;
                }

                let mut split = value.split('_');
                let mut is_matching = (false, false, false);

                // flex-grow
                // https://developer.mozilla.org/en-US/docs/Web/CSS/flex-grow#values
                if let Some(value) = split.next() {
                    if is_matching_number(value) {
                        is_matching.0 = true;
                    }
                }

                // flex-shrink
                // https://developer.mozilla.org/en-US/docs/Web/CSS/flex-shrink#values
                if let Some(value) = split.next() {
                    if is_matching_number(value) {
                        is_matching.1 = true;
                    }
                }

                // flex-basis
                // https://developer.mozilla.org/en-US/docs/Web/CSS/flex-basis#values
                if let Some(value) = split.next() {
                    if is_matching_length(value) || is_matching_percentage(value) || value == "auto"
                    {
                        is_matching.2 = true;
                    }
                }

                is_matching.0 && is_matching.1 && is_matching.2
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
                "1" => writeln!(buffer, "flex: 1 1 0%;")?,
                "auto" => writeln!(buffer, "flex: 1 1 auto;")?,
                "initial" => writeln!(buffer, "flex: 0 1 auto;")?,
                "none" => writeln!(buffer, "flex: none;")?,
                "grow" => writeln!(buffer, "flex-grow: 1;")?,
                "grow-0" => writeln!(buffer, "flex-grow: 0;")?,
                "shrink" => writeln!(buffer, "flex-shrink: 1;")?,
                "shrink-0" => writeln!(buffer, "flex-shrink: 0;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
