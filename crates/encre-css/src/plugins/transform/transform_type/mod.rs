#![doc = include_str!("README.md")]
#![doc(alias = "transform")]
use super::CSS_TRANSFORM;
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        matches!(
            context.modifier,
            Modifier::Builtin {
                value: "" | "gpu" | "cpu" | "none",
                ..
            }
        )
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "" | "cpu" => context.buffer.line(CSS_TRANSFORM),
                "gpu" => context.buffer.line("transform: translate3d(var(--en-translate-x), var(--en-translate-y), var(--en-translate-z)) rotateX(var(--en-rotate-x)) rotateY(var(--en-rotate-y)) rotateZ(var(--en-rotate-z)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scale3d(var(--en-scale-x), var(--en-scale-y), var(--en-scale-z));"),
                "none" => context.buffer.line("transform: none;"),
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }
    }
}
