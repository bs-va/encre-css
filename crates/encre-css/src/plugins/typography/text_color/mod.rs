#![doc = include_str!("README.md")]
use crate::{
    context::{ContextCanHandle, ContextHandle},
    plugins::{to_css_value, Plugin},
    selector::Modifier,
    utils::{color, indent, value_matchers::is_matching_color},
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "text"
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

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                let color = color::get(context.config, value, Some("--en-text-opacity")).unwrap();
                if color.contains("--en-text-opacity") {
                    writeln!(context.buffer, "--en-text-opacity: 1;")?;
                    indent(context.indentation, context.buffer)?;
                }

                writeln!(context.buffer, "color: {color};")?;
            }
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "color: {};", to_css_value(value))?;
            }
        }

        Ok(())
    }
}
