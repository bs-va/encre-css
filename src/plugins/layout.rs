use super::Plugin;
use crate::utils::{default_lengths, value_matchers::*};

#[derive(Debug)]
pub struct LayoutPositionPlugin;

impl Plugin for LayoutPositionPlugin {
    fn namespace(&self) -> String {
        "".to_string()
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        if ["static", "fixed", "absolute", "relative", "sticky"].contains(&modifier) {
            Some(format!("position: {modifier};"))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct LayoutDisplayPlugin;

impl Plugin for LayoutDisplayPlugin {
    fn namespace(&self) -> String {
        "".to_string()
    }

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
pub struct LayoutVisibilityPlugin;

impl Plugin for LayoutVisibilityPlugin {
    fn namespace(&self) -> String {
        "".to_string()
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        match modifier {
            "visible" => Some("visibility: visible;".to_string()),
            "invisible" => Some("visibility: hidden;".to_string()),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct LayoutIsolationPlugin;

impl Plugin for LayoutIsolationPlugin {
    fn namespace(&self) -> String {
        "".to_string()
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        match modifier {
            "isolate" => Some("isolation: isolate;".to_string()),
            "isolation-auto" => Some("isolation: auto;".to_string()),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct LayoutInsetPlugin;

impl Plugin for LayoutInsetPlugin {
    fn namespace(&self) -> String {
        "inset".to_string()
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
pub struct LayoutInsetXPlugin;

impl Plugin for LayoutInsetXPlugin {
    fn namespace(&self) -> String {
        "inset-x".to_string()
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
pub struct LayoutInsetYPlugin;

impl Plugin for LayoutInsetYPlugin {
    fn namespace(&self) -> String {
        "inset-y".to_string()
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
pub struct LayoutTopPlugin;

impl Plugin for LayoutTopPlugin {
    fn namespace(&self) -> String {
        "top".to_string()
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
pub struct LayoutBottomPlugin;

impl Plugin for LayoutBottomPlugin {
    fn namespace(&self) -> String {
        "bottom".to_string()
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
pub struct LayoutLeftPlugin;

impl Plugin for LayoutLeftPlugin {
    fn namespace(&self) -> String {
        "left".to_string()
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
pub struct LayoutRightPlugin;

impl Plugin for LayoutRightPlugin {
    fn namespace(&self) -> String {
        "right".to_string()
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
pub struct LayoutZIndexPlugin;

impl Plugin for LayoutZIndexPlugin {
    fn namespace(&self) -> String {
        "z".to_string()
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        // NOTE: Not-compatible with TailwindCSS, support all values
        if modifier.parse::<usize>().is_ok() || is_matching_auto(modifier) {
            Some(format!("z-index: {modifier};"))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct LayoutContainerPlugin;

impl Plugin for LayoutContainerPlugin {
    fn namespace(&self) -> String {
        "container".to_string()
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
pub struct LayoutBoxDecorationBreakPlugin;

impl Plugin for LayoutBoxDecorationBreakPlugin {
    fn namespace(&self) -> String {
        "decoration".to_string()
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
pub struct LayoutBoxSizingPlugin;

impl Plugin for LayoutBoxSizingPlugin {
    fn namespace(&self) -> String {
        "box".to_string()
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
pub struct LayoutFloatPlugin;

impl Plugin for LayoutFloatPlugin {
    fn namespace(&self) -> String {
        "float".to_string()
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
pub struct LayoutClearPlugin;

impl Plugin for LayoutClearPlugin {
    fn namespace(&self) -> String {
        "clear".to_string()
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
pub struct LayoutObjectFitPlugin;

impl Plugin for LayoutObjectFitPlugin {
    fn namespace(&self) -> String {
        "object".to_string()
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
pub struct LayoutObjectPositionPlugin;

impl Plugin for LayoutObjectPositionPlugin {
    fn namespace(&self) -> String {
        "object".to_string()
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
pub struct LayoutOverflowPlugin;

impl Plugin for LayoutOverflowPlugin {
    fn namespace(&self) -> String {
        "overflow".to_string()
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
pub struct LayoutOverscrollPlugin;

impl Plugin for LayoutOverscrollPlugin {
    fn namespace(&self) -> String {
        "overscroll".to_string()
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
