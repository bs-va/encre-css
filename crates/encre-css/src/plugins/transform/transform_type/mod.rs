#![doc = include_str!("README.md")]
use super::CSS_TRANSFORM;
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
    fn namespace(&self) -> &str {
        "transform"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => ["", "gpu", "cpu", "none"].contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "" | "cpu" => writeln!(context.buffer, "{}", CSS_TRANSFORM)?,
                "gpu" => writeln!(context.buffer, "transform: translate3d(var(--en-translate-x), var(--en-translate-y), 0) rotate(var(--en-rotate)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scaleX(var(--en-scale-x)) scaleY(var(--en-scale-y));")?,
                "none" => writeln!(context.buffer, "transform: none;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
