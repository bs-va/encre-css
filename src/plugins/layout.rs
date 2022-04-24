use super::Plugin;
use crate::utils::{default_lengths, value_matchers::*};

#[derive(Debug)]
pub struct PositionPlugin;

impl Plugin for PositionPlugin {
    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        if ["static", "fixed", "absolute", "relative", "sticky"].contains(&modifier) {
            Some(format!("position: {modifier};"))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct DisplayPlugin;

impl Plugin for DisplayPlugin {
    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        match modifier {
            "hidden" => Some("display: none;".to_string()),
            "contents" => Some("display: contents;".to_string()),
            "list-item" => Some("display: list-item;".to_string()),
            "block" => Some("display: block;".to_string()),
            "inline-block" => Some("display: inline-block;".to_string()),
            "flex" => Some("display: flex;".to_string()),
            "inline-flex" => Some("display: inline-flex;".to_string()),
            "inline" => Some("display: inline;".to_string()),
            "table" => Some("display: table;".to_string()),
            "inline-table" => Some("display: inline-table;".to_string()),
            "table-cell" => Some("display: table-cell;".to_string()),
            "table-caption" => Some("display: table-caption;".to_string()),
            "table-column" => Some("display: table-column;".to_string()),
            "table-column-group" => Some("display: table-column-group;".to_string()),
            "table-footer-group" => Some("display: table-footer-group;".to_string()),
            "table-header-group" => Some("display: table-header-group;".to_string()),
            "table-row-group" => Some("display: table-row-group;".to_string()),
            "table-row" => Some("display: table-row;".to_string()),
            "flow-root" => Some("display: flow-root;".to_string()),
            "grid" => Some("display: grid;".to_string()),
            "inline-grid" => Some("display: inline-grid;".to_string()),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct VisibilityPlugin;

impl Plugin for VisibilityPlugin {
    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        match modifier {
            "visible" => Some("visibility: visible;".to_string()),
            "invisible" => Some("visibility: hidden;".to_string()),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct IsolationPlugin;

impl Plugin for IsolationPlugin {
    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        match modifier {
            "isolate" => Some("isolation: isolate;".to_string()),
            "isolation-auto" => Some("isolation: auto;".to_string()),
            _ => None,
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

    fn css_template_value(&self, val: &str) -> String {
        format!(
            "top: {val};
  right: {val};
  bottom: {val};
  left: {val};"
        )
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        default_lengths::get_extended(modifier).map(|c| self.css_template_value(&c))
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

    fn css_template_value(&self, val: &str) -> String {
        format!(
            "left: {val};
  right: {val};"
        )
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        default_lengths::get_extended(modifier).map(|c| self.css_template_value(&c))
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

    fn css_template_value(&self, val: &str) -> String {
        format!(
            "top: {val};
  bottom: {val};"
        )
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        default_lengths::get_extended(modifier).map(|c| self.css_template_value(&c))
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

    fn css_template_value(&self, val: &str) -> String {
        format!("top: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        default_lengths::get_extended(modifier).map(|c| self.css_template_value(&c))
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

    fn css_template_value(&self, val: &str) -> String {
        format!("bottom: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        default_lengths::get_extended(modifier).map(|c| self.css_template_value(&c))
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

    fn css_template_value(&self, val: &str) -> String {
        format!("left: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        default_lengths::get_extended(modifier).map(|c| self.css_template_value(&c))
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

    fn css_template_value(&self, val: &str) -> String {
        format!("right: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        default_lengths::get_extended(modifier).map(|c| self.css_template_value(&c))
    }
}

#[derive(Debug)]
pub struct ZIndexPlugin;

impl Plugin for ZIndexPlugin {
    fn namespace(&self) -> &str {
        "z"
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        // NOTE: Not-compatible with TailwindCSS, support all values
        if modifier.parse::<f32>().is_ok() || is_matching_auto(modifier) {
            Some(format!("z-index: {modifier};"))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct ContainerPlugin;

impl Plugin for ContainerPlugin {
    fn namespace(&self) -> &str {
        "container"
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        match modifier {
            "none" => Some("width: 100%;".to_string()),
            "sm" => Some("max-width: 640px;".to_string()),
            "md" => Some("max-width: 768px;".to_string()),
            "lg" => Some("max-width: 1024px;".to_string()),
            "xl" => Some("max-width: 1280px;".to_string()),
            "2xl" => Some("max-width: 1536px;".to_string()),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct BoxDecorationBreakPlugin;

impl Plugin for BoxDecorationBreakPlugin {
    fn namespace(&self) -> &str {
        "decoration"
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        if ["slice", "clone"].contains(&modifier) {
            Some(format!("box-decoration-break: {modifier};"))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct BoxSizingPlugin;

impl Plugin for BoxSizingPlugin {
    fn namespace(&self) -> &str {
        "box"
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        if ["border", "content"].contains(&modifier) {
            Some(format!("box-sizing: {modifier}-box;"))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct FloatPlugin;

impl Plugin for FloatPlugin {
    fn namespace(&self) -> &str {
        "float"
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        if ["left", "right", "none"].contains(&modifier) {
            Some(format!("float: {modifier};"))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct ClearPlugin;

impl Plugin for ClearPlugin {
    fn namespace(&self) -> &str {
        "clear"
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        if ["left", "right", "both", "none"].contains(&modifier) {
            Some(format!("clear: {modifier};"))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct ObjectFitPlugin;

impl Plugin for ObjectFitPlugin {
    fn namespace(&self) -> &str {
        "object"
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        if ["contain", "cover", "fill", "none", "scale-down"].contains(&modifier) {
            Some(format!("object-fit: {modifier};"))
        } else {
            None
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

    fn css_template_value(&self, val: &str) -> String {
        format!("object-position: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        match modifier {
            "bottom" => Some(self.css_template_value("bottom")),
            "center" => Some(self.css_template_value("center")),
            "left" => Some(self.css_template_value("left")),
            "left-bottom" => Some(self.css_template_value("left bottom")),
            "left-top" => Some(self.css_template_value("left top")),
            "right" => Some(self.css_template_value("right")),
            "right-bottom" => Some(self.css_template_value("right bottom")),
            "right-top" => Some(self.css_template_value("right top")),
            "top" => Some(self.css_template_value("top")),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct OverflowPlugin;

impl Plugin for OverflowPlugin {
    fn namespace(&self) -> &str {
        "overflow"
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        match modifier {
            "auto" => Some("overflow: auto;".to_string()),
            "x-auto" => Some("overflow-x: auto;".to_string()),
            "y-auto" => Some("overflow-y: auto;".to_string()),
            "hidden" => Some("overflow: hidden;".to_string()),
            "x-hidden" => Some("overflow-x: hidden;".to_string()),
            "y-hidden" => Some("overflow-y: hidden;".to_string()),
            "visible" => Some("overflow: visible;".to_string()),
            "x-visible" => Some("overflow-x: visible;".to_string()),
            "y-visible" => Some("overflow-y: visible;".to_string()),
            "scroll" => Some("overflow: scroll;".to_string()),
            "x-scroll" => Some("overflow-x: scroll;".to_string()),
            "y-scroll" => Some("overflow-y: scroll;".to_string()),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct OverscrollPlugin;

impl Plugin for OverscrollPlugin {
    fn namespace(&self) -> &str {
        "overscroll"
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        match modifier {
            "auto" => Some("overscroll-behavior: auto;".to_string()),
            "y-auto" => Some("overscroll-behavior-y: auto;".to_string()),
            "x-auto" => Some("overscroll-behavior-x: auto;".to_string()),
            "contain" => Some("overscroll-behavior: contain;".to_string()),
            "y-contain" => Some("overscroll-behavior-y: contain;".to_string()),
            "x-contain" => Some("overscroll-behavior-x: contain;".to_string()),
            "none" => Some("overscroll-behavior: none;".to_string()),
            "y-none" => Some("overscroll-behavior-y: none;".to_string()),
            "x-none" => Some("overscroll-behavior-x: none;".to_string()),
            _ => None,
        }
    }
}
