#![doc = include_str!("README.md")]
#![doc(alias = "transform")]
use super::CSS_TRANSFORM;
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "transform"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => ["", "gpu", "cpu", "none"].contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { value, .. } => match *value {
                "" | "cpu" => writeln!(buffer, "{indentation}{}", CSS_TRANSFORM)?,
                "gpu" => writeln!(buffer, "{indentation}transform: translate3d(var(--en-translate-x), var(--en-translate-y), 0) rotate(var(--en-rotate)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scaleX(var(--en-scale-x)) scaleY(var(--en-scale-y));")?,
                "none" => writeln!(buffer, "{indentation}transform: none;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
