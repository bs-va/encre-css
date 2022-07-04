#![doc = include_str!("README.md")]
use crate::{
    context::{ContextCanHandle, ContextHandle},
    plugins::{to_css_value, Plugin},
    selector::Modifier,
    utils::{indent, value_matchers::is_matching_position},
};
use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "bg"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => [
                "bottom",
                "center",
                "left",
                "left-bottom",
                "left-top",
                "right",
                "right-bottom",
                "right-top",
                "top",
            ]
            .contains(value),
            Modifier::Arbitrary { hint, value, .. } => {
                // https://developer.mozilla.org/en-US/docs/Web/CSS/background-position
                *hint == "position"
                    || (hint.is_empty() && value.split(',').all(is_matching_position))
            }
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "bottom" => writeln!(context.buffer, "background-position: bottom;")?,
                "center" => writeln!(context.buffer, "background-position: center;")?,
                "left" => writeln!(context.buffer, "background-position: left;")?,
                "left-bottom" => writeln!(context.buffer, "background-position: left-bottom;")?,
                "left-top" => writeln!(context.buffer, "background-position: left-top;")?,
                "right" => writeln!(context.buffer, "background-position: right;")?,
                "right-bottom" => writeln!(context.buffer, "background-position: right-bottom;")?,
                "right-top" => writeln!(context.buffer, "background-position: right-top;")?,
                "top" => writeln!(context.buffer, "background-position: top;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => writeln!(
                context.buffer,
                "background-position: {};",
                to_css_value(value)
            )?,
        }

        Ok(())
    }
}
