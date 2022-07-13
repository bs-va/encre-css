#![doc = include_str!("README.md")]
use crate::{
    generator::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
    utils::{indent, spacing, value_matchers::is_matching_length},
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "max-h"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                [
                    "xs", "sm", "md", "lg", "xl", "2xl", "3xl", "4xl", "5xl", "6xl", "7xl", "full",
                    "min", "max", "screen", "fit", "none",
                ]
                .contains(&&**value)
                    || spacing::is_matching_builtin_spacing(value)
            }
            Modifier::Arbitrary { value, .. } => is_matching_length(value),
        }
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { is_negative, value } => match *value {
                "none" => writeln!(context.buffer, "max-height: none;")?,
                "xs" => writeln!(context.buffer, "max-height: 20rem;")?,
                "sm" => writeln!(context.buffer, "max-height: 24rem;")?,
                "md" => writeln!(context.buffer, "max-height: 28rem;")?,
                "lg" => writeln!(context.buffer, "max-height: 32rem;")?,
                "xl" => writeln!(context.buffer, "max-height: 36rem;")?,
                "2xl" => writeln!(context.buffer, "max-height: 42rem;")?,
                "3xl" => writeln!(context.buffer, "max-height: 48rem;")?,
                "4xl" => writeln!(context.buffer, "max-height: 56rem;")?,
                "5xl" => writeln!(context.buffer, "max-height: 64rem;")?,
                "6xl" => writeln!(context.buffer, "max-height: 72rem;")?,
                "7xl" => writeln!(context.buffer, "max-height: 80rem;")?,
                "full" => writeln!(context.buffer, "max-height: 100%;")?,
                "min" => writeln!(context.buffer, "max-height: min-content;")?,
                "max" => writeln!(context.buffer, "max-height: max-content;")?,
                "screen" => writeln!(context.buffer, "max-height: 100vh;")?,
                "fit" => writeln!(context.buffer, "max-height: fit-content;")?,
                _ => writeln!(
                    context.buffer,
                    "max-height: {};",
                    spacing::get(value, *is_negative).unwrap()
                )?,
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "max-height: {value};")?;
            }
        }

        Ok(())
    }
}
