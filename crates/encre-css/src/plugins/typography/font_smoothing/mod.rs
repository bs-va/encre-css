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
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                ["antialised", "subpixel-antialised"].contains(&&**value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "antialised" => {
                    writeln!(context.buffer, "-webkit-font-smoothing: antialiased;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "-moz-osx-font-smoothing: grayscale;")?;
                }
                "subpixel-antialised" => {
                    writeln!(context.buffer, "-webkit-font-smoothing: auto;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "-moz-osx-font-smoothing: auto;")?;
                }
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
