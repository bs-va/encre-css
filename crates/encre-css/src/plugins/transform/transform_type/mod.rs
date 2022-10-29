#![doc = include_str!("README.md")]
#![doc(alias = "transform")]
use super::CSS_TRANSFORM;
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => ["", "gpu", "cpu", "none"].contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "" | "cpu" => context.buffer.line(CSS_TRANSFORM),
                "gpu" => context.buffer.line("transform: translate3d(var(--en-translate-x), var(--en-translate-y), 0) rotate(var(--en-rotate)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scaleX(var(--en-scale-x)) scaleY(var(--en-scale-y));"),
                "none" => context.buffer.line("transform: none;"),
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }
    }
}
