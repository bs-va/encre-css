#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "ring"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                color::is_matching_builtin_color(context.config, value)
            }
            Modifier::Arbitrary { hint, value, .. } => {
                *hint == "color" || (hint.is_empty() && is_matching_color(value))
            }
        }
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, config, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { value, .. } => {
                let color = color::get(config, value, Some("--en-ring-opacity")).unwrap();
                if color.contains("--en-ring-opacity") {
                    writeln!(buffer, "{indentation}--en-ring-opacity: 1;")?;
                }

                writeln!(buffer, "{indentation}--en-ring-color: {color};")?;
            }
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "{indentation}--en-ring-color: {value};")?;
            }
        }

        Ok(())
    }
}
