#![doc = include_str!("README.md")]
#![doc(alias("background", "bg"))]
use crate::{
    generator::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
};
use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "bg-clip"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                ["border", "padding", "content", "text"].contains(value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { value, .. } => match *value {
                "border" => writeln!(buffer, "{indentation}background-clip: border-box;")?,
                "padding" => writeln!(buffer, "{indentation}background-clip: padding-box;")?,
                "content" => writeln!(buffer, "{indentation}background-clip: content-box;")?,
                "text" => writeln!(buffer, "{indentation}background-clip: text;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
