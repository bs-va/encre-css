use super::Plugin;
use crate::utils::{default_lengths, value_matchers::*};

use std::fmt::{Result, Write};

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

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "padding: {val};")
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
pub struct PaddingXPlugin;

impl Plugin for PaddingXPlugin {
    fn namespace(&self) -> &str {
        "px"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content,
            "padding-left: {val};
  padding-right: {val};"
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
pub struct PaddingYPlugin;

impl Plugin for PaddingYPlugin {
    fn namespace(&self) -> &str {
        "py"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content,
            "padding-top: {val};
  padding-bottom: {val};"
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
pub struct PaddingLeftPlugin;

impl Plugin for PaddingLeftPlugin {
    fn namespace(&self) -> &str {
        "pl"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "padding-left: {val};")
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
pub struct PaddingRightPlugin;

impl Plugin for PaddingRightPlugin {
    fn namespace(&self) -> &str {
        "pr"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "padding-right: {val};")
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
pub struct PaddingTopPlugin;

impl Plugin for PaddingTopPlugin {
    fn namespace(&self) -> &str {
        "pt"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "padding-top: {val};")
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
pub struct PaddingBottomPlugin;

impl Plugin for PaddingBottomPlugin {
    fn namespace(&self) -> &str {
        "pb"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "padding-bottom: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if let Some(length) = default_lengths::get_basic(modifier) {
            self.css_template_value(&length, css_content)
        } else {
            Ok(())
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

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "margin: {val};")
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
pub struct MarginXPlugin;

impl Plugin for MarginXPlugin {
    fn namespace(&self) -> &str {
        "mx"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content,
            "margin-left: {val};
  margin-right: {val};"
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
pub struct MarginYPlugin;

impl Plugin for MarginYPlugin {
    fn namespace(&self) -> &str {
        "my"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content,
            "margin-top: {val};
  margin-bottom: {val};"
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
pub struct MarginLeftPlugin;

impl Plugin for MarginLeftPlugin {
    fn namespace(&self) -> &str {
        "ml"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "margin-left: {val};")
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
pub struct MarginRightPlugin;

impl Plugin for MarginRightPlugin {
    fn namespace(&self) -> &str {
        "mr"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "margin-right: {val};")
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
pub struct MarginTopPlugin;

impl Plugin for MarginTopPlugin {
    fn namespace(&self) -> &str {
        "mt"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "margin-top: {val};")
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
pub struct MarginBottomPlugin;

impl Plugin for MarginBottomPlugin {
    fn namespace(&self) -> &str {
        "mb"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "margin-bottom: {val};")
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
pub struct SpaceXPlugin;

impl Plugin for SpaceXPlugin {
    fn namespace(&self) -> &str {
        "space-x"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        if !css_content.contains("--tw-space-x-reverse") {
            write!(css_content, "--tw-space-x-reverse: 0;\n  ")?;
        }

        write!(css_content,
            "margin-left: calc({val} * calc(1 - var(--tw-space-x-reverse)));
  margin-right: calc({val} * var(--tw-space-x-reverse));"
        )
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        // TODO: class with `> :not([hidden]) ~ :not([hidden])`
        if modifier == "reverse" {
            return write!(css_content, "--tw-space-x-reverse: 1;");
        }

        if let Some(length) = default_lengths::get_basic(modifier) {
            write!(css_content, "--tw-space-x-reverse: 0;\n  ")?;
            self.css_template_value(&length, css_content)
        } else {
            Ok(())
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

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        if !css_content.contains("--tw-space-y-reverse") {
            write!(css_content, "--tw-space-y-reverse: 0;\n  ")?;
        }

        write!(css_content,
            "margin-top: calc({val} * calc(1 - var(--tw-space-y-reverse)));
  margin-bottom: calc({val} * var(--tw-space-y-reverse));"
        )
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        // TODO: class with `> :not([hidden]) ~ :not([hidden])`
        if modifier == "reverse" {
            return write!(css_content, "--tw-space-y-reverse: 1;");
        }

        if let Some(length) = default_lengths::get_basic(modifier) {
            write!(css_content, "--tw-space-y-reverse: 0;\n  ")?;
            self.css_template_value(&length, css_content)
        } else {
            Ok(())
        }
    }
}

/*pub fn init(selectors: &mut SelectorList) {
    selectors.register("-m-0", "".to_string());
    selectors.register("-m-0.5", "".to_string());
    selectors.register("-m-1", "".to_string());
    selectors.register("-m-1.5", "".to_string());
    selectors.register("-m-2", "".to_string());
    selectors.register("-m-2.5", "".to_string());
    selectors.register("-m-3", "".to_string());
    selectors.register("-m-3.5", "".to_string());
    selectors.register("-m-4", "".to_string());
    selectors.register("-m-5", "".to_string());
    selectors.register("-m-6", "".to_string());
    selectors.register("-m-8", "".to_string());
    selectors.register("-m-10", "".to_string());
    selectors.register("-m-11", "".to_string());
    selectors.register("-m-12", "".to_string());
    selectors.register("-m-14", "".to_string());
    selectors.register("-m-16", "".to_string());
    selectors.register("-m-20", "".to_string());
    selectors.register("-m-24", "".to_string());
    selectors.register("-m-28", "".to_string());
    selectors.register("-m-32", "".to_string());
    selectors.register("-m-36", "".to_string());
    selectors.register("-m-40", "".to_string());
    selectors.register("-m-44", "".to_string());
    selectors.register("-m-48", "".to_string());
    selectors.register("-m-52", "".to_string());
    selectors.register("-m-56", "".to_string());
    selectors.register("-m-64", "".to_string());
    selectors.register("-m-72", "".to_string());
    selectors.register("-m-80", "".to_string());
    selectors.register("-m-96", "".to_string());
    selectors.register("-m-px", "".to_string());
    selectors.register("-my-0", "".to_string());
    selectors.register("-mx-0", "".to_string());
    selectors.register("-my-0.5", "".to_string());
    selectors.register("-mx-0.5", "".to_string());
    selectors.register("-my-1", "".to_string());
    selectors.register("-mx-1", "".to_string());
    selectors.register("-my-1.5", "".to_string());
    selectors.register("-mx-1.5", "".to_string());
    selectors.register("-my-2", "".to_string());
    selectors.register("-mx-2", "".to_string());
    selectors.register("-my-2.5", "".to_string());
    selectors.register("-mx-2.5", "".to_string());
    selectors.register("-my-3", "".to_string());
    selectors.register("-mx-3", "".to_string());
    selectors.register("-my-3.5", "".to_string());
    selectors.register("-mx-3.5", "".to_string());
    selectors.register("-my-4", "".to_string());
    selectors.register("-mx-4", "".to_string());
    selectors.register("-my-5", "".to_string());
    selectors.register("-mx-5", "".to_string());
    selectors.register("-my-6", "".to_string());
    selectors.register("-mx-6", "".to_string());
    selectors.register("-my-7", "".to_string());
    selectors.register("-mx-7", "".to_string());
    selectors.register("-my-8", "".to_string());
    selectors.register("-mx-8", "".to_string());
    selectors.register("-my-9", "".to_string());
    selectors.register("-mx-9", "".to_string());
    selectors.register("-my-10", "".to_string());
    selectors.register("-mx-10", "".to_string());
    selectors.register("-my-11", "".to_string());
    selectors.register("-mx-11", "".to_string());
    selectors.register("-my-12", "".to_string());
    selectors.register("-mx-12", "".to_string());
    selectors.register("-my-14", "".to_string());
    selectors.register("-mx-14", "".to_string());
    selectors.register("-my-16", "".to_string());
    selectors.register("-mx-16", "".to_string());
    selectors.register("-my-20", "".to_string());
    selectors.register("-mx-20", "".to_string());
    selectors.register("-my-24", "".to_string());
    selectors.register("-mx-24", "".to_string());
    selectors.register("-my-28", "".to_string());
    selectors.register("-mx-28", "".to_string());
    selectors.register("-my-32", "".to_string());
    selectors.register("-mx-32", "".to_string());
    selectors.register("-my-36", "".to_string());
    selectors.register("-mx-36", "".to_string());
    selectors.register("-my-40", "".to_string());
    selectors.register("-mx-40", "".to_string());
    selectors.register("-my-44", "".to_string());
    selectors.register("-mx-44", "".to_string());
    selectors.register("-my-48", "".to_string());
    selectors.register("-mx-48", "".to_string());
    selectors.register("-my-52", "".to_string());
    selectors.register("-mx-52", "".to_string());
    selectors.register("-my-56", "".to_string());
    selectors.register("-mx-56", "".to_string());
    selectors.register("-my-60", "".to_string());
    selectors.register("-mx-60", "".to_string());
    selectors.register("-my-64", "".to_string());
    selectors.register("-mx-64", "".to_string());
    selectors.register("-my-70", "".to_string());
    selectors.register("-mx-70", "".to_string());
    selectors.register("-my-80", "".to_string());
    selectors.register("-mx-80", "".to_string());
    selectors.register("-my-96", "".to_string());
    selectors.register("-mx-96", "".to_string());
    selectors.register("-my-px", "".to_string());
    selectors.register("-mx-px", "".to_string());
    selectors.register("-mt-0", "".to_string());
    selectors.register("-mr-0", "".to_string());
    selectors.register("-mb-0", "".to_string());
    selectors.register("-ml-0", "".to_string());
    selectors.register("-mt-0.5", "".to_string());
    selectors.register("-mr-0.5", "".to_string());
    selectors.register("-mb-0.5", "".to_string());
    selectors.register("-ml-0.5", "".to_string());
    selectors.register("-mt-1", "".to_string());
    selectors.register("-mr-1", "".to_string());
    selectors.register("-mb-1", "".to_string());
    selectors.register("-ml-1", "".to_string());
    selectors.register("-mt-1.5", "".to_string());
    selectors.register("-mr-1.5", "".to_string());
    selectors.register("-mb-1.5", "".to_string());
    selectors.register("-ml-1.5", "".to_string());
    selectors.register("-mt-2", "".to_string());
    selectors.register("-mr-2", "".to_string());
    selectors.register("-mb-2", "".to_string());
    selectors.register("-ml-2", "".to_string());
    selectors.register("-mt-2.5", "".to_string());
    selectors.register("-mr-2.5", "".to_string());
    selectors.register("-mb-2.5", "".to_string());
    selectors.register("-ml-2.5", "".to_string());
    selectors.register("-mt-3", "".to_string());
    selectors.register("-mr-3", "".to_string());
    selectors.register("-mb-3", "".to_string());
    selectors.register("-ml-3", "".to_string());
    selectors.register("-mt-3.5", "".to_string());
    selectors.register("-mr-3.5", "".to_string());
    selectors.register("-mb-3.5", "".to_string());
    selectors.register("-ml-3.5", "".to_string());
    selectors.register("-mt-4", "".to_string());
    selectors.register("-mr-4", "".to_string());
    selectors.register("-mb-4", "".to_string());
    selectors.register("-ml-4", "".to_string());
    selectors.register("-mt-5", "".to_string());
    selectors.register("-mr-5", "".to_string());
    selectors.register("-mb-5", "".to_string());
    selectors.register("-ml-5", "".to_string());
    selectors.register("-mt-6", "".to_string());
    selectors.register("-mr-6", "".to_string());
    selectors.register("-mb-6", "".to_string());
    selectors.register("-ml-6", "".to_string());
    selectors.register("-mt-7", "".to_string());
    selectors.register("-mr-7", "".to_string());
    selectors.register("-mb-7", "".to_string());
    selectors.register("-ml-7", "".to_string());
    selectors.register("-mt-8", "".to_string());
    selectors.register("-mr-8", "".to_string());
    selectors.register("-mb-8", "".to_string());
    selectors.register("-ml-8", "".to_string());
    selectors.register("-mt-9", "".to_string());
    selectors.register("-mr-9", "".to_string());
    selectors.register("-mb-9", "".to_string());
    selectors.register("-ml-9", "".to_string());
    selectors.register("-mt-10", "".to_string());
    selectors.register("-mr-10", "".to_string());
    selectors.register("-mb-10", "".to_string());
    selectors.register("-ml-10", "".to_string());
    selectors.register("-mt-11", "".to_string());
    selectors.register("-mr-11", "".to_string());
    selectors.register("-mb-11", "".to_string());
    selectors.register("-ml-11", "".to_string());
    selectors.register("-mt-12", "".to_string());
    selectors.register("-mr-12", "".to_string());
    selectors.register("-mb-12", "".to_string());
    selectors.register("-ml-12", "".to_string());
    selectors.register("-mt-14", "".to_string());
    selectors.register("-mr-14", "".to_string());
    selectors.register("-mb-14", "".to_string());
    selectors.register("-ml-14", "".to_string());
    selectors.register("-mt-16", "".to_string());
    selectors.register("-mr-16", "".to_string());
    selectors.register("-mb-16", "".to_string());
    selectors.register("-ml-16", "".to_string());
    selectors.register("-mt-20", "".to_string());
    selectors.register("-mr-20", "".to_string());
    selectors.register("-mb-20", "".to_string());
    selectors.register("-ml-20", "".to_string());
    selectors.register("-mt-24", "".to_string());
    selectors.register("-mr-24", "".to_string());
    selectors.register("-mb-24", "".to_string());
    selectors.register("-ml-24", "".to_string());
    selectors.register("-mt-28", "".to_string());
    selectors.register("-mr-28", "".to_string());
    selectors.register("-mb-28", "".to_string());
    selectors.register("-ml-28", "".to_string());
    selectors.register("-mt-32", "".to_string());
    selectors.register("-mr-32", "".to_string());
    selectors.register("-mb-32", "".to_string());
    selectors.register("-ml-32", "".to_string());
    selectors.register("-mt-36", "".to_string());
    selectors.register("-mr-36", "".to_string());
    selectors.register("-mb-36", "".to_string());
    selectors.register("-ml-36", "".to_string());
    selectors.register("-mt-40", "".to_string());
    selectors.register("-mr-40", "".to_string());
    selectors.register("-mb-40", "".to_string());
    selectors.register("-ml-40", "".to_string());
    selectors.register("-mt-44", "".to_string());
    selectors.register("-mr-44", "".to_string());
    selectors.register("-mb-44", "".to_string());
    selectors.register("-ml-44", "".to_string());
    selectors.register("-mt-48", "".to_string());
    selectors.register("-mr-48", "".to_string());
    selectors.register("-mb-48", "".to_string());
    selectors.register("-ml-48", "".to_string());
    selectors.register("-mt-52", "".to_string());
    selectors.register("-mr-52", "".to_string());
    selectors.register("-mb-52", "".to_string());
    selectors.register("-ml-52", "".to_string());
    selectors.register("-mt-56", "".to_string());
    selectors.register("-mr-56", "".to_string());
    selectors.register("-mb-56", "".to_string());
    selectors.register("-ml-56", "".to_string());
    selectors.register("-mt-60", "".to_string());
    selectors.register("-mr-60", "".to_string());
    selectors.register("-mb-60", "".to_string());
    selectors.register("-ml-60", "".to_string());
    selectors.register("-mt-64", "".to_string());
    selectors.register("-mr-64", "".to_string());
    selectors.register("-mb-64", "".to_string());
    selectors.register("-ml-64", "".to_string());
    selectors.register("-mt-72", "".to_string());
    selectors.register("-mr-72", "".to_string());
    selectors.register("-mb-72", "".to_string());
    selectors.register("-ml-72", "".to_string());
    selectors.register("-mt-80", "".to_string());
    selectors.register("-mr-80", "".to_string());
    selectors.register("-mb-80", "".to_string());
    selectors.register("-ml-80", "".to_string());
    selectors.register("-mt-96", "".to_string());
    selectors.register("-mr-96", "".to_string());
    selectors.register("-mb-96", "".to_string());
    selectors.register("-ml-96", "".to_string());
    selectors.register("-mt-px", "".to_string());
    selectors.register("-mr-px", "".to_string());
    selectors.register("-mb-px", "".to_string());
    selectors.register("-ml-px", "".to_string());
}*/
