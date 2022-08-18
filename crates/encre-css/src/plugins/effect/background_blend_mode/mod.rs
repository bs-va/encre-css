#![doc = include_str!("README.md")]
#![doc(alias("effect", "bg", "background"))]
use crate::{
    generator::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
    utils::indent,
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "bg-blend"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => [
                "normal",
                "multiply",
                "screen",
                "overlay",
                "darken",
                "lighten",
                "color-dodge",
                "color-burn",
                "hard-light",
                "soft-light",
                "difference",
                "exclusion",
                "hue",
                "saturation",
                "color",
                "luminosity",
            ]
            .contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Builtin { value, .. } => match *value {
                "normal" => writeln!(context.buffer, "background-blend-mode: normal;")?,
                "multiply" => writeln!(context.buffer, "background-blend-mode: multiply;")?,
                "screen" => writeln!(context.buffer, "background-blend-mode: screen;")?,
                "overlay" => writeln!(context.buffer, "background-blend-mode: overlay;")?,
                "darken" => writeln!(context.buffer, "background-blend-mode: darken;")?,
                "lighten" => writeln!(context.buffer, "background-blend-mode: lighten;")?,
                "color-dodge" => writeln!(context.buffer, "background-blend-mode: color-dodge;")?,
                "color-burn" => writeln!(context.buffer, "background-blend-mode: color-burn;")?,
                "hard-light" => writeln!(context.buffer, "background-blend-mode: hard-light;")?,
                "soft-light" => writeln!(context.buffer, "background-blend-mode: soft-light;")?,
                "difference" => writeln!(context.buffer, "background-blend-mode: difference;")?,
                "exclusion" => writeln!(context.buffer, "background-blend-mode: exclusion;")?,
                "hue" => writeln!(context.buffer, "background-blend-mode: hue;")?,
                "saturation" => writeln!(context.buffer, "background-blend-mode: saturation;")?,
                "color" => writeln!(context.buffer, "background-blend-mode: color;")?,
                "luminosity" => writeln!(context.buffer, "background-blend-mode: luminosity;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
