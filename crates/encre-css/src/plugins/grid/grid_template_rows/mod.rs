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
        "grid-rows"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => value.parse::<usize>().is_ok() || *value == "none",
            Modifier::Arbitrary { value, .. } => is_matching_all(value), // TODO: Better matching
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                if *value == "none" {
                    return writeln!(context.buffer, "grid-template-rows: none;");
                }

                writeln!(
                    context.buffer,
                    "grid-template-rows: repeat({}, minmax(0, 1fr));",
                    value.parse::<usize>().unwrap(),
                )?;
            }
            Modifier::Arbitrary { value, .. } => writeln!(
                context.buffer,
                "grid-template-rows: {};",
                to_css_value(value)
            )?,
        }

        Ok(())
    }
}
