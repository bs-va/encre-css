#![doc = include_str!("README.md")]
#![doc(alias = "interactivity")]
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

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { value, .. } => match *value {
                "none" => {
                    writeln!(buffer, "{indentation}-ms-scroll-snap-type: none;\n{indentation}scroll-snap-type: none;")?;
                }
                "x" => {
                    writeln!(
                        buffer,
                        "{indentation}-ms-scroll-snap-type: x var(--en-scroll-snap-strictness);\n{indentation}scroll-snap-type: x var(--en-scroll-snap-strictness);"
                    )?;
                }
                "y" => {
                    writeln!(
                        buffer,
                        "{indentation}-ms-scroll-snap-type: y var(--en-scroll-snap-strictness);\n{indentation}scroll-snap-type: y var(--en-scroll-snap-strictness);"
                    )?;
                }
                "both" => {
                    writeln!(
                        buffer,
                        "{indentation}-ms-scroll-snap-type: both var(--en-scroll-snap-strictness);\n{indentation}scroll-snap-type: both var(--en-scroll-snap-strictness);"
                    )?;
                }
                "mandatory" => writeln!(buffer, "{indentation}--en-scroll-snap-strictness: mandatory;")?,
                "proximity" => writeln!(buffer, "{indentation}--en-scroll-snap-strictness: proximity;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
