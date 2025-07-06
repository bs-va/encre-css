#![doc = include_str!("README.md")]
#![doc(alias = "grid")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                *value == "auto"
                    || value
                        .strip_prefix("span-")
                        .is_some_and(|v| v == "full" || v.parse::<usize>().is_ok())
                    || value
                        .strip_prefix("start-")
                        .is_some_and(|v| v == "auto" || v.parse::<usize>().is_ok())
                    || value
                        .strip_prefix("end-")
                        .is_some_and(|v| v == "auto" || v.parse::<usize>().is_ok())
            }
            Modifier::Arbitrary { value, .. } => is_matching_all(value),
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                if *value == "auto" {
                    return context.buffer.line("grid-column: auto;");
                }

                if let Some(value) = value.strip_prefix("span-") {
                    if value == "full" {
                        return context.buffer.line("grid-column: 1 / -1;");
                    }
                    context
                        .buffer
                        .line(format_args!("grid-column: span {value} / span {value};"));
                } else if let Some(value) = value.strip_prefix("start-") {
                    if value == "auto" {
                        return context.buffer.line("grid-column-start: auto;");
                    }
                    context
                        .buffer
                        .line(format_args!("grid-column-start: {value};"));
                } else if let Some(value) = value.strip_prefix("end-") {
                    if value == "auto" {
                        return context.buffer.line("grid-column-end: auto;");
                    }
                    context
                        .buffer
                        .line(format_args!("grid-column-end: {value};"));
                }
            }
            Modifier::Arbitrary { value, .. } => {
                context.buffer.line(format_args!("grid-column: {value};"));
            }
        }
    }
}
