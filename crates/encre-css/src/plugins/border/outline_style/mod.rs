#![doc = include_str!("README.md")]
use crate::{
    plugins::Plugin,
    context::{ContextCanHandle, ContextHandle},
    selector::Modifier,
    utils::indent,
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "outline"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                ["", "dashed", "dotted", "double", "hidden", "none"].contains(&&**value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "" => writeln!(context.buffer, "outline-style: solid;")?,
                "none" => {
                    writeln!(context.buffer, "outline: 2px solid transparent;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "outline-offset: 2px;")?;
                }
                "dashed" => writeln!(context.buffer, "outline-style: dashed;")?,
                "dotted" => writeln!(context.buffer, "outline-style: dotted;")?,
                "double" => writeln!(context.buffer, "outline-style: double;")?,
                "hidden" => writeln!(context.buffer, "outline-style: hidden;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
