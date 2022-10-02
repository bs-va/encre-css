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

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        generate_at_rules(context, |context| {
            generate_class(
                context,
                |ContextHandle { modifier, indentation, buffer, .. }| {
                    match modifier {
                        Modifier::Builtin { value, .. } => match *value {
                            "solid" => writeln!(buffer, "{indentation}border-style: solid;")?,
                            "dashed" => writeln!(buffer, "{indentation}border-style: dashed;")?,
                            "dotted" => writeln!(buffer, "{indentation}border-style: dotted;")?,
                            "double" => writeln!(buffer, "{indentation}border-style: double;")?,
                            "none" => writeln!(buffer, "{indentation}border-style: none;")?,
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
