#![doc = include_str!("README.md")]
use crate::{
    plugins::{to_css_value, Plugin},
    context::{ContextCanHandle, ContextHandle},
    selector::Modifier,
    utils::{indent, value_matchers::is_matching_all},
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
            Modifier::Arbitrary { value, .. } => is_matching_all(value), // TODO: Better matching
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                if *value == "auto" {
                    return writeln!(context.buffer, "grid-row: auto;");
                }

                if let Some(value) = value.strip_prefix("span-") {
                    if value == "full" {
                        return writeln!(context.buffer, "grid-row: 1 / -1;");
                    }
                    writeln!(context.buffer, "grid-row: span {value} / span {value};")?;
                } else if let Some(value) = value.strip_prefix("start-") {
                    if value == "auto" {
                        return writeln!(context.buffer, "grid-row-start: auto;");
                    }
                    writeln!(context.buffer, "grid-row-start: {value};")?;
                } else if let Some(value) = value.strip_prefix("end-") {
                    if value == "auto" {
                        return writeln!(context.buffer, "grid-row-end: auto;");
                    }
                    writeln!(context.buffer, "grid-row-end: {value};")?;
                }
            }
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "grid-row: {};", to_css_value(value))?;
            }
        }

        Ok(())
    }
}
