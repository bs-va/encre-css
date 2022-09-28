#![doc = include_str!("README.md")]
#![doc(alias = "border")]
use crate::{
    generator::{generate_at_rules, generate_class, ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "divide-opacity"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => value.parse::<usize>().map_or(false, |v| v <= 100),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn needs_wrapping(&self) -> bool {
        false
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        generate_at_rules(context, |context| {
            generate_class(
                context,
                |ContextHandle { modifier, indentation, buffer, .. }| {
                    match modifier {
                        #[allow(clippy::cast_precision_loss)]
                        Modifier::Builtin { value, .. } => writeln!(
                            buffer,
                            "{indentation}--en-divide-opacity: {};",
                            value.parse::<usize>().unwrap() as f32 / 100.,
                        )?,
                        Modifier::Arbitrary { .. } => unreachable!(),
                    }

                    Ok(())
                },
                " > :not([hidden]) ~ :not([hidden])",
            )
        })
    }
}
