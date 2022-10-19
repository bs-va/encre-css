#![doc = include_str!("README.md")]
#![doc(alias = "grid")]
use crate::prelude::build_plugin::*;

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

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                if *value == "auto" {
                    return context.buffer.line("grid-row: auto;");
                }

                if let Some(value) = value.strip_prefix("span-") {
                    if value == "full" {
                        return context.buffer.line("grid-row: 1 / -1;");
                    }
                    context
                        .buffer
                        .line(format_args!("grid-row: span {value} / span {value};"));
                } else if let Some(value) = value.strip_prefix("start-") {
                    if value == "auto" {
                        return context.buffer.line("grid-row-start: auto;");
                    }
                    context
                        .buffer
                        .line(format_args!("grid-row-start: {value};"));
                } else if let Some(value) = value.strip_prefix("end-") {
                    if value == "auto" {
                        return context.buffer.line("grid-row-end: auto;");
                    }
                    context.buffer.line(format_args!("grid-row-end: {value};"));
                }
            }
            Modifier::Arbitrary { value, .. } => {
                context.buffer.line(format_args!("grid-row: {value};"));
            }
        }
    }
}
