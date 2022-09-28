#![doc = include_str!("README.md")]
#![doc(alias("effect", "bg", "background"))]
use crate::{
    generator::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
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

    fn handle(&self, ContextHandle { modifier, buffer, indentation, .. }: &mut ContextHandle) -> fmt::Result {
        match modifier {
            Modifier::Builtin { value, .. } => match *value {
                "normal" => writeln!(buffer, "{indentation}background-blend-mode: normal;")?,
                "multiply" => writeln!(buffer, "{indentation}background-blend-mode: multiply;")?,
                "screen" => writeln!(buffer, "{indentation}background-blend-mode: screen;")?,
                "overlay" => writeln!(buffer, "{indentation}background-blend-mode: overlay;")?,
                "darken" => writeln!(buffer, "{indentation}background-blend-mode: darken;")?,
                "lighten" => writeln!(buffer, "{indentation}background-blend-mode: lighten;")?,
                "color-dodge" => writeln!(buffer, "{indentation}background-blend-mode: color-dodge;")?,
                "color-burn" => writeln!(buffer, "{indentation}background-blend-mode: color-burn;")?,
                "hard-light" => writeln!(buffer, "{indentation}background-blend-mode: hard-light;")?,
                "soft-light" => writeln!(buffer, "{indentation}background-blend-mode: soft-light;")?,
                "difference" => writeln!(buffer, "{indentation}background-blend-mode: difference;")?,
                "exclusion" => writeln!(buffer, "{indentation}background-blend-mode: exclusion;")?,
                "hue" => writeln!(buffer, "{indentation}background-blend-mode: hue;")?,
                "saturation" => writeln!(buffer, "{indentation}background-blend-mode: saturation;")?,
                "color" => writeln!(buffer, "{indentation}background-blend-mode: color;")?,
                "luminosity" => writeln!(buffer, "{indentation}background-blend-mode: luminosity;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
