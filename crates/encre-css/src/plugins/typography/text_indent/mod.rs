#![doc = include_str!("README.md")]
use crate::{
    plugins::{to_css_value, Plugin},
    context::{ContextCanHandle, ContextHandle},
    selector::Modifier,
    utils::{indent, spacing, value_matchers::is_matching_length},
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "indent"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => spacing::is_matching_builtin_spacing(value),
            Modifier::Arbitrary { value, .. } => is_matching_length(value),
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { is_negative, value } => {
                writeln!(
                    context.buffer,
                    "text-indent: {};",
                    spacing::get(value, *is_negative).unwrap(),
                )?;
            }
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "text-indent: {};", to_css_value(value))?;
            }
        }

        Ok(())
    }
}
