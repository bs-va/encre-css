#![doc = include_str!("README.md")]
#![doc(alias("background", "bg"))]
use crate::prelude::build_plugin::*;

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

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { value, .. } => match *value {
                "auto" => writeln!(buffer, "{indentation}background-size: auto;")?,
                "cover" => writeln!(buffer, "{indentation}background-size: cover;")?,
                "contain" => writeln!(buffer, "{indentation}background-size: contain;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "{indentation}background-size: {value};")?;
            }
        }

        Ok(())
    }
}
