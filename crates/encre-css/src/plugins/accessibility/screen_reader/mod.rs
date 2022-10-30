#![doc = include_str!("README.md")]
#![doc(alias = "accessibility")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        matches!(context.modifier, Modifier::Builtin { value: "sr-only" | "not-sr-only", .. })
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "sr-only" => context.buffer.lines([
                    "position: absolute;",
                    "width: 1px;",
                    "height: 1px;",
                    "padding: 0;",
                    "margin: -1px;",
                    "overflow: hidden;",
                    "clip: rect(0, 0, 0, 0);",
                    "white-space: nowrap;",
                    "border-width: 0;",
                ]),
                "not-sr-only" => context.buffer.lines([
                    "position: static;",
                    "width: auto;",
                    "height: auto;",
                    "padding: 0;",
                    "margin: 0;",
                    "overflow: visible;",
                    "clip: auto;",
                    "white-space: normal;",
                ]),
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }
    }
}
