#![doc = include_str!("README.md")]
#![doc(alias("effect", "bg", "background"))]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        matches!(
            context.modifier,
            Modifier::Builtin {
                value: "normal"
                    | "multiply"
                    | "screen"
                    | "overlay"
                    | "darken"
                    | "lighten"
                    | "color-dodge"
                    | "color-burn"
                    | "hard-light"
                    | "soft-light"
                    | "difference"
                    | "exclusion"
                    | "hue"
                    | "saturation"
                    | "color"
                    | "luminosity",
                ..
            }
        )
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => context
                .buffer
                .line(format_args!("background-blend-mode: {value};")),
            Modifier::Arbitrary { .. } => unreachable!(),
        }
    }
}
