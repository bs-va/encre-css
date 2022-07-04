#![doc = include_str!("README.md")]
use crate::{
    context::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
    utils::indent,
};

use std::fmt::{self, Write};

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

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "normal-nums" => return writeln!(context.buffer, "font-variant-numeric: normal;"),
                "ordinal" => writeln!(context.buffer, "--en-ordinal: ordinal;")?,
                "slashed-zero" => writeln!(context.buffer, "--en-slashed-zero: slashed-zero;")?,
                "lining-nums" => writeln!(context.buffer, "--en-numeric-figure: lining-nums;")?,
                "oldstyle-nums" => writeln!(context.buffer, "--en-numeric-figure: oldstyle-nums;")?,
                "proportional-nums" => {
                    writeln!(context.buffer, "--en-numeric-spacing: proportional-nums;")?;
                }
                "tabular-nums" => writeln!(context.buffer, "--en-numeric-spacing: tabular-nums;")?,
                "diagonal-fractions" => {
                    writeln!(context.buffer, "--en-numeric-fraction: diagonal-fractions;")?;
                }
                "stacked-fractions" => {
                    writeln!(context.buffer, "--en-numeric-fraction: stacked-fractions;")?;
                }
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        indent(context.indentation, context.buffer)?;
        writeln!(context.buffer, "font-variant-numeric: var(--en-ordinal) var(--en-slashed-zero) var(--en-numeric-figure) var(--en-numeric-spacing) var(--en-numeric-fraction);")?;

        Ok(())
    }
}
