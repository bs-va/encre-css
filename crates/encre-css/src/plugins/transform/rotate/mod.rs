#![doc = include_str!("README.md")]
#![doc(alias = "transform")]
use super::CSS_TRANSFORM;
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &'static str {
        "rotate"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => value.parse::<usize>().is_ok(),
            Modifier::Arbitrary { value, .. } => is_matching_angle(value),
        }
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { is_negative, value } => writeln!(
                buffer,
                "{indentation}--en-rotate: {}{}deg;",
                format_negative(is_negative),
                value.parse::<usize>().unwrap(),
            )?,
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "{indentation}--en-rotate: {value};")?;
            }
        }

        writeln!(buffer, "{indentation}{}", CSS_TRANSFORM)?;
        Ok(())
    }
}
