#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "ring-opacity"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => value.parse::<usize>().map_or(false, |v| v <= 100),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            #[allow(clippy::cast_precision_loss)]
            Modifier::Builtin { value, .. } => writeln!(
                buffer,
                "{indentation}--en-ring-opacity: {};",
                value.parse::<usize>().unwrap() as f32 / 100.,
            )?,
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
