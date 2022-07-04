#![doc = include_str!("README.md")]
use super::CSS_FILTER;
use crate::{
    context::{ContextCanHandle, ContextHandle},
    plugins::{to_css_value, Plugin},
    selector::Modifier,
    utils::{indent, value_matchers::is_matching_length},
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "blur"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                ["", "sm", "md", "lg", "xl", "2xl", "3xl", "none"].contains(&&**value)
            }
            Modifier::Arbitrary { value, .. } => is_matching_length(value),
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "" => writeln!(context.buffer, "--en-blur: blur(8px);")?,
                "sm" => writeln!(context.buffer, "--en-blur: blur(4px);")?,
                "md" => writeln!(context.buffer, "--en-blur: blur(12px);")?,
                "lg" => writeln!(context.buffer, "--en-blur: blur(16px);")?,
                "xl" => writeln!(context.buffer, "--en-blur: blur(24px);")?,
                "2xl" => writeln!(context.buffer, "--en-blur: blur(40px);")?,
                "3xl" => writeln!(context.buffer, "--en-blur: blur(64px);")?,
                "none" => writeln!(context.buffer, "--en-blur: blur(0);")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "--en-blur: blur({});", to_css_value(value))?;
            }
        }

        indent(context.indentation, context.buffer)?;
        writeln!(context.buffer, "{}", CSS_FILTER)?;

        Ok(())
    }
}
