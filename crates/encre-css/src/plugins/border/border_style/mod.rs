#![doc = include_str!("README.md")]
use crate::{
    context::{ContextCanHandle, ContextHandle},
    plugins::{to_css_value, Plugin},
    selector::Modifier,
    utils::{indent, value_matchers::is_matching_line_style},
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "border"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => is_matching_line_style(value),
            Modifier::Arbitrary { value, .. } => value.split('_').all(is_matching_line_style),
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;

        let (Modifier::Builtin { value, .. } | Modifier::Arbitrary { value, .. }) =
            context.modifier;
        writeln!(context.buffer, "border-style: {};", to_css_value(value))?;

        Ok(())
    }
}
