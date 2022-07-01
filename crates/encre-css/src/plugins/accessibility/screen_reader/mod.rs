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
            Modifier::Builtin { value, .. } => *value == "sr-only" || *value == "not-sr-only",
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "sr-only" => {
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "position: absolute;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "width: 1px;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "height: 1px;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "padding: 0;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "margin: -1px;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "overflow: hidden;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "clip: rect(0, 0, 0, 0);")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "white-space: nowrap;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "border-width: 0;")?;
                }
                "not-sr-only" => {
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "position: static;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "width: auto;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "height: auto;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "padding: 0;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "margin: 0;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "overflow: visible;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "clip: auto;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "white-space: normal;")?;
                }
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
