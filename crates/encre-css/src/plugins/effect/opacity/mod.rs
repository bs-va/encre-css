#![doc = include_str!("README.md")]
#![doc(alias = "effect")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => value.parse::<usize>().map_or(false, |v| v <= 100),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: &mut ContextHandle) {
        match context.modifier {
            #[allow(clippy::cast_precision_loss)]
            Modifier::Builtin { value, .. } => context.buffer.line(format_args!(
                "opacity: {};",
                value.parse::<usize>().unwrap() as f32 / 100.
            )),
            Modifier::Arbitrary { .. } => unreachable!(),
        }
    }
}
