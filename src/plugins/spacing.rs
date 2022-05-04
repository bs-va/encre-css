use super::Plugin;
use crate::utils::{default_lengths, value_matchers::*};

use std::fmt::Write;

// TODO: Boilerplate generator (just one structure for padding and another for margin)
#[derive(Debug)]
pub struct PaddingPlugin;

impl Plugin for PaddingPlugin {
    fn namespace(&self) -> &str {
        "p"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "padding: {val};").is_ok()
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> bool {
        if let Some(length) = default_lengths::get_basic(modifier) {
            self.css_template_value(&length, css_content)
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct PaddingXPlugin;

impl Plugin for PaddingXPlugin {
    fn namespace(&self) -> &str {
        "px"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(
            css_content,
            "padding-left: {val};
  padding-right: {val};"
        )
        .is_ok()
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> bool {
        if let Some(length) = default_lengths::get_basic(modifier) {
            self.css_template_value(&length, css_content)
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct PaddingYPlugin;

impl Plugin for PaddingYPlugin {
    fn namespace(&self) -> &str {
        "py"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(
            css_content,
            "padding-top: {val};
  padding-bottom: {val};"
        )
        .is_ok()
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> bool {
        if let Some(length) = default_lengths::get_basic(modifier) {
            self.css_template_value(&length, css_content)
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct PaddingLeftPlugin;

impl Plugin for PaddingLeftPlugin {
    fn namespace(&self) -> &str {
        "pl"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "padding-left: {val};").is_ok()
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> bool {
        if let Some(length) = default_lengths::get_basic(modifier) {
            self.css_template_value(&length, css_content)
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct PaddingRightPlugin;

impl Plugin for PaddingRightPlugin {
    fn namespace(&self) -> &str {
        "pr"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "padding-right: {val};").is_ok()
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> bool {
        if let Some(length) = default_lengths::get_basic(modifier) {
            self.css_template_value(&length, css_content)
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct PaddingTopPlugin;

impl Plugin for PaddingTopPlugin {
    fn namespace(&self) -> &str {
        "pt"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "padding-top: {val};").is_ok()
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> bool {
        if let Some(length) = default_lengths::get_basic(modifier) {
            self.css_template_value(&length, css_content)
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct PaddingBottomPlugin;

impl Plugin for PaddingBottomPlugin {
    fn namespace(&self) -> &str {
        "pb"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "padding-bottom: {val};").is_ok()
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> bool {
        if let Some(length) = default_lengths::get_basic(modifier) {
            self.css_template_value(&length, css_content)
        } else {
            false
        }
    }
}

// Margin

#[derive(Debug)]
pub struct MarginPlugin;

impl Plugin for MarginPlugin {
    fn namespace(&self) -> &str {
        "m"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "margin: {val};").is_ok()
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> bool {
        if is_matching_auto(modifier) {
            return self.css_template_value("auto", css_content);
        }

        if let Some(length) = default_lengths::get_basic(modifier) {
            self.css_template_value(&length, css_content)
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct MarginXPlugin;

impl Plugin for MarginXPlugin {
    fn namespace(&self) -> &str {
        "mx"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(
            css_content,
            "margin-left: {val};
  margin-right: {val};"
        )
        .is_ok()
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> bool {
        if is_matching_auto(modifier) {
            return self.css_template_value("auto", css_content);
        }

        if let Some(length) = default_lengths::get_basic(modifier) {
            self.css_template_value(&length, css_content)
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct MarginYPlugin;

impl Plugin for MarginYPlugin {
    fn namespace(&self) -> &str {
        "my"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(
            css_content,
            "margin-top: {val};
  margin-bottom: {val};"
        )
        .is_ok()
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> bool {
        if is_matching_auto(modifier) {
            return self.css_template_value("auto", css_content);
        }

        if let Some(length) = default_lengths::get_basic(modifier) {
            self.css_template_value(&length, css_content)
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct MarginLeftPlugin;

impl Plugin for MarginLeftPlugin {
    fn namespace(&self) -> &str {
        "ml"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "margin-left: {val};").is_ok()
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> bool {
        if is_matching_auto(modifier) {
            return self.css_template_value("auto", css_content);
        }

        if let Some(length) = default_lengths::get_basic(modifier) {
            self.css_template_value(&length, css_content)
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct MarginRightPlugin;

impl Plugin for MarginRightPlugin {
    fn namespace(&self) -> &str {
        "mr"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "margin-right: {val};").is_ok()
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> bool {
        if is_matching_auto(modifier) {
            return self.css_template_value("auto", css_content);
        }

        if let Some(length) = default_lengths::get_basic(modifier) {
            self.css_template_value(&length, css_content)
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct MarginTopPlugin;

impl Plugin for MarginTopPlugin {
    fn namespace(&self) -> &str {
        "mt"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "margin-top: {val};").is_ok()
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> bool {
        if is_matching_auto(modifier) {
            return self.css_template_value("auto", css_content);
        }

        if let Some(length) = default_lengths::get_basic(modifier) {
            self.css_template_value(&length, css_content)
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct MarginBottomPlugin;

impl Plugin for MarginBottomPlugin {
    fn namespace(&self) -> &str {
        "mb"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "margin-bottom: {val};").is_ok()
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> bool {
        if is_matching_auto(modifier) {
            return self.css_template_value("auto", css_content);
        }

        if let Some(length) = default_lengths::get_basic(modifier) {
            self.css_template_value(&length, css_content)
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct SpaceXPlugin;

impl Plugin for SpaceXPlugin {
    fn namespace(&self) -> &str {
        "space-x"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        if !css_content.contains("--tw-space-x-reverse") {
            write!(css_content, "--tw-space-x-reverse: 0;\n  ").ok();
        }

        write!(
            css_content,
            "margin-left: calc({val} * calc(1 - var(--tw-space-x-reverse)));
  margin-right: calc({val} * var(--tw-space-x-reverse));"
        )
        .is_ok()
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> bool {
        // TODO: class with `> :not([hidden]) ~ :not([hidden])`
        if modifier == "reverse" {
            return write!(css_content, "--tw-space-x-reverse: 1;").is_ok();
        }

        if let Some(length) = default_lengths::get_basic(modifier) {
            write!(css_content, "--tw-space-x-reverse: 0;\n  ").ok();
            self.css_template_value(&length, css_content)
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct SpaceYPlugin;

impl Plugin for SpaceYPlugin {
    fn namespace(&self) -> &str {
        "space-y"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        if !css_content.contains("--tw-space-y-reverse") {
            write!(css_content, "--tw-space-y-reverse: 0;\n  ").ok();
        }

        write!(
            css_content,
            "margin-top: calc({val} * calc(1 - var(--tw-space-y-reverse)));
  margin-bottom: calc({val} * var(--tw-space-y-reverse));"
        )
        .is_ok()
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> bool {
        // TODO: class with `> :not([hidden]) ~ :not([hidden])`
        if modifier == "reverse" {
            return write!(css_content, "--tw-space-y-reverse: 1;").is_ok();
        }

        if let Some(length) = default_lengths::get_basic(modifier) {
            write!(css_content, "--tw-space-y-reverse: 0;\n  ").ok();
            self.css_template_value(&length, css_content)
        } else {
            false
        }
    }
}
