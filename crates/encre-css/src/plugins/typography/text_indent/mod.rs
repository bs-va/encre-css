#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "indent"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => spacing::is_matching_builtin_spacing(value),
            Modifier::Arbitrary { value, .. } => is_matching_length(value),
        }
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { is_negative, value } => {
                writeln!(
                    buffer,
                    "{indentation}text-indent: {};",
                    spacing::get(value, *is_negative).unwrap(),
                )?;
            }
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "{indentation}text-indent: {value};")?;
            }
        }

        Ok(())
    }
}
