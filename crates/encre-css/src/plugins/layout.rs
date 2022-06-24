use super::{to_css_value, Plugin};
use crate::utils::{indent, length, value_matchers::*};
use crate::{config::Config, selector::Modifier};

use std::fmt::{self, Write};

#[derive(Debug)]
pub struct PositionPlugin;

impl Plugin for PositionPlugin {
    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => {
                ["static", "fixed", "absolute", "relative", "sticky"].contains(&&**value)
            }
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
            Modifier::Basic { value, .. } => writeln!(buffer, "position: {value};")?,
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct DisplayPlugin;

impl Plugin for DisplayPlugin {
    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => [
                "hidden",
                "contents",
                "list-item",
                "block",
                "inline-block",
                "flex",
                "inline-flex",
                "inline",
                "table",
                "inline-table",
                "table-cell",
                "table-caption",
                "table-column",
                "table-column-group",
                "table-footer-group",
                "table-header-group",
                "table-row-group",
                "table-row",
                "flow-root",
                "grid",
                "inline-grid",
            ]
            .contains(&&**value),
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
                "hidden" => writeln!(buffer, "display: none;")?,
                "contents" => writeln!(buffer, "display: contents;")?,
                "list-item" => writeln!(buffer, "display: list-item;")?,
                "block" => writeln!(buffer, "display: block;")?,
                "inline-block" => writeln!(buffer, "display: inline-block;")?,
                "flex" => writeln!(buffer, "display: flex;")?,
                "inline-flex" => writeln!(buffer, "display: inline-flex;")?,
                "inline" => writeln!(buffer, "display: inline;")?,
                "table" => writeln!(buffer, "display: table;")?,
                "inline-table" => writeln!(buffer, "display: inline-table;")?,
                "table-cell" => writeln!(buffer, "display: table-cell;")?,
                "table-caption" => writeln!(buffer, "display: table-caption;")?,
                "table-column" => writeln!(buffer, "display: table-column;")?,
                "table-column-group" => writeln!(buffer, "display: table-column-group;")?,
                "table-footer-group" => writeln!(buffer, "display: table-footer-group;")?,
                "table-header-group" => writeln!(buffer, "display: table-header-group;")?,
                "table-row-group" => writeln!(buffer, "display: table-row-group;")?,
                "table-row" => writeln!(buffer, "display: table-row;")?,
                "flow-root" => writeln!(buffer, "display: flow-root;")?,
                "grid" => writeln!(buffer, "display: grid;")?,
                "inline-grid" => writeln!(buffer, "display: inline-grid;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct VisibilityPlugin;

impl Plugin for VisibilityPlugin {
    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => ["visible", "invisible"].contains(&&**value),
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
                "visible" => writeln!(buffer, "visibility: visible;")?,
                "invisible" => writeln!(buffer, "visibility: hidden;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct IsolationPlugin;

impl Plugin for IsolationPlugin {
    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => ["isolate", "isolation-auto"].contains(&&**value),
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
                "isolate" => writeln!(buffer, "isolation: isolate;")?,
                "isolation-auto" => writeln!(buffer, "isolation: auto;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

pub fn position_can_handle(modifier: &Modifier) -> bool {
    match modifier {
        Modifier::Basic { is_negative, value } => {
            length::get_extended(value, *is_negative).is_some()
        }
        Modifier::Arbitrary { value, .. } => {
            is_matching_length(value) || is_matching_percentage(value) || *value == "auto"
        }
    }
}

pub fn position_handle(
    css_properties: &[&str],
    modifier: &Modifier,
    indentation: usize,
    buffer: &mut String,
) -> fmt::Result {
    match modifier {
        Modifier::Basic { is_negative, value } => {
            for css_prop in css_properties {
                indent(indentation, buffer)?;
                writeln!(
                    buffer,
                    "{}: {};",
                    css_prop,
                    length::get_extended(value, *is_negative).unwrap(),
                )?
            }
        }
        Modifier::Arbitrary { value, .. } => {
            let value = to_css_value(value);
            for css_prop in css_properties {
                indent(indentation, buffer)?;
                writeln!(buffer, "{}: {};", css_prop, value)?;
            }
        }
    }

    Ok(())
}

#[derive(Debug)]
pub struct InsetPlugin;

impl Plugin for InsetPlugin {
    fn namespace(&self) -> &str {
        "inset"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        position_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        position_handle(
            &["top", "bottom", "left", "right"],
            modifier,
            indentation,
            buffer,
        )
    }
}

#[derive(Debug)]
pub struct InsetXPlugin;

impl Plugin for InsetXPlugin {
    fn namespace(&self) -> &str {
        "inset-x"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        position_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        position_handle(&["left", "right"], modifier, indentation, buffer)
    }
}

#[derive(Debug)]
pub struct InsetYPlugin;

impl Plugin for InsetYPlugin {
    fn namespace(&self) -> &str {
        "inset-y"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        position_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        position_handle(&["top", "bottom"], modifier, indentation, buffer)
    }
}

#[derive(Debug)]
pub struct TopPlugin;

impl Plugin for TopPlugin {
    fn namespace(&self) -> &str {
        "top"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        position_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        position_handle(&["top"], modifier, indentation, buffer)
    }
}

#[derive(Debug)]
pub struct BottomPlugin;

impl Plugin for BottomPlugin {
    fn namespace(&self) -> &str {
        "bottom"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        position_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        position_handle(&["bottom"], modifier, indentation, buffer)
    }
}

#[derive(Debug)]
pub struct LeftPlugin;

impl Plugin for LeftPlugin {
    fn namespace(&self) -> &str {
        "left"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        position_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        position_handle(&["left"], modifier, indentation, buffer)
    }
}

#[derive(Debug)]
pub struct RightPlugin;

impl Plugin for RightPlugin {
    fn namespace(&self) -> &str {
        "right"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        position_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        position_handle(&["right"], modifier, indentation, buffer)
    }
}

#[derive(Debug)]
pub struct ZIndexPlugin;

impl Plugin for ZIndexPlugin {
    fn namespace(&self) -> &str {
        "z"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => value.parse::<usize>().is_ok() || *value == "auto",
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
        // NOTE: Not-compatible with TailwindCSS, support all values
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { value, .. } => writeln!(buffer, "z-index: {value};")?,
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct ContainerPlugin;

impl Plugin for ContainerPlugin {
    fn namespace(&self) -> &str {
        "container"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => {
                ["sm", "md", "lg", "xl", "2xl", "none"].contains(&&**value)
            }
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
                "none" => writeln!(buffer, "width: 100%;")?,
                "sm" => writeln!(buffer, "max-width: 640px;")?,
                "md" => writeln!(buffer, "max-width: 768px;")?,
                "lg" => writeln!(buffer, "max-width: 1024px;")?,
                "xl" => writeln!(buffer, "max-width: 1280px;")?,
                "2xl" => writeln!(buffer, "max-width: 1536px;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct BoxDecorationBreakPlugin;

impl Plugin for BoxDecorationBreakPlugin {
    fn namespace(&self) -> &str {
        "decoration"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => ["slice", "clone"].contains(&&**value),
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
            Modifier::Basic { value, .. } => writeln!(buffer, "box-decoration-break: {value};")?,
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct BoxSizingPlugin;

impl Plugin for BoxSizingPlugin {
    fn namespace(&self) -> &str {
        "box"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => ["border", "content"].contains(&&**value),
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
            Modifier::Basic { value, .. } => writeln!(buffer, "box-sizing: {value}-box;")?,
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct FloatPlugin;

impl Plugin for FloatPlugin {
    fn namespace(&self) -> &str {
        "float"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => ["left", "right", "none"].contains(&&**value),
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
            Modifier::Basic { value, .. } => writeln!(buffer, "float: {value};")?,
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct ClearPlugin;

impl Plugin for ClearPlugin {
    fn namespace(&self) -> &str {
        "clear"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => ["left", "right", "both", "none"].contains(&&**value),
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
            Modifier::Basic { value, .. } => writeln!(buffer, "clear: {value};")?,
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct ObjectFitPlugin;

impl Plugin for ObjectFitPlugin {
    fn namespace(&self) -> &str {
        "object"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => {
                ["contain", "cover", "fill", "scale-down", "none"].contains(&&**value)
            }
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
            Modifier::Basic { value, .. } => writeln!(buffer, "object-fit: {value};")?,
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct ObjectPositionPlugin;

impl Plugin for ObjectPositionPlugin {
    fn namespace(&self) -> &str {
        "object"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => [
                "bottom",
                "center",
                "left",
                "left-bottom",
                "left-top",
                "right",
                "right-bottom",
                "right-top",
                "top",
            ]
            .contains(&&**value),
            Modifier::Arbitrary { value, .. } => value.split('_').all(is_matching_position),
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
                "bottom" => writeln!(buffer, "object-position: bottom;")?,
                "center" => writeln!(buffer, "object-position: center;")?,
                "left" => writeln!(buffer, "object-position: left;")?,
                "left-bottom" => writeln!(buffer, "object-position: left bottom;")?,
                "left-top" => writeln!(buffer, "object-position: left top;")?,
                "right" => writeln!(buffer, "object-position: right;")?,
                "right-bottom" => writeln!(buffer, "object-position: right bottom;")?,
                "right-top" => writeln!(buffer, "object-position: right top;")?,
                "top" => writeln!(buffer, "object-position: top;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "object-position: {};", to_css_value(value))?
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct OverflowPlugin;

impl Plugin for OverflowPlugin {
    fn namespace(&self) -> &str {
        "overflow"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => [
                "auto",
                "x-auto",
                "y-auto",
                "hidden",
                "x-hidden",
                "y-hidden",
                "visible",
                "x-visible",
                "y-visible",
                "scroll",
                "x-scroll",
                "y-scroll",
            ]
            .contains(&&**value),
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
                "auto" => writeln!(buffer, "overflow: auto;")?,
                "x-auto" => writeln!(buffer, "overflow-x: auto;")?,
                "y-auto" => writeln!(buffer, "overflow-y: auto;")?,
                "hidden" => writeln!(buffer, "overflow: hidden;")?,
                "x-hidden" => writeln!(buffer, "overflow-x: hidden;")?,
                "y-hidden" => writeln!(buffer, "overflow-y: hidden;")?,
                "visible" => writeln!(buffer, "overflow: visible;")?,
                "x-visible" => writeln!(buffer, "overflow-x: visible;")?,
                "y-visible" => writeln!(buffer, "overflow-y: visible;")?,
                "scroll" => writeln!(buffer, "overflow: scroll;")?,
                "x-scroll" => writeln!(buffer, "overflow-x: scroll;")?,
                "y-scroll" => writeln!(buffer, "overflow-y: scroll;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct OverscrollPlugin;

impl Plugin for OverscrollPlugin {
    fn namespace(&self) -> &str {
        "overscroll"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => [
                "auto",
                "x-auto",
                "y-auto",
                "contain",
                "x-contain",
                "y-contain",
                "none",
                "x-none",
                "y-none",
            ]
            .contains(&&**value),
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
                "auto" => writeln!(buffer, "overscroll-behavior: auto;")?,
                "x-auto" => writeln!(buffer, "overscroll-behavior-x: auto;")?,
                "y-auto" => writeln!(buffer, "overscroll-behavior-y: auto;")?,
                "contain" => writeln!(buffer, "overscroll-behavior: contain;")?,
                "x-contain" => writeln!(buffer, "overscroll-behavior-x: contain;")?,
                "y-contain" => writeln!(buffer, "overscroll-behavior-y: contain;")?,
                "none" => writeln!(buffer, "overscroll-behavior: none;")?,
                "x-none" => writeln!(buffer, "overscroll-behavior-x: none;")?,
                "y-none" => writeln!(buffer, "overscroll-behavior-y: none;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
