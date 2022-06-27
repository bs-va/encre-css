use super::{to_css_value, Plugin};
use crate::{
    context::{ContextCanHandle, ContextHandle},
    selector::Modifier,
    utils::{indent, length, value_matchers::*},
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub struct BorderCollapsePlugin;

impl Plugin for BorderCollapsePlugin {
    fn namespace(&self) -> &str {
        "border"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => ["collapse", "separate"].contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "collapse" => writeln!(context.buffer, "border-collapse: collapse;")?,
                "separate" => writeln!(context.buffer, "border-collapse: separate;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

pub fn border_spacing_can_handle(context: ContextCanHandle) -> bool {
    match context.modifier {
        Modifier::Basic { is_negative, value } => {
            length::get_extended(value, *is_negative).is_some()
        }
        Modifier::Arbitrary { value, .. } => is_matching_length(value),
    }
}

pub fn border_spacing_handle(css_props: &[&str], context: ContextHandle) -> fmt::Result {
    match context.modifier {
        Modifier::Basic { is_negative, value } => {
            for css_prop in css_props {
                indent(context.indentation, context.buffer)?;
                writeln!(
                    context.buffer,
                    "{}: {};",
                    css_prop,
                    length::get_extended(value, *is_negative).unwrap()
                )?;
            }
        }
        Modifier::Arbitrary { value, .. } => {
            for css_prop in css_props {
                indent(context.indentation, context.buffer)?;
                writeln!(context.buffer, "{}: {};", css_prop, to_css_value(value))?
            }
        }
    }

    indent(context.indentation, context.buffer)?;
    writeln!(
        context.buffer,
        "border-spacing: var(--en-border-spacing-x) var(--en-border-spacing-y);"
    )?;
    Ok(())
}

#[derive(Debug)]
pub struct BorderSpacingPlugin;

impl Plugin for BorderSpacingPlugin {
    fn namespace(&self) -> &'static str {
        "border-spacing"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        border_spacing_can_handle(context)
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        border_spacing_handle(&["--en-border-spacing-x", "--en-border-spacing-y"], context)
    }
}

#[derive(Debug)]
pub struct BorderSpacingXPlugin;

impl Plugin for BorderSpacingXPlugin {
    fn namespace(&self) -> &'static str {
        "border-spacing-x"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        border_spacing_can_handle(context)
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        border_spacing_handle(&["--en-border-spacing-x"], context)
    }
}

#[derive(Debug)]
pub struct BorderSpacingYPlugin;

impl Plugin for BorderSpacingYPlugin {
    fn namespace(&self) -> &'static str {
        "border-spacing-y"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        border_spacing_can_handle(context)
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        border_spacing_handle(&["--en-border-spacing-y"], context)
    }
}

#[derive(Debug)]
pub struct TableLayoutPlugin;

impl Plugin for TableLayoutPlugin {
    fn namespace(&self) -> &str {
        "table"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => ["auto", "fixed"].contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "auto" => writeln!(context.buffer, "table-layout: auto;")?,
                "fixed" => writeln!(context.buffer, "table-layout: fixed;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
