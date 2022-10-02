#![doc = include_str!("README.md")]
#![doc(alias = "filter")]
use super::CSS_FILTER;
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "hue-rotate"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => value.parse::<usize>().map_or(false, |v| v <= 360),
            Modifier::Arbitrary { value, .. } => is_matching_angle(value),
        }
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { is_negative, value } => writeln!(
                buffer,
                "{indentation}--en-hue-rotate: hue-rotate({}{}deg);",
                format_negative(is_negative),
                value
            )?,
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "{indentation}--en-hue-rotate: hue-rotate({value});")?;
            }
        }

        writeln!(buffer, "{indentation}{}", CSS_FILTER)?;

        Ok(())
    }
}
