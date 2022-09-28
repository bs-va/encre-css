#![doc = include_str!("README.md")]
#![doc(alias = "typography")]
use crate::{
    generator::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
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

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { value, .. } => match *value {
                "normal-nums" => return writeln!(buffer, "{indentation}font-variant-numeric: normal;"),
                "ordinal" => writeln!(buffer, "{indentation}--en-ordinal: ordinal;")?,
                "slashed-zero" => writeln!(buffer, "{indentation}--en-slashed-zero: slashed-zero;")?,
                "lining-nums" => writeln!(buffer, "{indentation}--en-numeric-figure: lining-nums;")?,
                "oldstyle-nums" => writeln!(buffer, "{indentation}--en-numeric-figure: oldstyle-nums;")?,
                "proportional-nums" => {
                    writeln!(buffer, "{indentation}--en-numeric-spacing: proportional-nums;")?;
                }
                "tabular-nums" => writeln!(buffer, "{indentation}--en-numeric-spacing: tabular-nums;")?,
                "diagonal-fractions" => {
                    writeln!(buffer, "{indentation}--en-numeric-fraction: diagonal-fractions;")?;
                }
                "stacked-fractions" => {
                    writeln!(buffer, "{indentation}--en-numeric-fraction: stacked-fractions;")?;
                }
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        writeln!(buffer, "{indentation}font-variant-numeric: var(--en-ordinal) var(--en-slashed-zero) var(--en-numeric-figure) var(--en-numeric-spacing) var(--en-numeric-fraction);")?;

        Ok(())
    }
}
