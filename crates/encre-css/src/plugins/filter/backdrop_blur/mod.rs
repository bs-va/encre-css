#![doc = include_str!("README.md")]
use super::{CSS_BACKDROP_FILTER_1, CSS_BACKDROP_FILTER_2};
use crate::{
    generator::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
    utils::{indent, value_matchers::is_matching_length},
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "backdrop-blur"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                ["", "sm", "md", "lg", "xl", "2xl", "3xl", "none"].contains(&&**value)
            }
            Modifier::Arbitrary { value, .. } => is_matching_length(value),
        }
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "" => writeln!(context.buffer, "--en-backdrop-blur: blur(8px);")?,
                "sm" => writeln!(context.buffer, "--en-backdrop-blur: blur(4px);")?,
                "md" => writeln!(context.buffer, "--en-backdrop-blur: blur(12px);")?,
                "lg" => writeln!(context.buffer, "--en-backdrop-blur: blur(16px);")?,
                "xl" => writeln!(context.buffer, "--en-backdrop-blur: blur(24px);")?,
                "2xl" => writeln!(context.buffer, "--en-backdrop-blur: blur(40px);")?,
                "3xl" => writeln!(context.buffer, "--en-backdrop-blur: blur(64px);")?,
                "none" => writeln!(context.buffer, "--en-backdrop-blur: blur(0);")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "--en-backdrop-blur: blur({value});")?;
            }
        }

        indent(context.indentation, context.buffer)?;
        writeln!(context.buffer, "{}", CSS_BACKDROP_FILTER_1)?;
        indent(context.indentation, context.buffer)?;
        writeln!(context.buffer, "{}", CSS_BACKDROP_FILTER_2)?;

        Ok(())
    }
}
