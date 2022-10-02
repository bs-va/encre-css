#![doc = include_str!("README.md")]
#![doc(alias = "transition")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "ease"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => value.parse::<usize>().is_ok(),
            Modifier::Arbitrary { value, .. } => is_matching_all(value),
        }
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { value, .. } => match *value {
                "linear" => writeln!(buffer, "{indentation}transition-timing-function: linear;")?,
                "in" => writeln!(
                    buffer,
                    "{indentation}transition-timing-function: cubic-bezier(0.4, 0, 1, 1);"
                )?,
                "out" => writeln!(
                    buffer,
                    "{indentation}transition-timing-function: cubic-bezier(0, 0, 0.2, 1);"
                )?,
                "in-out" => writeln!(
                    buffer,
                    "{indentation}transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);"
                )?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "{indentation}transition-timing-function: {value};")?;
            }
        }

        Ok(())
    }
}
