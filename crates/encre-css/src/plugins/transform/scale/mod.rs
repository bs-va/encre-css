#![doc = include_str!("README.md")]
use super::CSS_TRANSFORM;
use crate::{
    context::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
    utils::{format_negative, indent},
};

use std::fmt::{self, Write};

fn scale_can_handle(context: &mut ContextCanHandle) -> bool {
    match context.modifier {
        Modifier::Builtin { value, .. } => value.parse::<usize>().is_ok(),
        Modifier::Arbitrary { .. } => false,
    }
}

fn scale_handle(css_properties: &[&str], context: &mut ContextHandle) -> fmt::Result {
    match context.modifier {
        Modifier::Builtin { is_negative, value } => {
            for css_prop in css_properties {
                indent(context.indentation, context.buffer)?;

                #[allow(clippy::cast_precision_loss)]
                writeln!(
                    context.buffer,
                    "{}: {}{};",
                    css_prop,
                    format_negative(is_negative),
                    value.parse::<usize>().unwrap() as f32 / 100.,
                )?;
            }
        }
        Modifier::Arbitrary { .. } => unreachable!(),
    }

    indent(context.indentation, context.buffer)?;
    writeln!(context.buffer, "{}", CSS_TRANSFORM)?;
    Ok(())
}

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &'static str {
        "scale"
    }

    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        scale_can_handle(&mut context)
    }

    fn handle(&self, mut context: ContextHandle) -> fmt::Result {
        scale_handle(&["--en-scale-x", "--en-scale-y"], &mut context)
    }
}

#[derive(Debug)]
pub(crate) struct PluginXDefinition;

impl Plugin for PluginXDefinition {
    fn namespace(&self) -> &'static str {
        "scale-x"
    }

    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        scale_can_handle(&mut context)
    }

    fn handle(&self, mut context: ContextHandle) -> fmt::Result {
        scale_handle(&["--en-scale-x"], &mut context)
    }
}

#[derive(Debug)]
pub(crate) struct PluginYDefinition;

impl Plugin for PluginYDefinition {
    fn namespace(&self) -> &'static str {
        "scale-y"
    }

    fn can_handle(&self, mut context: ContextCanHandle) -> bool {
        scale_can_handle(&mut context)
    }

    fn handle(&self, mut context: ContextHandle) -> fmt::Result {
        scale_handle(&["--en-scale-y"], &mut context)
    }
}
