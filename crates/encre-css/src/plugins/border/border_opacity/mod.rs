#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        matches!(context.modifier, Modifier::Builtin { value, .. } if value.parse::<usize>().map_or(false, |v| v <= 100))
    }

    fn handle(&self, context: &mut ContextHandle) {
        if let Modifier::Builtin { value, .. } = context.modifier {
            #[allow(clippy::cast_precision_loss)]
            context.buffer.line(format_args!(
                "--en-border-opacity: {};",
                value.parse::<usize>().unwrap() as f32 / 100.,
            ));
        }
    }
}
