#![doc = include_str!("README.md")]
use crate::{
    generator::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
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

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;

        match context.modifier {
            Modifier::Builtin { value, .. } => writeln!(context.buffer, "border-style: {value};")?,
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "border-style: {value};")?;
            }
        }

        Ok(())
    }
}
