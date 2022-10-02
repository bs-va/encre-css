#![doc = include_str!("README.md")]
#![doc(alias = "filter")]
use super::CSS_FILTER;
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "blur"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                ["", "sm", "md", "lg", "xl", "2xl", "3xl", "none"].contains(&&**value)
            }
            Modifier::Arbitrary { value, .. } => is_matching_length(value),
        }
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { value, .. } => match *value {
                "" => writeln!(buffer, "{indentation}--en-blur: blur(8px);")?,
                "sm" => writeln!(buffer, "{indentation}--en-blur: blur(4px);")?,
                "md" => writeln!(buffer, "{indentation}--en-blur: blur(12px);")?,
                "lg" => writeln!(buffer, "{indentation}--en-blur: blur(16px);")?,
                "xl" => writeln!(buffer, "{indentation}--en-blur: blur(24px);")?,
                "2xl" => writeln!(buffer, "{indentation}--en-blur: blur(40px);")?,
                "3xl" => writeln!(buffer, "{indentation}--en-blur: blur(64px);")?,
                "none" => writeln!(buffer, "{indentation}--en-blur: blur(0);")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "{indentation}--en-blur: blur({value});")?;
            }
        }

        writeln!(buffer, "{indentation}{}", CSS_FILTER)?;

        Ok(())
    }
}
