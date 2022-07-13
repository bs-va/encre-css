#![doc = include_str!("README.md")]
use crate::{
    generator::{generate_at_rules, generate_class, ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
    utils::indent,
};

use std::fmt::{self, Write};

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

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        generate_at_rules(context, |context| {
            generate_class(
                context,
                |context| {
                    indent(context.indentation, context.buffer)?;
                    match context.modifier {
                        Modifier::Builtin { value, .. } => match *value {
                            "solid" => writeln!(context.buffer, "border-style: solid;")?,
                            "dashed" => writeln!(context.buffer, "border-style: dashed;")?,
                            "dotted" => writeln!(context.buffer, "border-style: dotted;")?,
                            "double" => writeln!(context.buffer, "border-style: double;")?,
                            "none" => writeln!(context.buffer, "border-style: none;")?,
                            _ => unreachable!(),
                        },
                        Modifier::Arbitrary { .. } => unreachable!(),
                    }

                    Ok(())
                },
                " > :not([hidden]) ~ :not([hidden])",
            )
        })
    }
}
