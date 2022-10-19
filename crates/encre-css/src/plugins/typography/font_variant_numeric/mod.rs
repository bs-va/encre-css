#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => [
                "normal-nums",
                "ordinal",
                "slashed-zero",
                "lining-nums",
                "oldstyle-nums",
                "proportional-nums",
                "tabular-nums",
                "diagonal-fractions",
                "stacked-fractions",
            ]
            .contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "normal-nums" => return context.buffer.line("font-variant-numeric: normal;"),
                "ordinal" => context.buffer.line("--en-ordinal: ordinal;"),
                "slashed-zero" => context.buffer.line("--en-slashed-zero: slashed-zero;"),
                "lining-nums" => context.buffer.line("--en-numeric-figure: lining-nums;"),
                "oldstyle-nums" => context.buffer.line("--en-numeric-figure: oldstyle-nums;"),
                "proportional-nums" => {
                    context
                        .buffer
                        .line("--en-numeric-spacing: proportional-nums;");
                }
                "tabular-nums" => context.buffer.line("--en-numeric-spacing: tabular-nums;"),
                "diagonal-fractions" => {
                    context
                        .buffer
                        .line("--en-numeric-fraction: diagonal-fractions;");
                }
                "stacked-fractions" => {
                    context
                        .buffer
                        .line("--en-numeric-fraction: stacked-fractions;");
                }
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        context.buffer.line("font-variant-numeric: var(--en-ordinal) var(--en-slashed-zero) var(--en-numeric-figure) var(--en-numeric-spacing) var(--en-numeric-fraction);");
    }
}
