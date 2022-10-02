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
                color::is_matching_builtin_color(context.config, value)
            }
            Modifier::Arbitrary { hint, value, .. } => {
                *hint == "color" || (hint.is_empty() && is_matching_color(value))
            }
        }
    }

    fn needs_wrapping(&self) -> bool {
        false
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        generate_at_rules(context, |context| {
            generate_class(
                context,
                |ContextHandle { modifier, indentation, buffer, config, .. }: &mut ContextHandle| {
                    match modifier {
                        Modifier::Builtin { value, .. } => {
                            let color =
                                color::get(config, value, Some("--en-divide-opacity"))
                                    .unwrap();
                            if color.contains("--en-divide-opacity") {
                                writeln!(buffer, "{indentation}--en-divide-opacity: 1;")?;
                            }

                            writeln!(buffer, "{indentation}border-color: {color};")?;
                        }
                        Modifier::Arbitrary { value, .. } => {
                            writeln!(buffer, "{indentation}border-color: {value};")?;
                        }
                    }

                    Ok(())
                },
                " > :not([hidden]) ~ :not([hidden])",
            )
        })
    }
}
