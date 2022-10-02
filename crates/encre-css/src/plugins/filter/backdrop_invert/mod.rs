#![doc = include_str!("README.md")]
#![doc(alias = "filter")]
use super::{CSS_BACKDROP_FILTER_1, CSS_BACKDROP_FILTER_2};
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "backdrop-invert"
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
                "" => writeln!(buffer, "{indentation}--en-backdrop-invert: invert(100%);")?,
                _ => writeln!(
                    buffer,
                    "--en-backdrop-invert: invert({});",
                    value.parse::<usize>().unwrap() as f32 / 100.
                )?,
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        writeln!(buffer, "{indentation}{}\n{indentation}{}", CSS_BACKDROP_FILTER_1, CSS_BACKDROP_FILTER_2)?;

        Ok(())
    }
}
