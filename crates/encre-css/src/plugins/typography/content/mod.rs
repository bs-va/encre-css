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
        "content"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => *value == "none",
            Modifier::Arbitrary { value, .. } => is_matching_all(value),
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { .. } => writeln!(context.buffer, "--en-content: none;")?,
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "--en-content: \"{}\";", to_css_value(value))?;
            }
        }

        indent(context.indentation, context.buffer)?;
        writeln!(context.buffer, "content: var(--en-content);")?;

        Ok(())
    }
}
