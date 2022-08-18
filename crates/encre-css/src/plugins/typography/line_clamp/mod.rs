#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::{
    generator::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
    utils::indent,
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "line-clamp"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => *value == "none" || value.parse::<usize>().is_ok(),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                if *value == "none" {
                    writeln!(context.buffer, "-webkit-line-clamp: unset;")
                } else {
                    writeln!(context.buffer, "overflow: hidden;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "display: -webkit-box;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "-webkit-box-orient: vertical;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "-webkit-line-clamp: {value};")
                }
            }
            Modifier::Arbitrary { .. } => unreachable!(),
        }
    }
}
