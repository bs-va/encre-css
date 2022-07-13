#![doc = include_str!("README.md")]
use super::CSS_FILTER;
use crate::{
    generator::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
    utils::{indent, value_matchers::is_matching_shadow},
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "drop-shadow"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                ["", "sm", "md", "lg", "xl", "2xl", "none"].contains(&&**value)
            }
            Modifier::Arbitrary { value, .. } => is_matching_shadow(value),
        }
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "" => writeln!(context.buffer, "--en-drop-shadow: drop-shadow(0 1px 2px rgb(0 0 0 / 0.1)) drop-shadow(0 1px 1px rgb(0 0 0 / 0.06));")?,
                "sm" => writeln!(context.buffer, "--en-drop-shadow: drop-shadow(0 1px 1px rgb(0 0 0 / 0.05));")?,
                "md" => writeln!(context.buffer, "--en-drop-shadow: drop-shadow(0 4px 3px rgb(0 0 0 / 0.07)) drop-shadow(0 2px 2px rgb(0 0 0 / 0.06));")?,
                "lg" => writeln!(context.buffer, "--en-drop-shadow: drop-shadow(0 10px 8px rgb(0 0 0 / 0.04)) drop-shadow(0 4px 3px rgb(0 0 0 / 0.1));")?,
                "xl" => writeln!(context.buffer, "--en-drop-shadow: drop-shadow(0 20px 13px rgb(0 0 0 / 0.03)) drop-shadow(0 8px 5px rgb(0 0 0 / 0.08));")?,
                "2xl" => writeln!(context.buffer, "--en-drop-shadow: drop-shadow(0 25px 25px rgb(0 0 0 / 0.15));")?,
                "none" => writeln!(context.buffer, "--en-drop-shadow: drop-shadow(0 0 #0000);")?,
                _ => unreachable!(),
            }
            Modifier::Arbitrary { value, .. } => writeln!(context.buffer, "--en-drop-shadow: drop-shadow({value});")?,
        }

        indent(context.indentation, context.buffer)?;
        writeln!(context.buffer, "{}", CSS_FILTER)?;

        Ok(())
    }
}
