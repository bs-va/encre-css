#![doc = include_str!("README.md")]
#![doc(alias = "filter")]
use super::{CSS_BACKDROP_FILTER_1, CSS_BACKDROP_FILTER_2};
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "backdrop-brightness"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => value.parse::<usize>().is_ok(),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            #[allow(clippy::cast_precision_loss)]
            Modifier::Builtin { value, .. } => writeln!(
                buffer,
                "--en-backdrop-brightness: brightness({});",
                value.parse::<usize>().unwrap() as f32 / 100.,
            )?,
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        writeln!(buffer, "{indentation}{}\n{indentation}{}", CSS_BACKDROP_FILTER_1, CSS_BACKDROP_FILTER_2)?;

        Ok(())
    }
}
