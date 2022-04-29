use super::Plugin;
use crate::utils::{default_colors, default_lengths, value_matchers::*};

use std::fmt::{Result, Write};

#[derive(Debug)]
pub struct AccentColorPlugin;

impl Plugin for AccentColorPlugin {
    fn namespace(&self) -> &str {
        "accent"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "color" || is_matching_color(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        if val.contains("--tw-opacity") {
            write!(
                css_content,
                "accent-color: {};",
                val.replace(" / var(--tw-opacity)", "")
            )
        } else {
            write!(css_content, "accent-color: {val};")
        }
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if let Some(color) = default_colors::get(modifier) {
            self.css_template_value(&color, css_content)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct AppearancePlugin;

impl Plugin for AppearancePlugin {
    fn namespace(&self) -> &str {
        "appearance"
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if modifier == "none" {
            write!(css_content, "-webkit-appearance: none;
  -moz-appearance: none;
  appearance: none;")
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn namespace(&self) -> &str {
        "cursor"
    }

    fn is_matching_value(&self, _hint: &str, _val: &str) -> bool {
        true
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "cursor: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if [
            "auto",
            "default",
            "pointer",
            "wait",
            "text",
            "move",
            "help",
            "not-allowed",
            "none",
            "context-menu",
            "progress",
            "cell",
            "crosshair",
            "vertical-text",
            "alias",
            "copy",
            "no-drop",
            "grab",
            "grabbing",
            "all-scroll",
            "col-resize",
            "row-resize",
            "n-resize",
            "e-resize",
            "s-resize",
            "w-resize",
            "ne-resize",
            "nw-resize",
            "se-resize",
            "sw-resize",
            "ew-resize",
            "ns-resize",
            "nesw-resize",
            "nwse-resize",
            "zoom-in",
            "zoom-out",
        ]
        .contains(&modifier)
        {
            self.css_template_value(modifier, css_content)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct CaretColorPlugin;

impl Plugin for CaretColorPlugin {
    fn namespace(&self) -> &str {
        "caret"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "color" || is_matching_color(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        if val.contains("--tw-opacity") {
            write!(
                css_content,
                "caret-color: {};",
                val.replace(" / var(--tw-opacity)", "")
            )
        } else {
            write!(css_content, "caret-color: {val};")
        }
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if let Some(color) = default_colors::get(modifier) {
            self.css_template_value(&color, css_content)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct PointerEventsPlugin;

impl Plugin for PointerEventsPlugin {
    fn namespace(&self) -> &str {
        "pointer-events"
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if ["none", "auto"].contains(&modifier) {
            write!(css_content, "pointer-events: {modifier};")
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct ResizePlugin;

impl Plugin for ResizePlugin {
    fn namespace(&self) -> &str {
        "resize"
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        match modifier {
            "" => write!(css_content, "resize: both;"),
            "none" => write!(css_content, "resize: none;"),
            "x" => write!(css_content, "resize: horizontal;"),
            "y" => write!(css_content, "resize: vertical;"),
            _ => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct ScrollBehaviorPlugin;

impl Plugin for ScrollBehaviorPlugin {
    fn namespace(&self) -> &str {
        "scroll"
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if ["auto", "smooth"].contains(&modifier) {
            write!(css_content, "scroll-behavior: {modifier};")
        } else {
            Ok(())
        }
    }
}

// Scroll margin

#[derive(Debug)]
pub struct ScrollMarginPlugin;

impl Plugin for ScrollMarginPlugin {
    fn namespace(&self) -> &str {
        "scroll-m"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "scroll-margin: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if is_matching_auto(modifier) {
            return self.css_template_value("auto", css_content);
        }

        if let Some(length) = default_lengths::get_basic(modifier) {
            self.css_template_value(&length, css_content)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct ScrollMarginXPlugin;

impl Plugin for ScrollMarginXPlugin {
    fn namespace(&self) -> &str {
        "scroll-mx"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(
            css_content,
            "scroll-margin-left: {val};
  scroll-margin-right: {val};"
        )
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if is_matching_auto(modifier) {
            return self.css_template_value("auto", css_content);
        }

        if let Some(length) = default_lengths::get_basic(modifier) {
            self.css_template_value(&length, css_content)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct ScrollMarginYPlugin;

impl Plugin for ScrollMarginYPlugin {
    fn namespace(&self) -> &str {
        "scroll-my"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(
            css_content,
            "scroll-margin-top: {val};
  scroll-margin-bottom: {val};"
        )
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if is_matching_auto(modifier) {
            return self.css_template_value("auto", css_content);
        }

        if let Some(length) = default_lengths::get_basic(modifier) {
            self.css_template_value(&length, css_content)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct ScrollMarginLeftPlugin;

impl Plugin for ScrollMarginLeftPlugin {
    fn namespace(&self) -> &str {
        "scroll-ml"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "scroll-margin-left: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if is_matching_auto(modifier) {
            return self.css_template_value("auto", css_content);
        }

        if let Some(length) = default_lengths::get_basic(modifier) {
            self.css_template_value(&length, css_content)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct ScrollMarginRightPlugin;

impl Plugin for ScrollMarginRightPlugin {
    fn namespace(&self) -> &str {
        "scroll-mr"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "scroll-margin-right: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if is_matching_auto(modifier) {
            return self.css_template_value("auto", css_content);
        }

        if let Some(length) = default_lengths::get_basic(modifier) {
            self.css_template_value(&length, css_content)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct ScrollMarginTopPlugin;

impl Plugin for ScrollMarginTopPlugin {
    fn namespace(&self) -> &str {
        "scroll-mt"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "scroll-margin-top: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if is_matching_auto(modifier) {
            return self.css_template_value("auto", css_content);
        }

        if let Some(length) = default_lengths::get_basic(modifier) {
            self.css_template_value(&length, css_content)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct ScrollMarginBottomPlugin;

impl Plugin for ScrollMarginBottomPlugin {
    fn namespace(&self) -> &str {
        "scroll-mb"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "scroll-margin-bottom: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if is_matching_auto(modifier) {
            return self.css_template_value("auto", css_content);
        }

        if let Some(length) = default_lengths::get_basic(modifier) {
            self.css_template_value(&length, css_content)
        } else {
            Ok(())
        }
    }
}

// Scroll padding

#[derive(Debug)]
pub struct ScrollPaddingPlugin;

impl Plugin for ScrollPaddingPlugin {
    fn namespace(&self) -> &str {
        "scroll-p"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "scroll-padding: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if let Some(length) = default_lengths::get_basic(modifier) {
            self.css_template_value(&length, css_content)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct ScrollPaddingXPlugin;

impl Plugin for ScrollPaddingXPlugin {
    fn namespace(&self) -> &str {
        "scroll-px"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(
            css_content,
            "scroll-padding-left: {val};
  scroll-padding-right: {val};"
        )
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if let Some(length) = default_lengths::get_basic(modifier) {
            self.css_template_value(&length, css_content)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct ScrollPaddingYPlugin;

impl Plugin for ScrollPaddingYPlugin {
    fn namespace(&self) -> &str {
        "scroll-py"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(
            css_content,
            "scroll-padding-top: {val};
  scroll-padding-bottom: {val};"
        )
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if let Some(length) = default_lengths::get_basic(modifier) {
            self.css_template_value(&length, css_content)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct ScrollPaddingLeftPlugin;

impl Plugin for ScrollPaddingLeftPlugin {
    fn namespace(&self) -> &str {
        "scroll-pl"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "scroll-padding-left: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if let Some(length) = default_lengths::get_basic(modifier) {
            self.css_template_value(&length, css_content)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct ScrollPaddingRightPlugin;

impl Plugin for ScrollPaddingRightPlugin {
    fn namespace(&self) -> &str {
        "scroll-pr"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "scroll-padding-right: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if let Some(length) = default_lengths::get_basic(modifier) {
            self.css_template_value(&length, css_content)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct ScrollPaddingTopPlugin;

impl Plugin for ScrollPaddingTopPlugin {
    fn namespace(&self) -> &str {
        "scroll-pt"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "scroll-padding-top: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if let Some(length) = default_lengths::get_basic(modifier) {
            self.css_template_value(&length, css_content)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct ScrollPaddingBottomPlugin;

impl Plugin for ScrollPaddingBottomPlugin {
    fn namespace(&self) -> &str {
        "scroll-pb"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "scroll-padding-bottom: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if let Some(length) = default_lengths::get_basic(modifier) {
            self.css_template_value(&length, css_content)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct ScrollSnapAlignPlugin;

impl Plugin for ScrollSnapAlignPlugin {
    fn namespace(&self) -> &str {
        "snap"
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        match modifier {
            "start" => write!(css_content, "scroll-snap-align: start;"),
            "end" => write!(css_content, "scroll-snap-align: end;"),
            "center" => write!(css_content, "scroll-snap-align: center;"),
            "align-none" => write!(css_content, "scroll-snap-align: none;"),
            _ => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct ScrollSnapStopPlugin;

impl Plugin for ScrollSnapStopPlugin {
    fn namespace(&self) -> &str {
        "snap"
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        match modifier {
            "normal" => write!(css_content, "scroll-snap-stop: normal;"),
            "always" => write!(css_content, "scroll-snap-stop: always;"),
            _ => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct ScrollSnapTypePlugin;

impl Plugin for ScrollSnapTypePlugin {
    fn namespace(&self) -> &str {
        "snap"
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        match modifier {
            "none" => write!(css_content, "-ms-scroll-snap-type: none;
  scroll-snap-type: none;"),
            "x" => write!(css_content, "-ms-scroll-snap-type: x var(--tw-scroll-snap-strictness);
  scroll-snap-type: x var(--tw-scroll-snap-strictness);"),
            "y" => write!(css_content, "-ms-scroll-snap-type: y var(--tw-scroll-snap-strictness);
  scroll-snap-type: y var(--tw-scroll-snap-strictness);"),
            "both" => write!(css_content, "-ms-scroll-snap-type: both var(--tw-scroll-snap-strictness);
  scroll-snap-type: both var(--tw-scroll-snap-strictness);"),
            "mandatory" => write!(css_content, "--tw-scroll-snap-strictness: mandatory;"),
            "proximity" => write!(css_content, "--tw-scroll-snap-strictness: proximity;"),
            _ => Ok(()),
        }
    }
}


#[derive(Debug)]
pub struct TouchActionPlugin;

impl Plugin for TouchActionPlugin {
    fn namespace(&self) -> &str {
        "touch"
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if ["auto", "none", "pan-x", "pan-left", "pan-right", "pan-y", "pan-up", "pan-down", "pinch-zoom", "manipulation"].contains(&modifier) {
            write!(css_content, "touch-action: {modifier};")
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct UserSelectPlugin;

impl Plugin for UserSelectPlugin {
    fn namespace(&self) -> &str {
        "select"
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if ["none", "text", "all", "auto"].contains(&modifier) {
            write!(css_content, "user-select: {modifier};")
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct WillChangePlugin;

impl Plugin for WillChangePlugin {
    fn namespace(&self) -> &str {
        "will-change"
    }

    fn is_matching_value(&self, _hint: &str, _val: &str) -> bool {
        true
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "will-change: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        match modifier {
            "auto" => self.css_template_value("auto", css_content),
            "scroll" => self.css_template_value("scroll-position", css_content),
            "contents" => self.css_template_value("contents", css_content),
            "transform" => self.css_template_value("transfrom", css_content),
            _ => Ok(()),
        }
    }
}
