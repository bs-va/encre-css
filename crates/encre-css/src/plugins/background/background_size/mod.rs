#![doc = include_str!("README.md")]
use crate::{
    generator::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
    utils::{
        indent,
        value_matchers::{is_matching_length, is_matching_percentage},
    },
};
use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "bg"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => ["contain", "cover", "auto"].contains(value),
            Modifier::Arbitrary { hint, value, .. } => {
                *hint == "length"
                    || *hint == "percentage"
                    || (hint.is_empty()
                        && value.split(',').all(|v| {
                            v.split('_').all(|v| {
                                is_matching_length(v)
                                    || is_matching_percentage(v)
                                    || ["contain", "cover", "auto"].contains(&v)
                            })
                        }))
            }
        }
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "auto" => writeln!(context.buffer, "background-size: auto;")?,
                "cover" => writeln!(context.buffer, "background-size: cover;")?,
                "contain" => writeln!(context.buffer, "background-size: contain;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "background-size: {value};")?;
            }
        }

        Ok(())
    }
}
