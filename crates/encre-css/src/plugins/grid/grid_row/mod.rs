#![doc = include_str!("README.md")]
#![doc(alias = "grid")]
use crate::{
    generator::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
    utils::value_matchers::is_matching_all,
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "row"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                *value == "auto"
                    || value
                        .strip_prefix("span-")
                        .map_or(false, |v| v == "full" || v.parse::<usize>().is_ok())
                    || value
                        .strip_prefix("start-")
                        .map_or(false, |v| v == "auto" || v.parse::<usize>().is_ok())
                    || value
                        .strip_prefix("end-")
                        .map_or(false, |v| v == "auto" || v.parse::<usize>().is_ok())
            }
            Modifier::Arbitrary { value, .. } => is_matching_all(value),
        }
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { value, .. } => {
                if *value == "auto" {
                    return writeln!(buffer, "{indentation}grid-row: auto;");
                }

                if let Some(value) = value.strip_prefix("span-") {
                    if value == "full" {
                        return writeln!(buffer, "{indentation}grid-row: 1 / -1;");
                    }
                    writeln!(buffer, "{indentation}grid-row: span {value} / span {value};")?;
                } else if let Some(value) = value.strip_prefix("start-") {
                    if value == "auto" {
                        return writeln!(buffer, "{indentation}grid-row-start: auto;");
                    }
                    writeln!(buffer, "{indentation}grid-row-start: {value};")?;
                } else if let Some(value) = value.strip_prefix("end-") {
                    if value == "auto" {
                        return writeln!(buffer, "{indentation}grid-row-end: auto;");
                    }
                    writeln!(buffer, "{indentation}grid-row-end: {value};")?;
                }
            }
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "{indentation}grid-row: {value};")?;
            }
        }

        Ok(())
    }
}
