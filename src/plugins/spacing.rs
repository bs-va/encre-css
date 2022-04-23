use super::Plugin;
use crate::utils::{default_lengths, value_matchers::*};

// TODO: Boilerplate generator (just one structure for padding and another for margin)
#[derive(Debug)]
pub struct SpacingPaddingPlugin;

impl Plugin for SpacingPaddingPlugin {
    fn namespace(&self) -> String {
        "p".to_string()
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str) -> String {
        format!("padding: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        default_lengths::get_basic(modifier).map(|c| self.css_template_value(&c))
    }
}

#[derive(Debug)]
pub struct SpacingPaddingXPlugin;

impl Plugin for SpacingPaddingXPlugin {
    fn namespace(&self) -> String {
        "px".to_string()
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str) -> String {
        format!(
            "padding-left: {val};
  padding-right: {val};"
        )
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        default_lengths::get_basic(modifier).map(|c| self.css_template_value(&c))
    }
}

#[derive(Debug)]
pub struct SpacingPaddingYPlugin;

impl Plugin for SpacingPaddingYPlugin {
    fn namespace(&self) -> String {
        "py".to_string()
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str) -> String {
        format!(
            "padding-top: {val};
  padding-bottom: {val};"
        )
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        default_lengths::get_basic(modifier).map(|c| self.css_template_value(&c))
    }
}

#[derive(Debug)]
pub struct SpacingPaddingLeftPlugin;

impl Plugin for SpacingPaddingLeftPlugin {
    fn namespace(&self) -> String {
        "pl".to_string()
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str) -> String {
        format!("padding-left: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        default_lengths::get_basic(modifier).map(|c| self.css_template_value(&c))
    }
}

#[derive(Debug)]
pub struct SpacingPaddingRightPlugin;

impl Plugin for SpacingPaddingRightPlugin {
    fn namespace(&self) -> String {
        "pr".to_string()
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str) -> String {
        format!("padding-right: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        default_lengths::get_basic(modifier).map(|c| self.css_template_value(&c))
    }
}

#[derive(Debug)]
pub struct SpacingPaddingTopPlugin;

impl Plugin for SpacingPaddingTopPlugin {
    fn namespace(&self) -> String {
        "pt".to_string()
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str) -> String {
        format!("padding-top: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        default_lengths::get_basic(modifier).map(|c| self.css_template_value(&c))
    }
}

#[derive(Debug)]
pub struct SpacingPaddingBottomPlugin;

impl Plugin for SpacingPaddingBottomPlugin {
    fn namespace(&self) -> String {
        "pb".to_string()
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str) -> String {
        format!("padding-bottom: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        default_lengths::get_basic(modifier).map(|c| self.css_template_value(&c))
    }
}

// Margin

#[derive(Debug)]
pub struct SpacingMarginPlugin;

impl Plugin for SpacingMarginPlugin {
    fn namespace(&self) -> String {
        "m".to_string()
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str) -> String {
        format!("margin: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        if is_matching_auto(modifier) {
            return Some(self.css_template_value("auto"));
        }

        default_lengths::get_basic(modifier).map(|c| self.css_template_value(&c))
    }
}

#[derive(Debug)]
pub struct SpacingMarginXPlugin;

impl Plugin for SpacingMarginXPlugin {
    fn namespace(&self) -> String {
        "mx".to_string()
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str) -> String {
        format!(
            "margin-left: {val};
  margin-right: {val};"
        )
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        if is_matching_auto(modifier) {
            return Some(self.css_template_value("auto"));
        }

        default_lengths::get_basic(modifier).map(|c| self.css_template_value(&c))
    }
}

#[derive(Debug)]
pub struct SpacingMarginYPlugin;

impl Plugin for SpacingMarginYPlugin {
    fn namespace(&self) -> String {
        "my".to_string()
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str) -> String {
        format!(
            "margin-top: {val};
  margin-bottom: {val};"
        )
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        if is_matching_auto(modifier) {
            return Some(self.css_template_value("auto"));
        }

        default_lengths::get_basic(modifier).map(|c| self.css_template_value(&c))
    }
}

#[derive(Debug)]
pub struct SpacingMarginLeftPlugin;

impl Plugin for SpacingMarginLeftPlugin {
    fn namespace(&self) -> String {
        "ml".to_string()
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str) -> String {
        format!("margin-left: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        if is_matching_auto(modifier) {
            return Some(self.css_template_value("auto"));
        }

        default_lengths::get_basic(modifier).map(|c| self.css_template_value(&c))
    }
}

#[derive(Debug)]
pub struct SpacingMarginRightPlugin;

impl Plugin for SpacingMarginRightPlugin {
    fn namespace(&self) -> String {
        "mr".to_string()
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str) -> String {
        format!("margin-right: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        if is_matching_auto(modifier) {
            return Some(self.css_template_value("auto"));
        }

        default_lengths::get_basic(modifier).map(|c| self.css_template_value(&c))
    }
}

#[derive(Debug)]
pub struct SpacingMarginTopPlugin;

impl Plugin for SpacingMarginTopPlugin {
    fn namespace(&self) -> String {
        "mt".to_string()
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str) -> String {
        format!("margin-top: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        if is_matching_auto(modifier) {
            return Some(self.css_template_value("auto"));
        }

        default_lengths::get_basic(modifier).map(|c| self.css_template_value(&c))
    }
}

#[derive(Debug)]
pub struct SpacingMarginBottomPlugin;

impl Plugin for SpacingMarginBottomPlugin {
    fn namespace(&self) -> String {
        "mb".to_string()
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str) -> String {
        format!("margin-bottom: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        if is_matching_auto(modifier) {
            return Some(self.css_template_value("auto"));
        }

        default_lengths::get_basic(modifier).map(|c| self.css_template_value(&c))
    }
}

#[derive(Debug)]
pub struct SpacingSpaceXPlugin;

impl Plugin for SpacingSpaceXPlugin {
    fn namespace(&self) -> String {
        "space-x".to_string()
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str) -> String {
        format!("margin-left: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        if modifier == "reverse" {
            return Some("--tw-space-x-reverse: 1;".to_string());
        }

        default_lengths::get_basic(modifier).map(|c| self.css_template_value(&c))
    }
}

#[derive(Debug)]
pub struct SpacingSpaceYPlugin;

impl Plugin for SpacingSpaceYPlugin {
    fn namespace(&self) -> String {
        "space-y".to_string()
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str) -> String {
        format!("margin-top: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        if modifier == "reverse" {
            return Some("--tw-space-y-reverse: 1;".to_string());
        }

        default_lengths::get_basic(modifier).map(|c| self.css_template_value(&c))
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
