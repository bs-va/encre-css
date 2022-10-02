#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "border"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => is_matching_line_style(value),
            Modifier::Arbitrary { value, .. } => value.split('_').all(is_matching_line_style),
        }
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { value, .. } => writeln!(buffer, "{indentation}border-style: {value};")?,
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "{indentation}border-style: {value};")?;
            }
        }

        Ok(())
    }
}
