#![doc = include_str!("README.md")]
#![doc(alias = "filter")]
use super::CSS_FILTER;
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "drop-shadow"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                ["", "sm", "md", "lg", "xl", "2xl", "none"].contains(&&**value)
            }
            Modifier::Arbitrary { value, .. } => is_matching_shadow(value),
        }
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { value, .. } => match *value {
                "" => writeln!(buffer, "{indentation}--en-drop-shadow: drop-shadow(0 1px 2px rgb(0 0 0 / 0.1)) drop-shadow(0 1px 1px rgb(0 0 0 / 0.06));")?,
                "sm" => writeln!(buffer, "{indentation}--en-drop-shadow: drop-shadow(0 1px 1px rgb(0 0 0 / 0.05));")?,
                "md" => writeln!(buffer, "{indentation}--en-drop-shadow: drop-shadow(0 4px 3px rgb(0 0 0 / 0.07)) drop-shadow(0 2px 2px rgb(0 0 0 / 0.06));")?,
                "lg" => writeln!(buffer, "{indentation}--en-drop-shadow: drop-shadow(0 10px 8px rgb(0 0 0 / 0.04)) drop-shadow(0 4px 3px rgb(0 0 0 / 0.1));")?,
                "xl" => writeln!(buffer, "{indentation}--en-drop-shadow: drop-shadow(0 20px 13px rgb(0 0 0 / 0.03)) drop-shadow(0 8px 5px rgb(0 0 0 / 0.08));")?,
                "2xl" => writeln!(buffer, "{indentation}--en-drop-shadow: drop-shadow(0 25px 25px rgb(0 0 0 / 0.15));")?,
                "none" => writeln!(buffer, "{indentation}--en-drop-shadow: drop-shadow(0 0 #0000);")?,
                _ => unreachable!(),
            }
            Modifier::Arbitrary { value, .. } => writeln!(buffer, "{indentation}--en-drop-shadow: drop-shadow({value});")?,
        }

        writeln!(buffer, "{indentation}{}", CSS_FILTER)?;

        Ok(())
    }
}
