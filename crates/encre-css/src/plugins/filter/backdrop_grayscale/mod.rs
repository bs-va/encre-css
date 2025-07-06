#![doc = include_str!("README.md")]
#![doc(alias = "filter")]
use super::CSS_BACKDROP_FILTER;
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        matches!(context.modifier, Modifier::Builtin { value, .. } if value.is_empty() || value.parse::<usize>().is_ok())
    }

    fn handle(&self, context: &mut ContextHandle) {
        if let Modifier::Builtin { value, .. } = context.modifier {
            #[allow(clippy::cast_precision_loss)]
            match *value {
                "" => context
                    .buffer
                    .line("--en-backdrop-grayscale: grayscale(100%);"),
                _ => context.buffer.line(format_args!(
                    "--en-backdrop-grayscale: grayscale({});",
                    value.parse::<usize>().unwrap() as f32 / 100.
                )),
            }

            context.buffer.lines(CSS_BACKDROP_FILTER);
        }
    }
}
