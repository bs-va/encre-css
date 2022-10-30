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
            Modifier::Builtin { value, .. } => match *value {
                "normal" => context.buffer.line("background-blend-mode: normal;"),
                "multiply" => context.buffer.line("background-blend-mode: multiply;"),
                "screen" => context.buffer.line("background-blend-mode: screen;"),
                "overlay" => context.buffer.line("background-blend-mode: overlay;"),
                "darken" => context.buffer.line("background-blend-mode: darken;"),
                "lighten" => context.buffer.line("background-blend-mode: lighten;"),
                "color-dodge" => context.buffer.line("background-blend-mode: color-dodge;"),
                "color-burn" => context.buffer.line("background-blend-mode: color-burn;"),
                "hard-light" => context.buffer.line("background-blend-mode: hard-light;"),
                "soft-light" => context.buffer.line("background-blend-mode: soft-light;"),
                "difference" => context.buffer.line("background-blend-mode: difference;"),
                "exclusion" => context.buffer.line("background-blend-mode: exclusion;"),
                "hue" => context.buffer.line("background-blend-mode: hue;"),
                "saturation" => context.buffer.line("background-blend-mode: saturation;"),
                "color" => context.buffer.line("background-blend-mode: color;"),
                "luminosity" => context.buffer.line("background-blend-mode: luminosity;"),
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }
    }
}
