#![doc = include_str!("README.md")]
#![doc(alias("sizing", "size"))]
use crate::{
    generator::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
    utils::{spacing, value_matchers::is_matching_length},
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

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { is_negative, value } => match *value {
                "none" => writeln!(buffer, "{indentation}max-height: none;")?,
                "xs" => writeln!(buffer, "{indentation}max-height: 20rem;")?,
                "sm" => writeln!(buffer, "{indentation}max-height: 24rem;")?,
                "md" => writeln!(buffer, "{indentation}max-height: 28rem;")?,
                "lg" => writeln!(buffer, "{indentation}max-height: 32rem;")?,
                "xl" => writeln!(buffer, "{indentation}max-height: 36rem;")?,
                "2xl" => writeln!(buffer, "{indentation}max-height: 42rem;")?,
                "3xl" => writeln!(buffer, "{indentation}max-height: 48rem;")?,
                "4xl" => writeln!(buffer, "{indentation}max-height: 56rem;")?,
                "5xl" => writeln!(buffer, "{indentation}max-height: 64rem;")?,
                "6xl" => writeln!(buffer, "{indentation}max-height: 72rem;")?,
                "7xl" => writeln!(buffer, "{indentation}max-height: 80rem;")?,
                "full" => writeln!(buffer, "{indentation}max-height: 100%;")?,
                "min" => writeln!(buffer, "{indentation}max-height: min-content;")?,
                "max" => writeln!(buffer, "{indentation}max-height: max-content;")?,
                "screen" => writeln!(buffer, "{indentation}max-height: 100vh;")?,
                "fit" => writeln!(buffer, "{indentation}max-height: fit-content;")?,
                _ => writeln!(
                    buffer,
                    "{indentation}max-height: {};",
                    spacing::get(value, *is_negative).unwrap()
                )?,
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "{indentation}max-height: {value};")?;
            }
        }

        Ok(())
    }
}
