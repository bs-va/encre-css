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
        "decoration"
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
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                let value = color::get(context.config, value, None).unwrap();
                writeln!(context.buffer, "-webkit-text-decoration-color: {value};")?;
                indent(context.indentation, context.buffer)?;
                writeln!(context.buffer, "text-decoration-color: {value};")
            }
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "-webkit-text-decoration-color: {value};")?;
                indent(context.indentation, context.buffer)?;
                writeln!(context.buffer, "text-decoration-color: {value};")
            }
        }
    }
}
