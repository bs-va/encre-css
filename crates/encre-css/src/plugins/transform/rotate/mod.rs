#![doc = include_str!("README.md")]
#![doc(alias = "transform")]
use super::CSS_TRANSFORM;
use crate::{
    generator::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
    utils::{format_negative, value_matchers::is_matching_angle},
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &'static str {
        "rotate"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => value.parse::<usize>().is_ok(),
            Modifier::Arbitrary { value, .. } => is_matching_angle(value),
        }
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { is_negative, value } => writeln!(
                buffer,
                "{indentation}--en-rotate: {}{}deg;",
                format_negative(is_negative),
                value.parse::<usize>().unwrap(),
            )?,
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "{indentation}--en-rotate: {value};")?;
            }
        }

        writeln!(buffer, "{indentation}{}", CSS_TRANSFORM)?;
        Ok(())
    }
}
