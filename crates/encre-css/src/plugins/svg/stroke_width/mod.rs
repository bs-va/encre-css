#![doc = include_str!("README.md")]
use crate::{
    context::{ContextCanHandle, ContextHandle},
    plugins::{to_css_value, Plugin},
    selector::Modifier,
    utils::{
        indent,
        value_matchers::{is_matching_length, is_matching_percentage},
    },
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "stroke"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => value.parse::<usize>().is_ok(),
            Modifier::Arbitrary { hint, value, .. } => {
                *hint == "length"
                    || *hint == "number"
                    || *hint == "percentage"
                    || (hint.is_empty()
                        && (is_matching_length(value) || is_matching_percentage(value)))
            }
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                writeln!(context.buffer, "stroke-width: {value}px;")?;
            }
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "stroke-width: {};", to_css_value(value))?;
            }
        }

        Ok(())
    }
}
