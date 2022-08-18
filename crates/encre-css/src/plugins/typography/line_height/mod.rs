#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::{
    generator::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
    utils::{
        indent, spacing,
        value_matchers::{is_matching_length, is_matching_number, is_matching_percentage},
    },
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "leading"
    }
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                ["none", "tight", "snug", "normal", "relaxed", "loose"].contains(&&**value)
                    || spacing::is_matching_builtin_spacing(value)
            }
            // https://developer.mozilla.org/en-US/docs/Web/CSS/line-height#values
            Modifier::Arbitrary { value, .. } => {
                *value == "normal"
                    || is_matching_number(value)
                    || is_matching_length(value)
                    || is_matching_percentage(value)
            }
        }
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { is_negative, value } => match *value {
                "none" => writeln!(context.buffer, "line-height: 1;")?,
                "tight" => writeln!(context.buffer, "line-height: 1.25;")?,
                "snug" => writeln!(context.buffer, "line-height: 1.375;")?,
                "normal" => writeln!(context.buffer, "line-height: 1.5;")?,
                "relaxed" => writeln!(context.buffer, "line-height: 1.625;")?,
                "loose" => writeln!(context.buffer, "line-height: 2;")?,
                _ => writeln!(
                    context.buffer,
                    "line-height: {};",
                    spacing::get(value, *is_negative).unwrap()
                )?,
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "line-height: {value};")?;
            }
        }

        Ok(())
    }
}
