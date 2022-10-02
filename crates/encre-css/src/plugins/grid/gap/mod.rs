#![doc = include_str!("README.md")]
#![doc(alias("grid", "flexbox"))]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "gap"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => spacing::is_matching_builtin_spacing(value),
            Modifier::Arbitrary { prefix, value, .. } => {
                prefix.is_empty() && is_matching_length(value)
            }
        }
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { is_negative, value } => writeln!(
                buffer,
                "{indentation}gap: {};",
                spacing::get(value, *is_negative).unwrap()
            )?,
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "{indentation}gap: {value};")?;
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub(crate) struct PluginXDefinition;

impl Plugin for PluginXDefinition {
    fn namespace(&self) -> &str {
        "gap-x"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => spacing::is_matching_builtin_spacing(value),
            Modifier::Arbitrary { value, .. } => is_matching_length(value),
        }
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { is_negative, value } => writeln!(
                buffer,
                "{indentation}column-gap: {};",
                spacing::get(value, *is_negative).unwrap()
            )?,
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "{indentation}column-gap: {value};")?;
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub(crate) struct PluginYDefinition;

impl Plugin for PluginYDefinition {
    fn namespace(&self) -> &str {
        "gap-y"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => spacing::is_matching_builtin_spacing(value),
            Modifier::Arbitrary { value, .. } => is_matching_length(value),
        }
    }

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { is_negative, value } => writeln!(
                buffer,
                "{indentation}row-gap: {};",
                spacing::get(value, *is_negative).unwrap()
            )?,
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "{indentation}row-gap: {value};")?;
            }
        }

        Ok(())
    }
}
