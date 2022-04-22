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
    selectors.register("m-auto", "margin: auto;".to_string());
    selectors.register(
        "mx-auto",
        "margin-left: auto;
margin-right: auto;"
            .to_string(),
    );
    selectors.register(
        "my-auto",
        "margin-top: auto;
margin-botom: auto;"
            .to_string(),
    );
    selectors.register("mt-auto", "margin-top: auto;".to_string());
    selectors.register("mr-auto", "margin-right: auto;".to_string());
    selectors.register("mb-auto", "margin-bottom: auto;".to_string());
    selectors.register("ml-auto", "margin-left: auto;".to_string());
    selectors.register("space-x-0", "margin-left: 0;".to_string());
    selectors.register("space-x-0.5", "margin-left: 0.125rem;".to_string());
    selectors.register("space-x-1", "margin-left: 0.25rem;".to_string());
    selectors.register("space-x-1.5", "margin-left: 0.375rem;".to_string());
    selectors.register("space-x-2", "margin-left: 0.5rem;".to_string());
    selectors.register("space-x-2.5", "margin-left: 0.625rem;".to_string());
    selectors.register("space-x-3", "margin-left: 0.75rem;".to_string());
    selectors.register("space-x-3", "margin-left: 0.875rem;".to_string());
    selectors.register("space-x-4", "margin-left: 1rem;".to_string());
    selectors.register("space-x-5", "margin-left: 1.25rem;".to_string());
    selectors.register("space-x-6", "margin-left: 1.5rem;".to_string());
    selectors.register("space-x-7", "margin-left: 1.75rem;".to_string());
    selectors.register("space-x-8", "margin-left: 2rem;".to_string());
    selectors.register("space-x-9", "margin-left: 2.25rem;".to_string());
    selectors.register("space-x-10", "margin-left: 2.5rem;".to_string());
    selectors.register("space-x-11", "margin-left: 2.75rem;".to_string());
    selectors.register("space-x-12", "margin-left: 3rem;".to_string());
    selectors.register("space-x-14", "margin-left: 3.5rem;".to_string());
    selectors.register("space-x-16", "margin-left: 4rem;".to_string());
    selectors.register("space-x-20", "margin-left: 5rem;".to_string());
    selectors.register("space-x-24", "margin-left: 6rem;".to_string());
    selectors.register("space-x-28", "margin-left: 7rem;".to_string());
    selectors.register("space-x-32", "margin-left: 8rem;".to_string());
    selectors.register("space-x-36", "margin-left: 9rem;".to_string());
    selectors.register("space-x-40", "10rem".to_string());
    selectors.register("space-x-44", "margin-left: 11rem;".to_string());
    selectors.register("space-x-48", "margin-left: 12rem;".to_string());
    selectors.register("space-x-52", "margin-left: 13rem;".to_string());
    selectors.register("space-x-56", "margin-left: 14rem;".to_string());
    selectors.register("space-x-60", "margin-left: 15rem;".to_string());
    selectors.register("space-x-64", "margin-left: 16rem;".to_string());
    selectors.register("space-x-72", "margin-left: 18rem;".to_string());
    selectors.register("space-x-80", "margin-left: 20rem;".to_string());
    selectors.register("space-x-96", "margin-left: 24rem;".to_string());
    selectors.register("space-x-px", "margin-left: 1px;".to_string());
    selectors.register("-space-x-0", "margin-left: 0;".to_string());
    selectors.register("-space-x-0.5", "margin-left: -0.125rem;".to_string());
    selectors.register("-space-x-1", "margin-left: -0.25rem;".to_string());
    selectors.register("-space-x-1.5", "margin-left: -0.375rem;".to_string());
    selectors.register("-space-x-2", "margin-left: -0.5rem;".to_string());
    selectors.register("-space-x-2.5", "margin-left: -0.625rem;".to_string());
    selectors.register("-space-x-3", "margin-left: -0.75rem;".to_string());
    selectors.register("-space-x-3.5", "margin-left: -0.875rem;".to_string());
    selectors.register("-space-x-4", "margin-left: -1rem;".to_string());
    selectors.register("-space-x-5", "margin-left: -1.25rem;".to_string());
    selectors.register("-space-x-6", "margin-left: -1.5rem;".to_string());
    selectors.register("-space-x-7", "margin-left: -1.75rem;".to_string());
    selectors.register("-space-x-8", "margin-left: -2rem;".to_string());
    selectors.register("-space-x-9", "margin-left: -2.25rem;".to_string());
    selectors.register("-space-x-10", "margin-left: -2.5rem;".to_string());
    selectors.register("-space-x-11", "margin-left: -2.75rem;".to_string());
    selectors.register("-space-x-12", "margin-left: -3rem;".to_string());
    selectors.register("-space-x-14", "margin-left: -3.5rem;".to_string());
    selectors.register("-space-x-16", "margin-left: -4rem;".to_string());
    selectors.register("-space-x-20", "margin-left: -5rem;".to_string());
    selectors.register("-space-x-24", "margin-left: -6rem;".to_string());
    selectors.register("-space-x-28", "margin-left: -7rem;".to_string());
    selectors.register("-space-x-32", "margin-left: -8rem;".to_string());
    selectors.register("-space-x-36", "margin-left: -9rem;".to_string());
    selectors.register("-space-x-40", "margin-left: -10rem;".to_string());
    selectors.register("-space-x-44", "margin-left: -11rem;".to_string());
    selectors.register("-space-x-48", "margin-left: -12rem;".to_string());
    selectors.register("-space-x-52", "margin-left: -13rem;".to_string());
    selectors.register("-space-x-56", "margin-left: -14rem;".to_string());
    selectors.register("-space-x-60", "margin-left: -15rem;".to_string());
    selectors.register("-space-x-64", "margin-left: -16rem;".to_string());
    selectors.register("-space-x-72", "margin-left: -18rem;".to_string());
    selectors.register("-space-x-80", "margin-left: -20rem;".to_string());
    selectors.register("-space-x-96", "margin-left: -24rem;".to_string());
    selectors.register("-space-x-px", "margin-left: -1px;".to_string());
    selectors.register("space-y-0", "margin-top: 0;".to_string());
    selectors.register("space-y-0.5", "margin-top: 0.125rem;".to_string());
    selectors.register("space-y-1", "margin-top: 0.25rem;".to_string());
    selectors.register("space-y-1.5", "margin-top: 0.375rem;".to_string());
    selectors.register("space-y-2", "margin-top: 0.5rem;".to_string());
    selectors.register("space-y-2.5", "margin-top: 0.625rem;".to_string());
    selectors.register("space-y-3", "margin-top: 0.75rem;".to_string());
    selectors.register("space-y-3.5", "margin-top: 0.875rem;".to_string());
    selectors.register("space-y-4", "margin-top: 1rem;".to_string());
    selectors.register("space-y-5", "margin-top: 1.25rem;".to_string());
    selectors.register("space-y-6", "margin-top: 1.5rem;".to_string());
    selectors.register("space-y-7", "margin-top: 1.75rem;".to_string());
    selectors.register("space-y-8", "margin-top: 2rem;".to_string());
    selectors.register("space-y-9", "margin-top: 2.25rem;".to_string());
    selectors.register("space-y-10", "margin-top: 2.5rem;".to_string());
    selectors.register("space-y-11", "margin-top: 2.75rem;".to_string());
    selectors.register("space-y-12", "margin-top: 3rem;".to_string());
    selectors.register("space-y-14", "margin-top: 3.5rem;".to_string());
    selectors.register("space-y-16", "margin-top: 4rem;".to_string());
    selectors.register("space-y-20", "margin-top: 5rem;".to_string());
    selectors.register("space-y-24", "margin-top: 6rem;".to_string());
    selectors.register("space-y-28", "margin-top: 7rem;".to_string());
    selectors.register("space-y-32", "margin-top: 8rem;".to_string());
    selectors.register("space-y-36", "margin-top: 9rem;".to_string());
    selectors.register("space-y-40", "margin-top: 10rem;".to_string());
    selectors.register("space-y-44", "margin-top: 11rem;".to_string());
    selectors.register("space-y-48", "margin-top: 12rem;".to_string());
    selectors.register("space-y-52", "margin-top: 13rem;".to_string());
    selectors.register("space-y-56", "margin-top: 14rem;".to_string());
    selectors.register("space-y-60", "margin-top: 15rem;".to_string());
    selectors.register("space-y-64", "margin-top: 16rem;".to_string());
    selectors.register("space-y-72", "margin-top: 18rem;".to_string());
    selectors.register("space-y-80", "margin-top: 20rem;".to_string());
    selectors.register("space-y-96", "margin-top: 24rem;".to_string());
    selectors.register("space-y-px", "margin-top: 1px;".to_string());
    selectors.register("-space-y-0", "".to_string());
    selectors.register("-space-y-0.5", "".to_string());
    selectors.register("-space-y-1", "".to_string());
    selectors.register("-space-y-1.5", "".to_string());
    selectors.register("-space-y-2", "".to_string());
    selectors.register("-space-y-2.5", "".to_string());
    selectors.register("-space-y-3", "".to_string());
    selectors.register("-space-y-3", "".to_string());
    selectors.register("-space-y-4", "".to_string());
    selectors.register("-space-y-5", "".to_string());
    selectors.register("-space-y-6", "".to_string());
    selectors.register("-space-y-7", "".to_string());
    selectors.register("-space-y-8", "".to_string());
    selectors.register("-space-y-9", "".to_string());
    selectors.register("-space-y-10", "".to_string());
    selectors.register("-space-y-11", "".to_string());
    selectors.register("-space-y-12", "".to_string());
    selectors.register("-space-y-14", "".to_string());
    selectors.register("-space-y-16", "".to_string());
    selectors.register("-space-y-20", "".to_string());
    selectors.register("-space-y-24", "".to_string());
    selectors.register("-space-y-28", "".to_string());
    selectors.register("-space-y-32", "".to_string());
    selectors.register("-space-y-36", "".to_string());
    selectors.register("-space-y-40", "".to_string());
    selectors.register("-space-y-44", "".to_string());
    selectors.register("-space-y-48", "".to_string());
    selectors.register("-space-y-52", "".to_string());
    selectors.register("-space-y-56", "".to_string());
    selectors.register("-space-y-60", "".to_string());
    selectors.register("-space-y-64", "".to_string());
    selectors.register("-space-y-72", "".to_string());
    selectors.register("-space-y-80", "".to_string());
    selectors.register("-space-y-96", "".to_string());
    selectors.register("-space-y-px", "".to_string());
    selectors.register("space-x-reverse", "".to_string());
    selectors.register("space-y-reverse", "".to_string());
}*/
