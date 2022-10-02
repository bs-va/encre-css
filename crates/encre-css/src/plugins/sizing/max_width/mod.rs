#![doc = include_str!("README.md")]
#![doc(alias("sizing", "size"))]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "max-w"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                [
                    "xs",
                    "sm",
                    "md",
                    "lg",
                    "xl",
                    "2xl",
                    "3xl",
                    "4xl",
                    "5xl",
                    "6xl",
                    "7xl",
                    "full",
                    "min",
                    "max",
                    "fit",
                    "prose",
                    "screen",
                    "screen-sm",
                    "screen-md",
                    "screen-lg",
                    "screen-lg",
                    "screen-xl",
                    "screen-2xl",
                    "none",
                ]
                .contains(&&**value)
                    || spacing::is_matching_builtin_spacing(value)
            }
            Modifier::Arbitrary { value, .. } => is_matching_length(value),
        }
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { is_negative, value } => match *value {
                "none" => writeln!(buffer, "{indentation}max-width: none;")?,
                "xs" => writeln!(buffer, "{indentation}max-width: 20rem;")?,
                "sm" => writeln!(buffer, "{indentation}max-width: 24rem;")?,
                "md" => writeln!(buffer, "{indentation}max-width: 28rem;")?,
                "lg" => writeln!(buffer, "{indentation}max-width: 32rem;")?,
                "xl" => writeln!(buffer, "{indentation}max-width: 36rem;")?,
                "2xl" => writeln!(buffer, "{indentation}max-width: 42rem;")?,
                "3xl" => writeln!(buffer, "{indentation}max-width: 48rem;")?,
                "4xl" => writeln!(buffer, "{indentation}max-width: 56rem;")?,
                "5xl" => writeln!(buffer, "{indentation}max-width: 64rem;")?,
                "6xl" => writeln!(buffer, "{indentation}max-width: 72rem;")?,
                "7xl" => writeln!(buffer, "{indentation}max-width: 80rem;")?,
                "full" => writeln!(buffer, "{indentation}max-width: 100%;")?,
                "min" => writeln!(buffer, "{indentation}max-width: min-content;")?,
                "max" => writeln!(buffer, "{indentation}max-width: max-content;")?,
                "fit" => writeln!(buffer, "{indentation}max-width: fit-content;")?,
                "prose" => writeln!(buffer, "{indentation}max-width: 65ch;")?,
                "screen" => writeln!(buffer, "{indentation}max-width: 100vw;")?,
                "screen-sm" => writeln!(buffer, "{indentation}max-width: 640px;")?,
                "screen-md" => writeln!(buffer, "{indentation}max-width: 768px;")?,
                "screen-lg" => writeln!(buffer, "{indentation}max-width: 1024px;")?,
                "screen-xl" => writeln!(buffer, "{indentation}max-width: 1280px;")?,
                "screen-2xl" => writeln!(buffer, "{indentation}max-width: 1536px;")?,
                _ => writeln!(
                    buffer,
                    "min-width: {};",
                    spacing::get(value, *is_negative).unwrap()
                )?,
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "{indentation}max-width: {value};")?;
            }
        }

        Ok(())
    }
}
