use super::Plugin;
use crate::{config::Config, selector::Modifier};
use crate::utils::{default_lengths, value_matchers::*};

use std::fmt::Write;

#[derive(Debug)]
pub struct PositionPlugin;

impl Plugin for PositionPlugin {
    fn get_css_for_modifier(&self, _config: &Config, modifier: &Modifier, css_content: &mut String, _custom_css: &mut String) -> bool {
        if modifier.is_one_of(&["static", "fixed", "absolute", "relative", "sticky"]) {
            write!(css_content, "position: {modifier};").is_ok()
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct DisplayPlugin;

impl Plugin for DisplayPlugin {
    fn get_css_for_modifier(&self, _config: &Config, modifier: &Modifier, css_content: &mut String, _custom_css: &mut String) -> bool {
        match modifier.content() {
            "hidden" => write!(css_content, "display: none;").is_ok(),
            "contents" => write!(css_content, "display: contents;").is_ok(),
            "list-item" => write!(css_content, "display: list-item;").is_ok(),
            "block" => write!(css_content, "display: block;").is_ok(),
            "inline-block" => write!(css_content, "display: inline-block;").is_ok(),
            "flex" => write!(css_content, "display: flex;").is_ok(),
            "inline-flex" => write!(css_content, "display: inline-flex;").is_ok(),
            "inline" => write!(css_content, "display: inline;").is_ok(),
            "table" => write!(css_content, "display: table;").is_ok(),
            "inline-table" => write!(css_content, "display: inline-table;").is_ok(),
            "table-cell" => write!(css_content, "display: table-cell;").is_ok(),
            "table-caption" => write!(css_content, "display: table-caption;").is_ok(),
            "table-column" => write!(css_content, "display: table-column;").is_ok(),
            "table-column-group" => write!(css_content, "display: table-column-group;").is_ok(),
            "table-footer-group" => write!(css_content, "display: table-footer-group;").is_ok(),
            "table-header-group" => write!(css_content, "display: table-header-group;").is_ok(),
            "table-row-group" => write!(css_content, "display: table-row-group;").is_ok(),
            "table-row" => write!(css_content, "display: table-row;").is_ok(),
            "flow-root" => write!(css_content, "display: flow-root;").is_ok(),
            "grid" => write!(css_content, "display: grid;").is_ok(),
            "inline-grid" => write!(css_content, "display: inline-grid;").is_ok(),
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct VisibilityPlugin;

impl Plugin for VisibilityPlugin {
    fn get_css_for_modifier(&self, _config: &Config, modifier: &Modifier, css_content: &mut String, _custom_css: &mut String) -> bool {
        match modifier.content() {
            "visible" => write!(css_content, "visibility: visible;").is_ok(),
            "invisible" => write!(css_content, "visibility: hidden;").is_ok(),
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct IsolationPlugin;

impl Plugin for IsolationPlugin {
    fn get_css_for_modifier(&self, _config: &Config, modifier: &Modifier, css_content: &mut String, _custom_css: &mut String) -> bool {
        match modifier.content() {
            "isolate" => write!(css_content, "isolation: isolate;").is_ok(),
            "isolation-auto" => write!(css_content, "isolation: auto;").is_ok(),
            _ => false,
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
        is_matching_length(val) || is_matching_percentage(val) || val == "auto"
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(
            css_content,
            "top: {val};
right: {val};
bottom: {val};
left: {val};"
        )
        .is_ok()
    }

    fn get_css_for_modifier(&self, _config: &Config, modifier: &Modifier, css_content: &mut String, _custom_css: &mut String) -> bool {
        if let Some(length) = default_lengths::get_extended(modifier.content(), modifier.is_negative()) {
            self.css_template_value(&length, css_content)
        } else {
            false
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
        is_matching_length(val) || is_matching_percentage(val) || val == "auto"
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(
            css_content,
            "left: {val};
right: {val};"
        )
        .is_ok()
    }

    fn get_css_for_modifier(&self, _config: &Config, modifier: &Modifier, css_content: &mut String, _custom_css: &mut String) -> bool {
        if let Some(length) = default_lengths::get_extended(modifier.content(), modifier.is_negative()) {
            self.css_template_value(&length, css_content)
        } else {
            false
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
        is_matching_length(val) || is_matching_percentage(val) || val == "auto"
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(
            css_content,
            "top: {val};
bottom: {val};"
        )
        .is_ok()
    }

    fn get_css_for_modifier(&self, _config: &Config, modifier: &Modifier, css_content: &mut String, _custom_css: &mut String) -> bool {
        if let Some(length) = default_lengths::get_extended(modifier.content(), modifier.is_negative()) {
            self.css_template_value(&length, css_content)
        } else {
            false
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
        is_matching_length(val) || is_matching_percentage(val) || val == "auto"
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "top: {val};").is_ok()
    }

    fn get_css_for_modifier(&self, _config: &Config, modifier: &Modifier, css_content: &mut String, _custom_css: &mut String) -> bool {
        if let Some(length) = default_lengths::get_extended(modifier.content(), modifier.is_negative()) {
            self.css_template_value(&length, css_content)
        } else {
            false
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
        is_matching_length(val) || is_matching_percentage(val) || val == "auto"
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "bottom: {val};").is_ok()
    }

    fn get_css_for_modifier(&self, _config: &Config, modifier: &Modifier, css_content: &mut String, _custom_css: &mut String) -> bool {
        if let Some(length) = default_lengths::get_extended(modifier.content(), modifier.is_negative()) {
            self.css_template_value(&length, css_content)
        } else {
            false
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
        is_matching_length(val) || is_matching_percentage(val) || val == "auto"
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "left: {val};").is_ok()
    }

    fn get_css_for_modifier(&self, _config: &Config, modifier: &Modifier, css_content: &mut String, _custom_css: &mut String) -> bool {
        if let Some(length) = default_lengths::get_extended(modifier.content(), modifier.is_negative()) {
            self.css_template_value(&length, css_content)
        } else {
            false
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
        is_matching_length(val) || is_matching_percentage(val) || val == "auto"
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "right: {val};").is_ok()
    }

    fn get_css_for_modifier(&self, _config: &Config, modifier: &Modifier, css_content: &mut String, _custom_css: &mut String) -> bool {
        if let Some(length) = default_lengths::get_extended(modifier.content(), modifier.is_negative()) {
            self.css_template_value(&length, css_content)
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct ZIndexPlugin;

impl Plugin for ZIndexPlugin {
    fn namespace(&self) -> &str {
        "z"
    }

    fn get_css_for_modifier(&self, _config: &Config, modifier: &Modifier, css_content: &mut String, _custom_css: &mut String) -> bool {
        // NOTE: Not-compatible with TailwindCSS, support all values
        if modifier.to_f32().is_ok() || modifier.is("auto") {
            write!(css_content, "z-index: {modifier};").is_ok()
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct ContainerPlugin;

impl Plugin for ContainerPlugin {
    fn namespace(&self) -> &str {
        "container"
    }

    fn get_css_for_modifier(&self, _config: &Config, modifier: &Modifier, css_content: &mut String, _custom_css: &mut String) -> bool {
        match modifier.content() {
            "none" => write!(css_content, "width: 100%;").is_ok(),
            "sm" => write!(css_content, "max-width: 640px;").is_ok(),
            "md" => write!(css_content, "max-width: 768px;").is_ok(),
            "lg" => write!(css_content, "max-width: 1024px;").is_ok(),
            "xl" => write!(css_content, "max-width: 1280px;").is_ok(),
            "2xl" => write!(css_content, "max-width: 1536px;").is_ok(),
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct BoxDecorationBreakPlugin;

impl Plugin for BoxDecorationBreakPlugin {
    fn namespace(&self) -> &str {
        "decoration"
    }

    fn get_css_for_modifier(&self, _config: &Config, modifier: &Modifier, css_content: &mut String, _custom_css: &mut String) -> bool {
        if modifier.is_one_of(&["slice", "clone"]) {
            write!(css_content, "box-decoration-break: {modifier};").is_ok()
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct BoxSizingPlugin;

impl Plugin for BoxSizingPlugin {
    fn namespace(&self) -> &str {
        "box"
    }

    fn get_css_for_modifier(&self, _config: &Config, modifier: &Modifier, css_content: &mut String, _custom_css: &mut String) -> bool {
        if modifier.is_one_of(&["border", "content"]) {
            write!(css_content, "box-sizing: {modifier}-box;").is_ok()
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct FloatPlugin;

impl Plugin for FloatPlugin {
    fn namespace(&self) -> &str {
        "float"
    }

    fn get_css_for_modifier(&self, _config: &Config, modifier: &Modifier, css_content: &mut String, _custom_css: &mut String) -> bool {
        if modifier.is_one_of(&["left", "right", "none"]) {
            write!(css_content, "float: {modifier};").is_ok()
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct ClearPlugin;

impl Plugin for ClearPlugin {
    fn namespace(&self) -> &str {
        "clear"
    }

    fn get_css_for_modifier(&self, _config: &Config, modifier: &Modifier, css_content: &mut String, _custom_css: &mut String) -> bool {
        if modifier.is_one_of(&["left", "right", "both", "none"]) {
            write!(css_content, "clear: {modifier};").is_ok()
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct ObjectFitPlugin;

impl Plugin for ObjectFitPlugin {
    fn namespace(&self) -> &str {
        "object"
    }

    fn get_css_for_modifier(&self, _config: &Config, modifier: &Modifier, css_content: &mut String, _custom_css: &mut String) -> bool {
        if modifier.is_one_of(&["contain", "cover", "fill", "none", "scale-down"]) {
            write!(css_content, "object-fit: {modifier};").is_ok()
        } else {
            false
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

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "object-position: {val};").is_ok()
    }

    fn get_css_for_modifier(&self, _config: &Config, modifier: &Modifier, css_content: &mut String, _custom_css: &mut String) -> bool {
        match modifier.content() {
            "bottom" => self.css_template_value("bottom", css_content),
            "center" => self.css_template_value("center", css_content),
            "left" => self.css_template_value("left", css_content),
            "left-bottom" => self.css_template_value("left bottom", css_content),
            "left-top" => self.css_template_value("left top", css_content),
            "right" => self.css_template_value("right", css_content),
            "right-bottom" => self.css_template_value("right bottom", css_content),
            "right-top" => self.css_template_value("right top", css_content),
            "top" => self.css_template_value("top", css_content),
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct OverflowPlugin;

impl Plugin for OverflowPlugin {
    fn namespace(&self) -> &str {
        "overflow"
    }

    fn get_css_for_modifier(&self, _config: &Config, modifier: &Modifier, css_content: &mut String, _custom_css: &mut String) -> bool {
        match modifier.content() {
            "auto" => write!(css_content, "overflow: auto;").is_ok(),
            "x-auto" => write!(css_content, "overflow-x: auto;").is_ok(),
            "y-auto" => write!(css_content, "overflow-y: auto;").is_ok(),
            "hidden" => write!(css_content, "overflow: hidden;").is_ok(),
            "x-hidden" => write!(css_content, "overflow-x: hidden;").is_ok(),
            "y-hidden" => write!(css_content, "overflow-y: hidden;").is_ok(),
            "visible" => write!(css_content, "overflow: visible;").is_ok(),
            "x-visible" => write!(css_content, "overflow-x: visible;").is_ok(),
            "y-visible" => write!(css_content, "overflow-y: visible;").is_ok(),
            "scroll" => write!(css_content, "overflow: scroll;").is_ok(),
            "x-scroll" => write!(css_content, "overflow-x: scroll;").is_ok(),
            "y-scroll" => write!(css_content, "overflow-y: scroll;").is_ok(),
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct OverscrollPlugin;

impl Plugin for OverscrollPlugin {
    fn namespace(&self) -> &str {
        "overscroll"
    }

    fn get_css_for_modifier(&self, _config: &Config, modifier: &Modifier, css_content: &mut String, _custom_css: &mut String) -> bool {
        match modifier.content() {
            "auto" => write!(css_content, "overscroll-behavior: auto;").is_ok(),
            "y-auto" => write!(css_content, "overscroll-behavior-y: auto;").is_ok(),
            "x-auto" => write!(css_content, "overscroll-behavior-x: auto;").is_ok(),
            "contain" => write!(css_content, "overscroll-behavior: contain;").is_ok(),
            "y-contain" => write!(css_content, "overscroll-behavior-y: contain;").is_ok(),
            "x-contain" => write!(css_content, "overscroll-behavior-x: contain;").is_ok(),
            "none" => write!(css_content, "overscroll-behavior: none;").is_ok(),
            "y-none" => write!(css_content, "overscroll-behavior-y: none;").is_ok(),
            "x-none" => write!(css_content, "overscroll-behavior-x: none;").is_ok(),
            _ => false,
        }
    }
}
