#![doc = include_str!("README.md")]
#![doc(alias = "border")]
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

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { value, .. } => match *value {
                "" => writeln!(buffer, "{indentation}outline-style: solid;")?,
                "none" => {
                    writeln!(buffer, "{indentation}outline: 2px solid transparent;\n{indentation}outline-offset: 2px;")?;
                }
                "dashed" => writeln!(buffer, "{indentation}outline-style: dashed;")?,
                "dotted" => writeln!(buffer, "{indentation}outline-style: dotted;")?,
                "double" => writeln!(buffer, "{indentation}outline-style: double;")?,
                "hidden" => writeln!(buffer, "{indentation}outline-style: hidden;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
