use super::Plugin;
use crate::utils::{default_colors, default_lengths, value_matchers::*};
use crate::{config::Config, selector::Modifier};

use std::fmt::Write;

#[derive(Debug)]
pub struct AccentColorPlugin;

impl Plugin for AccentColorPlugin {
    fn namespace(&self) -> &str {
        "accent"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "color" || is_matching_color(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        if val.contains("--en-opacity") {
            write!(
                css_content,
                "accent-color: {};",
                val.replace(" / var(--en-opacity)", "")
            )
            .is_ok()
        } else {
            write!(css_content, "accent-color: {val};").is_ok()
        }
    }

    fn get_css_for_modifier(
        &self,
        config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        if let Some(color) = default_colors::get(config, modifier.content()) {
            self.css_template_value(&color, css_content)
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct AppearancePlugin;

impl Plugin for AppearancePlugin {
    fn namespace(&self) -> &str {
        "appearance"
    }

    fn get_css_for_modifier(
        &self,
        _config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        if modifier.is("none") {
            write!(
                css_content,
                "-webkit-appearance: none;
-moz-appearance: none;
appearance: none;"
            )
            .is_ok()
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn namespace(&self) -> &str {
        "cursor"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        is_matching_all(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "cursor: {val};").is_ok()
    }

    fn get_css_for_modifier(
        &self,
        _config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        if modifier.is_one_of(&[
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
        ]) {
            self.css_template_value(modifier.content(), css_content)
        } else {
            false
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

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        if val.contains("--en-opacity") {
            write!(
                css_content,
                "caret-color: {};",
                val.replace(" / var(--en-opacity)", "")
            )
            .is_ok()
        } else {
            write!(css_content, "caret-color: {val};").is_ok()
        }
    }

    fn get_css_for_modifier(
        &self,
        config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        if let Some(color) = default_colors::get(config, modifier.content()) {
            self.css_template_value(&color, css_content)
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct PointerEventsPlugin;

impl Plugin for PointerEventsPlugin {
    fn namespace(&self) -> &str {
        "pointer-events"
    }

    fn get_css_for_modifier(
        &self,
        _config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        if modifier.is_one_of(&["none", "auto"]) {
            write!(css_content, "pointer-events: {modifier};").is_ok()
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct ResizePlugin;

impl Plugin for ResizePlugin {
    fn namespace(&self) -> &str {
        "resize"
    }

    fn get_css_for_modifier(
        &self,
        _config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        match modifier.content() {
            "" => write!(css_content, "resize: both;").is_ok(),
            "none" => write!(css_content, "resize: none;").is_ok(),
            "x" => write!(css_content, "resize: horizontal;").is_ok(),
            "y" => write!(css_content, "resize: vertical;").is_ok(),
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct ScrollBehaviorPlugin;

impl Plugin for ScrollBehaviorPlugin {
    fn namespace(&self) -> &str {
        "scroll"
    }

    fn get_css_for_modifier(
        &self,
        _config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        if modifier.is_one_of(&["auto", "smooth"]) {
            write!(css_content, "scroll-behavior: {modifier};").is_ok()
        } else {
            false
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

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "scroll-margin: {val};").is_ok()
    }

    fn get_css_for_modifier(
        &self,
        _config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        if modifier.is("auto") {
            return self.css_template_value("auto", css_content);
        }

        if let Some(length) = default_lengths::get_basic(modifier.content(), modifier.is_negative())
        {
            self.css_template_value(&length, css_content)
        } else {
            false
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

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(
            css_content,
            "scroll-margin-left: {val};
scroll-margin-right: {val};"
        )
        .is_ok()
    }

    fn get_css_for_modifier(
        &self,
        _config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        if modifier.is("auto") {
            return self.css_template_value("auto", css_content);
        }

        if let Some(length) = default_lengths::get_basic(modifier.content(), modifier.is_negative())
        {
            self.css_template_value(&length, css_content)
        } else {
            false
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

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(
            css_content,
            "scroll-margin-top: {val};
scroll-margin-bottom: {val};"
        )
        .is_ok()
    }

    fn get_css_for_modifier(
        &self,
        _config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        if modifier.is("auto") {
            return self.css_template_value("auto", css_content);
        }

        if let Some(length) = default_lengths::get_basic(modifier.content(), modifier.is_negative())
        {
            self.css_template_value(&length, css_content)
        } else {
            false
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

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "scroll-margin-left: {val};").is_ok()
    }

    fn get_css_for_modifier(
        &self,
        _config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        if modifier.is("auto") {
            return self.css_template_value("auto", css_content);
        }

        if let Some(length) = default_lengths::get_basic(modifier.content(), modifier.is_negative())
        {
            self.css_template_value(&length, css_content)
        } else {
            false
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

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "scroll-margin-right: {val};").is_ok()
    }

    fn get_css_for_modifier(
        &self,
        _config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        if modifier.is("auto") {
            return self.css_template_value("auto", css_content);
        }

        if let Some(length) = default_lengths::get_basic(modifier.content(), modifier.is_negative())
        {
            self.css_template_value(&length, css_content)
        } else {
            false
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

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "scroll-margin-top: {val};").is_ok()
    }

    fn get_css_for_modifier(
        &self,
        _config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        if modifier.is("auto") {
            return self.css_template_value("auto", css_content);
        }

        if let Some(length) = default_lengths::get_basic(modifier.content(), modifier.is_negative())
        {
            self.css_template_value(&length, css_content)
        } else {
            false
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

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "scroll-margin-bottom: {val};").is_ok()
    }

    fn get_css_for_modifier(
        &self,
        _config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        if modifier.is("auto") {
            return self.css_template_value("auto", css_content);
        }

        if let Some(length) = default_lengths::get_basic(modifier.content(), modifier.is_negative())
        {
            self.css_template_value(&length, css_content)
        } else {
            false
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

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "scroll-padding: {val};").is_ok()
    }

    fn get_css_for_modifier(
        &self,
        _config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        if let Some(length) = default_lengths::get_basic(modifier.content(), modifier.is_negative())
        {
            self.css_template_value(&length, css_content)
        } else {
            false
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

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(
            css_content,
            "scroll-padding-left: {val};
scroll-padding-right: {val};"
        )
        .is_ok()
    }

    fn get_css_for_modifier(
        &self,
        _config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        if let Some(length) = default_lengths::get_basic(modifier.content(), modifier.is_negative())
        {
            self.css_template_value(&length, css_content)
        } else {
            false
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

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(
            css_content,
            "scroll-padding-top: {val};
scroll-padding-bottom: {val};"
        )
        .is_ok()
    }

    fn get_css_for_modifier(
        &self,
        _config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        if let Some(length) = default_lengths::get_basic(modifier.content(), modifier.is_negative())
        {
            self.css_template_value(&length, css_content)
        } else {
            false
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

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "scroll-padding-left: {val};").is_ok()
    }

    fn get_css_for_modifier(
        &self,
        _config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        if let Some(length) = default_lengths::get_basic(modifier.content(), modifier.is_negative())
        {
            self.css_template_value(&length, css_content)
        } else {
            false
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

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "scroll-padding-right: {val};").is_ok()
    }

    fn get_css_for_modifier(
        &self,
        _config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        if let Some(length) = default_lengths::get_basic(modifier.content(), modifier.is_negative())
        {
            self.css_template_value(&length, css_content)
        } else {
            false
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

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "scroll-padding-top: {val};").is_ok()
    }

    fn get_css_for_modifier(
        &self,
        _config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        if let Some(length) = default_lengths::get_basic(modifier.content(), modifier.is_negative())
        {
            self.css_template_value(&length, css_content)
        } else {
            false
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

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "scroll-padding-bottom: {val};").is_ok()
    }

    fn get_css_for_modifier(
        &self,
        _config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        if let Some(length) = default_lengths::get_basic(modifier.content(), modifier.is_negative())
        {
            self.css_template_value(&length, css_content)
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct ScrollSnapAlignPlugin;

impl Plugin for ScrollSnapAlignPlugin {
    fn namespace(&self) -> &str {
        "snap"
    }

    fn get_css_for_modifier(
        &self,
        _config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        match modifier.content() {
            "start" => write!(css_content, "scroll-snap-align: start;").is_ok(),
            "end" => write!(css_content, "scroll-snap-align: end;").is_ok(),
            "center" => write!(css_content, "scroll-snap-align: center;").is_ok(),
            "align-none" => write!(css_content, "scroll-snap-align: none;").is_ok(),
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct ScrollSnapStopPlugin;

impl Plugin for ScrollSnapStopPlugin {
    fn namespace(&self) -> &str {
        "snap"
    }

    fn get_css_for_modifier(
        &self,
        _config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        match modifier.content() {
            "normal" => write!(css_content, "scroll-snap-stop: normal;").is_ok(),
            "always" => write!(css_content, "scroll-snap-stop: always;").is_ok(),
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct ScrollSnapTypePlugin;

impl Plugin for ScrollSnapTypePlugin {
    fn namespace(&self) -> &str {
        "snap"
    }

    fn get_css_for_modifier(
        &self,
        _config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        match modifier.content() {
            "none" => write!(
                css_content,
                "-ms-scroll-snap-type: none;
scroll-snap-type: none;"
            )
            .is_ok(),
            "x" => write!(
                css_content,
                "-ms-scroll-snap-type: x var(--en-scroll-snap-strictness);
scroll-snap-type: x var(--en-scroll-snap-strictness);"
            )
            .is_ok(),
            "y" => write!(
                css_content,
                "-ms-scroll-snap-type: y var(--en-scroll-snap-strictness);
scroll-snap-type: y var(--en-scroll-snap-strictness);"
            )
            .is_ok(),
            "both" => write!(
                css_content,
                "-ms-scroll-snap-type: both var(--en-scroll-snap-strictness);
scroll-snap-type: both var(--en-scroll-snap-strictness);"
            )
            .is_ok(),
            "mandatory" => write!(css_content, "--en-scroll-snap-strictness: mandatory;").is_ok(),
            "proximity" => write!(css_content, "--en-scroll-snap-strictness: proximity;").is_ok(),
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct TouchActionPlugin;

impl Plugin for TouchActionPlugin {
    fn namespace(&self) -> &str {
        "touch"
    }

    fn get_css_for_modifier(
        &self,
        _config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        if modifier.is_one_of(&[
            "auto",
            "none",
            "pan-x",
            "pan-left",
            "pan-right",
            "pan-y",
            "pan-up",
            "pan-down",
            "pinch-zoom",
            "manipulation",
        ]) {
            write!(css_content, "touch-action: {modifier};").is_ok()
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct UserSelectPlugin;

impl Plugin for UserSelectPlugin {
    fn namespace(&self) -> &str {
        "select"
    }

    fn get_css_for_modifier(
        &self,
        _config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        if modifier.is_one_of(&["none", "text", "all", "auto"]) {
            write!(css_content, "user-select: {modifier};").is_ok()
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct WillChangePlugin;

impl Plugin for WillChangePlugin {
    fn namespace(&self) -> &str {
        "will-change"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        is_matching_all(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "will-change: {val};").is_ok()
    }

    fn get_css_for_modifier(
        &self,
        _config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        match modifier.content() {
            "auto" => self.css_template_value("auto", css_content),
            "scroll" => self.css_template_value("scroll-position", css_content),
            "contents" => self.css_template_value("contents", css_content),
            "transform" => self.css_template_value("transfrom", css_content),
            _ => false,
        }
    }
}
