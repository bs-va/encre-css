#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::{
    generator::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
    utils::{color, value_matchers::is_matching_color},
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

    fn handle(&self, ContextHandle { modifier, buffer, indentation, config, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { value, .. } => {
                let color = color::get(config, value, Some("--en-text-opacity")).unwrap();
                if color.contains("--en-text-opacity") {
                    writeln!(buffer, "{indentation}--en-text-opacity: 1;")?;
                }

                writeln!(buffer, "{indentation}color: {color};")?;
            }
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "{indentation}color: {value};")?;
            }
        }

        Ok(())
    }
}
