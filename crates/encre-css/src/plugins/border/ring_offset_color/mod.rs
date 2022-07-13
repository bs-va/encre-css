#![doc = include_str!("README.md")]
use crate::{
    generator::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
    utils::{color, indent, value_matchers::is_matching_color},
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "ring-offset"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                color::is_matching_builtin_color(context.config, value)
            }
            Modifier::Arbitrary { hint, value, .. } => {
                *hint == "color" || (hint.is_empty() && is_matching_color(value))
            }
        }
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        writeln!(context.buffer, "--en-ring-offset-shadow: var(--en-ring-inset) 0 0 0 var(--en-ring-offset-width) var(--en-ring-offset-color);")?;

        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { value, .. } => writeln!(
                context.buffer,
                "--en-ring-offset-color: {};",
                color::get(context.config, value, None).unwrap()
            ),
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "--en-ring-offset-color: {value};")
            }
        }
    }
}
