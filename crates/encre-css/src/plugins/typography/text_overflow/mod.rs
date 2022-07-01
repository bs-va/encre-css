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
                ["truncate", "text-ellipsis", "text-clip"].contains(&&**value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "truncate" => {
                    writeln!(context.buffer, "overflow: hidden;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "text-overflow: ellipsis;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "white-space: nowrap;")?;
                }
                "text-ellipsis" => writeln!(context.buffer, "text-overflow: ellipsis;")?,
                "text-clip" => writeln!(context.buffer, "text-overflow: clip;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
