#![doc = include_str!("README.md")]
#![doc(alias = "effect")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "mix-blend"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => [
                "normal",
                "multiply",
                "screen",
                "overlay",
                "darken",
                "lighten",
                "color-dodge",
                "color-burn",
                "hard-light",
                "soft-light",
                "difference",
                "exclusion",
                "hue",
                "saturation",
                "color",
                "luminosity",
                "plus-lighter",
            ]
            .contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { value, .. } => {
                writeln!(buffer, "{indentation}mix-blend-mode: {value};")?;
            }
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
