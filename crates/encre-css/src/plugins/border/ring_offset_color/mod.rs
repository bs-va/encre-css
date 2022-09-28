#![doc = include_str!("README.md")]
#![doc(alias = "border")]
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

    fn handle(&self, ContextHandle { modifier, buffer, indentation, config, .. }: &mut ContextHandle) -> fmt::Result {
        writeln!(buffer, "{indentation}--en-ring-offset-shadow: var(--en-ring-inset) 0 0 0 var(--en-ring-offset-width) var(--en-ring-offset-color);")?;

        match modifier {
            Modifier::Builtin { value, .. } => writeln!(
                buffer,
                "{indentation}--en-ring-offset-color: {};",
                color::get(config, value, None).unwrap()
            ),
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "{indentation}--en-ring-offset-color: {value};")
            }
        }
    }
}
