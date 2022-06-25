use super::{to_css_value, Plugin};
use crate::{
    config::Config,
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

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => ["collapse", "separate"].contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { value, .. } => match *value {
                "collapse" => writeln!(buffer, "border-collapse: collapse;")?,
                "separate" => writeln!(buffer, "border-collapse: separate;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

pub fn border_spacing_can_handle(modifier: &Modifier) -> bool {
    match modifier {
        Modifier::Basic { is_negative, value } => {
            length::get_extended(value, *is_negative).is_some()
        }
        Modifier::Arbitrary { value, .. } => is_matching_length(value),
    }
}

pub fn border_spacing_handle(
    css_props: &[&str],
    modifier: &Modifier,
    indentation: usize,
    buffer: &mut String,
) -> fmt::Result {
    match modifier {
        Modifier::Basic { is_negative, value } => {
            for css_prop in css_props {
                indent(indentation, buffer)?;
                writeln!(
                    buffer,
                    "{}: {};",
                    css_prop,
                    length::get_extended(value, *is_negative).unwrap()
                )?;
            }
        }
        Modifier::Arbitrary { value, .. } => {
            for css_prop in css_props {
                indent(indentation, buffer)?;
                writeln!(buffer, "{}: {};", css_prop, to_css_value(value))?
            }
        }
    }

    indent(indentation, buffer)?;
    writeln!(
        buffer,
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

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        border_spacing_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        border_spacing_handle(
            &["--en-border-spacing-x", "--en-border-spacing-y"],
            modifier,
            indentation,
            buffer,
        )
    }
}

#[derive(Debug)]
pub struct BorderSpacingXPlugin;

impl Plugin for BorderSpacingXPlugin {
    fn namespace(&self) -> &'static str {
        "border-spacing-x"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        border_spacing_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        border_spacing_handle(&["--en-border-spacing-x"], modifier, indentation, buffer)
    }
}

#[derive(Debug)]
pub struct BorderSpacingYPlugin;

impl Plugin for BorderSpacingYPlugin {
    fn namespace(&self) -> &'static str {
        "border-spacing-y"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        border_spacing_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        border_spacing_handle(&["--en-border-spacing-y"], modifier, indentation, buffer)
    }
}

#[derive(Debug)]
pub struct TableLayoutPlugin;

impl Plugin for TableLayoutPlugin {
    fn namespace(&self) -> &str {
        "table"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => ["auto", "fixed"].contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { value, .. } => match *value {
                "auto" => writeln!(buffer, "table-layout: auto;")?,
                "fixed" => writeln!(buffer, "table-layout: fixed;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
