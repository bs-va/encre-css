#![doc = include_str!("README.md")]
use crate::{
    context::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
    utils::indent,
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "snap"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                ["x", "y", "both", "mandatory", "proximity", "none"].contains(&&**value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "none" => {
                    writeln!(context.buffer, "-ms-scroll-snap-type: none;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "scroll-snap-type: none;")?;
                }
                "x" => {
                    writeln!(
                        context.buffer,
                        "-ms-scroll-snap-type: x var(--en-scroll-snap-strictness);"
                    )?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(
                        context.buffer,
                        "scroll-snap-type: x var(--en-scroll-snap-strictness);"
                    )?;
                }
                "y" => {
                    writeln!(
                        context.buffer,
                        "-ms-scroll-snap-type: y var(--en-scroll-snap-strictness);"
                    )?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(
                        context.buffer,
                        "scroll-snap-type: y var(--en-scroll-snap-strictness);"
                    )?;
                }
                "both" => {
                    writeln!(
                        context.buffer,
                        "-ms-scroll-snap-type: both var(--en-scroll-snap-strictness);"
                    )?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(
                        context.buffer,
                        "scroll-snap-type: both var(--en-scroll-snap-strictness);"
                    )?;
                }
                "mandatory" => writeln!(context.buffer, "--en-scroll-snap-strictness: mandatory;")?,
                "proximity" => writeln!(context.buffer, "--en-scroll-snap-strictness: proximity;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
