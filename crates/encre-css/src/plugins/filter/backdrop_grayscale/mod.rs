#![doc = include_str!("README.md")]
#![doc(alias = "filter")]
use super::CSS_BACKDROP_FILTER;
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => value.is_empty() || value.parse::<usize>().is_ok(),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            #[allow(clippy::cast_precision_loss)]
            Modifier::Builtin { value, .. } => match *value {
                "" => context
                    .buffer
                    .line("--en-backdrop-grayscale: grayscale(100%);"),
                _ => context.buffer.line(format_args!(
                    "--en-backdrop-grayscale: grayscale({});",
                    value.parse::<usize>().unwrap() as f32 / 100.
                )),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        context.buffer.lines(CSS_BACKDROP_FILTER);
    }
}
