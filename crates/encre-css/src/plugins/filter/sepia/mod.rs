#![doc = include_str!("README.md")]
#![doc(alias = "filter")]
use super::CSS_FILTER;
use crate::{
    generator::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "sepia"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => value.is_empty() || value.parse::<usize>().is_ok(),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            #[allow(clippy::cast_precision_loss)]
            Modifier::Builtin { value, .. } => match *value {
                "" => writeln!(buffer, "{indentation}--en-sepia: sepia(100%);")?,
                _ => writeln!(
                    buffer,
                    "--en-sepia: sepia({});",
                    value.parse::<usize>().unwrap() as f32 / 100.
                )?,
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        writeln!(buffer, "{indentation}{}", CSS_FILTER)?;

        Ok(())
    }
}
