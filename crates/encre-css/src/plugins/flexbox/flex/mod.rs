#![doc = include_str!("README.md")]
use crate::{
    context::{ContextCanHandle, ContextHandle},
    plugins::{to_css_value, Plugin},
    selector::Modifier,
    utils::{indent, value_matchers::is_matching_all},
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "flex"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => ["1", "auto", "initial", "none"].contains(&*value),
            Modifier::Arbitrary { value, .. } => is_matching_all(value), // TODO: Better matching
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "1" => writeln!(context.buffer, "flex: 1 1 0%;")?,
                "auto" => writeln!(context.buffer, "flex: 1 1 auto;")?,
                "initial" => writeln!(context.buffer, "flex: 0 1 auto;")?,
                "none" => writeln!(context.buffer, "flex: none;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "flex: {};", to_css_value(value))?;
            }
        }

        Ok(())
    }
}
