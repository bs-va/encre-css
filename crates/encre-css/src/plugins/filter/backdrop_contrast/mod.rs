#![doc = include_str!("README.md")]
use super::{CSS_BACKDROP_FILTER_1, CSS_BACKDROP_FILTER_2};
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
        "backdrop-contrast"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => value.parse::<usize>().is_ok(),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            #[allow(clippy::cast_precision_loss)]
            Modifier::Builtin { value, .. } => writeln!(
                context.buffer,
                "--en-backdrop-contrast: contrast({});",
                value.parse::<usize>().unwrap() as f32 / 100.,
            )?,
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        indent(context.indentation, context.buffer)?;
        writeln!(context.buffer, "{}", CSS_BACKDROP_FILTER_1)?;
        indent(context.indentation, context.buffer)?;
        writeln!(context.buffer, "{}", CSS_BACKDROP_FILTER_2)?;

        Ok(())
    }
}
