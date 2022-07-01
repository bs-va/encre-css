#![doc = include_str!("README.md")]
use crate::{
    plugins::{to_css_value, Plugin},
    context::{ContextCanHandle, ContextHandle},
    selector::Modifier,
    utils::{indent, value_matchers::is_matching_position},
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "origin"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => [
                "center",
                "top",
                "top-right",
                "right",
                "bottom-right",
                "bottom",
                "bottom-left",
                "left",
                "top-left",
            ]
            .contains(value),
            Modifier::Arbitrary { value, .. } => is_matching_position(value),
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        match context.modifier {
            Modifier::Builtin { value, .. } | Modifier::Arbitrary { value, .. } => {
                indent(context.indentation, context.buffer)?;
                writeln!(context.buffer, "transform-origin: {};", to_css_value(value))?;
            }
        }

        Ok(())
    }
}
