#![doc = include_str!("README.md")]
#![doc(alias = "layout")]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &'static str {
        "columns"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                value.parse::<usize>().map_or(false, |v| v <= 12)
                    || [
                        "auto", "3xs", "2xs", "xs", "sm", "md", "lg", "xl", "2xl", "3xl", "4xl",
                        "5xl", "6xl", "7xl",
                    ]
                    .contains(value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { value, .. } => match *value {
                "3xs" => writeln!(buffer, "{indentation}columns: 16rem;")?,
                "2xs" => writeln!(buffer, "{indentation}columns: 18rem;")?,
                "xs" => writeln!(buffer, "{indentation}columns: 20rem;")?,
                "sm" => writeln!(buffer, "{indentation}columns: 24rem;")?,
                "md" => writeln!(buffer, "{indentation}columns: 28rem;")?,
                "lg" => writeln!(buffer, "{indentation}columns: 32rem;")?,
                "xl" => writeln!(buffer, "{indentation}columns: 36rem;")?,
                "2xl" => writeln!(buffer, "{indentation}columns: 42rem;")?,
                "3xl" => writeln!(buffer, "{indentation}columns: 48rem;")?,
                "4xl" => writeln!(buffer, "{indentation}columns: 56rem;")?,
                "5xl" => writeln!(buffer, "{indentation}columns: 64rem;")?,
                "6xl" => writeln!(buffer, "{indentation}columns: 72rem;")?,
                "7xl" => writeln!(buffer, "{indentation}columns: 80rem;")?,
                _ => writeln!(buffer, "{indentation}columns: {value};")?,
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
