#![doc = include_str!("README.md")]
use crate::{
    plugins::Plugin,
    context::{ContextCanHandle, ContextHandle},
    selector::Modifier,
    utils::indent,
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &'static str {
        "columns"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                value.parse::<usize>().map_or(false, |v| v <= 12)
                    || [
                        "auto", "3xs", "2xs", "xs", "sm", "md", "lg", "xl", "2xl", "3xl", "4xl",
                        "5xl", "6xl", "7xl",
                    ]
                    .contains(value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "3xs" => writeln!(context.buffer, "columns: 16rem;")?,
                "2xs" => writeln!(context.buffer, "columns: 18rem;")?,
                "xs" => writeln!(context.buffer, "columns: 20rem;")?,
                "sm" => writeln!(context.buffer, "columns: 24rem;")?,
                "md" => writeln!(context.buffer, "columns: 28rem;")?,
                "lg" => writeln!(context.buffer, "columns: 32rem;")?,
                "xl" => writeln!(context.buffer, "columns: 36rem;")?,
                "2xl" => writeln!(context.buffer, "columns: 42rem;")?,
                "3xl" => writeln!(context.buffer, "columns: 48rem;")?,
                "4xl" => writeln!(context.buffer, "columns: 56rem;")?,
                "5xl" => writeln!(context.buffer, "columns: 64rem;")?,
                "6xl" => writeln!(context.buffer, "columns: 72rem;")?,
                "7xl" => writeln!(context.buffer, "columns: 80rem;")?,
                _ => writeln!(context.buffer, "columns: {value};")?,
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
