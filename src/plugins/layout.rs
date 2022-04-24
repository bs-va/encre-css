use super::Plugin;
use crate::utils::{default_lengths, value_matchers::*};

use std::fmt::{Result, Write};

#[derive(Debug)]
pub struct PositionPlugin;

impl Plugin for PositionPlugin {
    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if ["static", "fixed", "absolute", "relative", "sticky"].contains(&modifier) {
            write!(css_content, "position: {modifier};")
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct DisplayPlugin;

impl Plugin for DisplayPlugin {
    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        match modifier {
            "hidden" => write!(css_content, "display: none;"),
            "contents" => write!(css_content, "display: contents;"),
            "list-item" => write!(css_content, "display: list-item;"),
            "block" => write!(css_content, "display: block;"),
            "inline-block" => write!(css_content, "display: inline-block;"),
            "flex" => write!(css_content, "display: flex;"),
            "inline-flex" => write!(css_content, "display: inline-flex;"),
            "inline" => write!(css_content, "display: inline;"),
            "table" => write!(css_content, "display: table;"),
            "inline-table" => write!(css_content, "display: inline-table;"),
            "table-cell" => write!(css_content, "display: table-cell;"),
            "table-caption" => write!(css_content, "display: table-caption;"),
            "table-column" => write!(css_content, "display: table-column;"),
            "table-column-group" => write!(css_content, "display: table-column-group;"),
            "table-footer-group" => write!(css_content, "display: table-footer-group;"),
            "table-header-group" => write!(css_content, "display: table-header-group;"),
            "table-row-group" => write!(css_content, "display: table-row-group;"),
            "table-row" => write!(css_content, "display: table-row;"),
            "flow-root" => write!(css_content, "display: flow-root;"),
            "grid" => write!(css_content, "display: grid;"),
            "inline-grid" => write!(css_content, "display: inline-grid;"),
            _ => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct VisibilityPlugin;

impl Plugin for VisibilityPlugin {
    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        match modifier {
            "visible" => write!(css_content, "visibility: visible;"),
            "invisible" => write!(css_content, "visibility: hidden;"),
            _ => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct IsolationPlugin;

impl Plugin for IsolationPlugin {
    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        match modifier {
            "isolate" => write!(css_content, "isolation: isolate;"),
            "isolation-auto" => write!(css_content, "isolation: auto;"),
            _ => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct InsetPlugin;

impl Plugin for InsetPlugin {
    fn namespace(&self) -> &str {
        "inset"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        is_matching_length(val) || is_matching_percentage(val) || is_matching_auto(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content,
            "top: {val};
  right: {val};
  bottom: {val};
  left: {val};"
        )
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if let Some(length) = default_lengths::get_extended(modifier) {
            self.css_template_value(&length, css_content)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct InsetXPlugin;

impl Plugin for InsetXPlugin {
    fn namespace(&self) -> &str {
        "inset-x"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        is_matching_length(val) || is_matching_percentage(val) || is_matching_auto(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content,
            "left: {val};
  right: {val};"
        )
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if let Some(length) = default_lengths::get_extended(modifier) {
            self.css_template_value(&length, css_content)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct InsetYPlugin;

impl Plugin for InsetYPlugin {
    fn namespace(&self) -> &str {
        "inset-y"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        is_matching_length(val) || is_matching_percentage(val) || is_matching_auto(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content,
            "top: {val};
  bottom: {val};"
        )
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if let Some(length) = default_lengths::get_extended(modifier) {
            self.css_template_value(&length, css_content)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct TopPlugin;

impl Plugin for TopPlugin {
    fn namespace(&self) -> &str {
        "top"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        is_matching_length(val) || is_matching_percentage(val) || is_matching_auto(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "top: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if let Some(length) = default_lengths::get_extended(modifier) {
            self.css_template_value(&length, css_content)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct BottomPlugin;

impl Plugin for BottomPlugin {
    fn namespace(&self) -> &str {
        "bottom"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        is_matching_length(val) || is_matching_percentage(val) || is_matching_auto(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "bottom: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if let Some(length) = default_lengths::get_extended(modifier) {
            self.css_template_value(&length, css_content)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct LeftPlugin;

impl Plugin for LeftPlugin {
    fn namespace(&self) -> &str {
        "left"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        is_matching_length(val) || is_matching_percentage(val) || is_matching_auto(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "left: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if let Some(length) = default_lengths::get_extended(modifier) {
            self.css_template_value(&length, css_content)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct RightPlugin;

impl Plugin for RightPlugin {
    fn namespace(&self) -> &str {
        "right"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        is_matching_length(val) || is_matching_percentage(val) || is_matching_auto(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "right: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if let Some(length) = default_lengths::get_extended(modifier) {
            self.css_template_value(&length, css_content)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct ZIndexPlugin;

impl Plugin for ZIndexPlugin {
    fn namespace(&self) -> &str {
        "z"
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        // NOTE: Not-compatible with TailwindCSS, support all values
        if modifier.parse::<f32>().is_ok() || is_matching_auto(modifier) {
            write!(css_content, "z-index: {modifier};")
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct ContainerPlugin;

impl Plugin for ContainerPlugin {
    fn namespace(&self) -> &str {
        "container"
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        match modifier {
            "none" => write!(css_content, "width: 100%;"),
            "sm" => write!(css_content, "max-width: 640px;"),
            "md" => write!(css_content, "max-width: 768px;"),
            "lg" => write!(css_content, "max-width: 1024px;"),
            "xl" => write!(css_content, "max-width: 1280px;"),
            "2xl" => write!(css_content, "max-width: 1536px;"),
            _ => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct BoxDecorationBreakPlugin;

impl Plugin for BoxDecorationBreakPlugin {
    fn namespace(&self) -> &str {
        "decoration"
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if ["slice", "clone"].contains(&modifier) {
            write!(css_content, "box-decoration-break: {modifier};")
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct BoxSizingPlugin;

impl Plugin for BoxSizingPlugin {
    fn namespace(&self) -> &str {
        "box"
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if ["border", "content"].contains(&modifier) {
            write!(css_content, "box-sizing: {modifier}-box;")
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct FloatPlugin;

impl Plugin for FloatPlugin {
    fn namespace(&self) -> &str {
        "float"
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if ["left", "right", "none"].contains(&modifier) {
            write!(css_content, "float: {modifier};")
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct ClearPlugin;

impl Plugin for ClearPlugin {
    fn namespace(&self) -> &str {
        "clear"
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if ["left", "right", "both", "none"].contains(&modifier) {
            write!(css_content, "clear: {modifier};")
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct ObjectFitPlugin;

impl Plugin for ObjectFitPlugin {
    fn namespace(&self) -> &str {
        "object"
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if ["contain", "cover", "fill", "none", "scale-down"].contains(&modifier) {
            write!(css_content, "object-fit: {modifier};")
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct ObjectPositionPlugin;

impl Plugin for ObjectPositionPlugin {
    fn namespace(&self) -> &str {
        "object"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        val.split('_').all(is_matching_position)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "object-position: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        match modifier {
            "bottom" => self.css_template_value("bottom", css_content),
            "center" => self.css_template_value("center", css_content),
            "left" => self.css_template_value("left", css_content),
            "left-bottom" => self.css_template_value("left bottom", css_content),
            "left-top" => self.css_template_value("left top", css_content),
            "right" => self.css_template_value("right", css_content),
            "right-bottom" => self.css_template_value("right bottom", css_content),
            "right-top" => self.css_template_value("right top", css_content),
            "top" => self.css_template_value("top", css_content),
            _ => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct OverflowPlugin;

impl Plugin for OverflowPlugin {
    fn namespace(&self) -> &str {
        "overflow"
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        match modifier {
            "auto" => write!(css_content, "overflow: auto;"),
            "x-auto" => write!(css_content, "overflow-x: auto;"),
            "y-auto" => write!(css_content, "overflow-y: auto;"),
            "hidden" => write!(css_content, "overflow: hidden;"),
            "x-hidden" => write!(css_content, "overflow-x: hidden;"),
            "y-hidden" => write!(css_content, "overflow-y: hidden;"),
            "visible" => write!(css_content, "overflow: visible;"),
            "x-visible" => write!(css_content, "overflow-x: visible;"),
            "y-visible" => write!(css_content, "overflow-y: visible;"),
            "scroll" => write!(css_content, "overflow: scroll;"),
            "x-scroll" => write!(css_content, "overflow-x: scroll;"),
            "y-scroll" => write!(css_content, "overflow-y: scroll;"),
            _ => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct OverscrollPlugin;

impl Plugin for OverscrollPlugin {
    fn namespace(&self) -> &str {
        "overscroll"
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        match modifier {
            "auto" => write!(css_content, "overscroll-behavior: auto;"),
            "y-auto" => write!(css_content, "overscroll-behavior-y: auto;"),
            "x-auto" => write!(css_content, "overscroll-behavior-x: auto;"),
            "contain" => write!(css_content, "overscroll-behavior: contain;"),
            "y-contain" => write!(css_content, "overscroll-behavior-y: contain;"),
            "x-contain" => write!(css_content, "overscroll-behavior-x: contain;"),
            "none" => write!(css_content, "overscroll-behavior: none;"),
            "y-none" => write!(css_content, "overscroll-behavior-y: none;"),
            "x-none" => write!(css_content, "overscroll-behavior-x: none;"),
            _ => Ok(()),
        }
    }
}
