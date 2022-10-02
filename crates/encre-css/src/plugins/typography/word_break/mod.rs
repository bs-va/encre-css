#![doc = include_str!("README.md")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "break"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => ["normal", "words", "all"].contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { value, .. } => match *value {
                "normal" => {
                    writeln!(buffer, "{indentation}overflow-wrap: normal;\n{indentation}word-break: normal;")?;
                }
                "words" => writeln!(buffer, "{indentation}overflow-wrap: break-word;")?,
                "all" => writeln!(buffer, "{indentation}word-break: break-all;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
