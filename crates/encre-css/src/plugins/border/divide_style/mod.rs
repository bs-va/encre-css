#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "divide"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                ["solid", "dashed", "dotted", "double", "none"].contains(value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn needs_wrapping(&self) -> bool {
        false
    }

    fn handle(&self, context: &mut ContextHandle) {
        generate_at_rules(context, |context| {
            generate_class(
                context,
                |context| match context.modifier {
                    Modifier::Builtin { value, .. } => match *value {
                        "solid" => context.buffer.line("border-style: solid;"),
                        "dashed" => context.buffer.line("border-style: dashed;"),
                        "dotted" => context.buffer.line("border-style: dotted;"),
                        "double" => context.buffer.line("border-style: double;"),
                        "none" => context.buffer.line("border-style: none;"),
                        _ => unreachable!(),
                    },
                    Modifier::Arbitrary { .. } => unreachable!(),
                },
                " > :not([hidden]) ~ :not([hidden])",
            )
        })
    }
}
