#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        matches!(context.modifier, Modifier::Builtin { value, .. } if value.parse::<usize>().map_or(false, |v| v <= 100))
    }

    fn needs_wrapping(&self) -> bool {
        false
    }

    fn handle(&self, context: &mut ContextHandle) {
        generate_at_rules(context, |context| {
            generate_class(
                context,
                |context| {
                    #[allow(clippy::cast_precision_loss)]
                    if let Modifier::Builtin { value, .. } = context.modifier {
                        context.buffer.line(format_args!(
                            "--en-divide-opacity: {};",
                            value.parse::<usize>().unwrap() as f32 / 100.,
                        ));
                    }
                },
                " > :not([hidden]) ~ :not([hidden])",
            );
        });
    }
}
