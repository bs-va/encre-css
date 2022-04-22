use super::Plugin;
use crate::utils::{default_colors, value_matchers::*};

#[derive(Debug)]
pub struct BorderColorPlugin;

impl Plugin for BorderColorPlugin {
    fn namespace(&self) -> String {
        "border".to_string()
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "color" || is_matching_color(val)
    }

    fn css_template_value(&self, val: &str) -> String {
        if val.contains("--tw-opacity") {
            format!(
                "--tw-border-opacity: 1;
  border-color: {};",
                val.replace("--tw-opacity", "--tw-border-opacity")
            )
        } else {
            format!("border-color: {val};")
        }
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        default_colors::get(modifier).map(|c| self.css_template_value(&c))
    }
}

#[derive(Debug)]
pub struct BorderOpacityPlugin;

impl Plugin for BorderOpacityPlugin {
    fn namespace(&self) -> String {
        "border-opacity".to_string()
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        // Support all values
        if let Ok(opacity_value) = modifier.parse::<f32>() {
            Some(format!("--tw-border-opacity: {};", opacity_value / 100.))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct DivideColorPlugin;

impl Plugin for DivideColorPlugin {
    fn namespace(&self) -> String {
        "divide".to_string()
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "color" || is_matching_color(val)
    }

    fn css_template_value(&self, val: &str) -> String {
        if val.contains("--tw-opacity") {
            format!(
                "--tw-divide-opacity: 1;
  border-color: {};",
                val.replace("--tw-opacity", "--tw-divide-opacity")
            )
        } else {
            format!("border-color: {val};")
        }
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        default_colors::get(modifier).map(|c| self.css_template_value(&c))
    }
}

#[derive(Debug)]
pub struct DivideOpacityPlugin;

impl Plugin for DivideOpacityPlugin {
    fn namespace(&self) -> String {
        "divide".to_string()
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        // Support all values
        if let Ok(opacity_value) = modifier.parse::<f32>() {
            Some(format!("--tw-divide-opacity: {};", opacity_value / 100.))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct RingColorPlugin;

impl Plugin for RingColorPlugin {
    fn namespace(&self) -> String {
        "ring".to_string()
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "color" || is_matching_color(val)
    }

    fn css_template_value(&self, val: &str) -> String {
        if val.contains("--tw-opacity") {
            format!(
                "--tw-ring-opacity: 1;
  --ring-color: {};",
                val.replace("--tw-opacity", "--tw-ring-opacity")
            )
        } else {
            format!("--ring-color: {val};")
        }
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        default_colors::get(modifier).map(|c| self.css_template_value(&c))
    }
}

#[derive(Debug)]
pub struct RingOpacityPlugin;

impl Plugin for RingOpacityPlugin {
    fn namespace(&self) -> String {
        "ring".to_string()
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        // Support all values
        if let Ok(opacity_value) = modifier.parse::<f32>() {
            Some(format!("--tw-ring-opacity: {};", opacity_value / 100.))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct RingOffsetColorPlugin;

impl Plugin for RingOffsetColorPlugin {
    fn namespace(&self) -> String {
        "ring-offset".to_string()
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "color" || is_matching_color(val)
    }

    fn css_template_value(&self, val: &str) -> String {
        if val.contains("--tw-opacity") {
            format!(
                "--tw-ring-offset-opacity: 1;
  --ring-offset-color: {};",
                val.replace("--tw-opacity", "--tw-ring-offset-opacity")
            )
        } else {
            format!("--ring-offset-color: {val};")
        }
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        default_colors::get(modifier).map(|c| self.css_template_value(&c))
    }
}

#[derive(Debug)]
pub struct RingOffsetOpacityPlugin;

impl Plugin for RingOffsetOpacityPlugin {
    fn namespace(&self) -> String {
        "ring-offset".to_string()
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        // Support all values
        if let Ok(opacity_value) = modifier.parse::<f32>() {
            Some(format!(
                "--tw-ring-offset-opacity: {};",
                opacity_value / 100.
            ))
        } else {
            None
        }
    }
}

/*pub fn init(selectors: &mut SelectorList) {
    selectors.register("rounded-none", "border-radius: 0;".to_string());
    selectors.register("rounded-sm", "border-radius: 0.125rem;".to_string());
    selectors.register("rounded", "border-radius: 0.25rem;".to_string());
    selectors.register("rounded-md", "border-radius: 0.375rem;".to_string());
    selectors.register("rounded-lg", "border-radius: 0.5rem;".to_string());
    selectors.register("rounded-xl", "border-radius: 0.75rem;".to_string());
    selectors.register("rounded-2xl", "border-radius: 1rem;".to_string());
    selectors.register("rounded-3xl", "border-radius: 1.5rem;".to_string());
    selectors.register("rounded-full", "border-radius: 9999px;".to_string());
    selectors.register("rounded-t-none", "".to_string());
    selectors.register("rounded-r-none", "".to_string());
    selectors.register("rounded-b-none", "".to_string());
    selectors.register("rounded-l-none", "".to_string());
    selectors.register("rounded-t-sm", "".to_string());
    selectors.register("rounded-r-sm", "".to_string());
    selectors.register("rounded-b-sm", "".to_string());
    selectors.register("rounded-l-sm", "".to_string());
    selectors.register("rounded-t", "".to_string());
    selectors.register("rounded-r", "".to_string());
    selectors.register("rounded-b", "".to_string());
    selectors.register("rounded-l", "".to_string());
    selectors.register("rounded-t-md", "".to_string());
    selectors.register("rounded-r-md", "".to_string());
    selectors.register("rounded-b-md", "".to_string());
    selectors.register("rounded-l-md", "".to_string());
    selectors.register("rounded-t-lg", "".to_string());
    selectors.register("rounded-r-lg", "".to_string());
    selectors.register("rounded-b-lg", "".to_string());
    selectors.register("rounded-l-lg", "".to_string());
    selectors.register("rounded-t-xl", "".to_string());
    selectors.register("rounded-r-xl", "".to_string());
    selectors.register("rounded-b-xl", "".to_string());
    selectors.register("rounded-l-xl", "".to_string());
    selectors.register("rounded-t-2xl", "".to_string());
    selectors.register("rounded-r-2xl", "".to_string());
    selectors.register("rounded-b-2xl", "".to_string());
    selectors.register("rounded-l-2xl", "".to_string());
    selectors.register("rounded-t-3xl", "".to_string());
    selectors.register("rounded-r-3xl", "".to_string());
    selectors.register("rounded-b-3xl", "".to_string());
    selectors.register("rounded-l-3xl", "".to_string());
    selectors.register("rounded-t-full", "".to_string());
    selectors.register("rounded-r-full", "".to_string());
    selectors.register("rounded-b-full", "".to_string());
    selectors.register("rounded-l-full", "".to_string());
    selectors.register("rounded-tl-none", "".to_string());
    selectors.register("rounded-tr-none", "".to_string());
    selectors.register("rounded-br-none", "".to_string());
    selectors.register("rounded-bl-none", "".to_string());
    selectors.register("rounded-tl-sm", "".to_string());
    selectors.register("rounded-tr-sm", "".to_string());
    selectors.register("rounded-br-sm", "".to_string());
    selectors.register("rounded-bl-sm", "".to_string());
    selectors.register("rounded-tl", "".to_string());
    selectors.register("rounded-tr", "".to_string());
    selectors.register("rounded-br", "".to_string());
    selectors.register("rounded-bl", "".to_string());
    selectors.register("rounded-tl-md", "".to_string());
    selectors.register("rounded-tr-md", "".to_string());
    selectors.register("rounded-br-md", "".to_string());
    selectors.register("rounded-bl-md", "".to_string());
    selectors.register("rounded-tl-lg", "".to_string());
    selectors.register("rounded-tr-lg", "".to_string());
    selectors.register("rounded-br-lg", "".to_string());
    selectors.register("rounded-bl-lg", "".to_string());
    selectors.register("rounded-tl-xl", "".to_string());
    selectors.register("rounded-tr-xl", "".to_string());
    selectors.register("rounded-br-xl", "".to_string());
    selectors.register("rounded-bl-xl", "".to_string());
    selectors.register("rounded-tl-2xl", "".to_string());
    selectors.register("rounded-tr-2xl", "".to_string());
    selectors.register("rounded-br-2xl", "".to_string());
    selectors.register("rounded-bl-2xl", "".to_string());
    selectors.register("rounded-tl-3xl", "".to_string());
    selectors.register("rounded-tr-3xl", "".to_string());
    selectors.register("rounded-br-3xl", "".to_string());
    selectors.register("rounded-bl-3xl", "".to_string());
    selectors.register("rounded-tl-full", "".to_string());
    selectors.register("rounded-tr-full", "".to_string());
    selectors.register("rounded-br-full", "".to_string());
    selectors.register("rounded-bl-full", "".to_string());
    selectors.register("border", "border-width: 1px;".to_string());
    selectors.register("border-0", "border-width: 0;".to_string());
    selectors.register("border-2", "border-width: 2px;".to_string());
    selectors.register("border-4", "border-width: 4px;".to_string());
    selectors.register("border-8", "border-width: 8px;".to_string());
    selectors.register("border-t", "border-top-width: 1px;".to_string());
    selectors.register("border-t-0", "border-top-width: 0;".to_string());
    selectors.register("border-t-2", "border-top-width: 2px;".to_string());
    selectors.register("border-t-4", "border-top-width: 4px;".to_string());
    selectors.register("border-t-8", "border-top-width: 8px;".to_string());
    selectors.register("border-r", "border-right-width: 1px;".to_string());
    selectors.register("border-r-0", "border-right-width: 0;".to_string());
    selectors.register("border-r-2", "border-right-width: 2px;".to_string());
    selectors.register("border-r-4", "border-right-width: 4px;".to_string());
    selectors.register("border-r-8", "border-right-width: 8px;".to_string());
    selectors.register("border-b", "border-bottom-width: 1px;".to_string());
    selectors.register("border-b-0", "border-bottom-width: 0;".to_string());
    selectors.register("border-b-2", "border-bottom-width: 2px;".to_string());
    selectors.register("border-b-4", "border-bottom-width: 4px;".to_string());
    selectors.register("border-b-8", "border-bottom-width: 8px;".to_string());
    selectors.register("border-l", "border-left-width: 1px;".to_string());
    selectors.register("border-l-0", "border-left-width: 0;".to_string());
    selectors.register("border-l-2", "border-left-width: 2px;".to_string());
    selectors.register("border-l-4", "border-left-width: 4px;".to_string());
    selectors.register("border-l-8", "border-left-width: 8px;".to_string());
    selectors.register("border-solid", "".to_string());
    selectors.register("border-dashed", "".to_string());
    selectors.register("border-dotted", "".to_string());
    selectors.register("border-double", "".to_string());
    selectors.register("border-none", "".to_string());
    selectors.register("divide-x-0", "".to_string());
    selectors.register("divide-x-2", "".to_string());
    selectors.register("divide-x-4", "".to_string());
    selectors.register("divide-x-8", "".to_string());
    selectors.register("divide-x", "".to_string());
    selectors.register("divide-y-0", "".to_string());
    selectors.register("divide-y-2", "".to_string());
    selectors.register("divide-y-4", "".to_string());
    selectors.register("divide-y-8", "".to_string());
    selectors.register("divide-y", "".to_string());
    selectors.register("divide-x-reverse", "".to_string());
    selectors.register("divide-y-reverse", "".to_string());
    selectors.register("divide-solid", "".to_string());
    selectors.register("divide-dashed", "".to_string());
    selectors.register("divide-dotted", "".to_string());
    selectors.register("divide-double", "".to_string());
    selectors.register("divide-none", "".to_string());
    selectors.register("ring-0", "box-shadow: var(--tw-ring-inset) 0 0 0 calc(0px + var(--tw-ring-offset-width)) var(--tw-ring-color);".to_string());
    selectors.register("ring-1", "box-shadow: var(--tw-ring-inset) 0 0 0 calc(1px + var(--tw-ring-offset-width)) var(--tw-ring-color);".to_string());
    selectors.register("ring-2", "box-shadow: var(--tw-ring-inset) 0 0 0 calc(2px + var(--tw-ring-offset-width)) var(--tw-ring-color);".to_string());
    selectors.register("ring-4", "box-shadow: var(--tw-ring-inset) 0 0 0 calc(4px + var(--tw-ring-offset-width)) var(--tw-ring-color);".to_string());
    selectors.register("ring-8", "box-shadow: var(--tw-ring-inset) 0 0 0 calc(8px + var(--tw-ring-offset-width)) var(--tw-ring-color);".to_string());
    selectors.register("ring", "box-shadow: var(--tw-ring-inset) 0 0 0 calc(3px + var(--tw-ring-offset-width)) var(--tw-ring-color);".to_string());
    selectors.register("ring-inset", "--tw-ring-inset: inset;".to_string());
    selectors.register("ring-offset-0", "".to_string());
    selectors.register("ring-offset-1", "".to_string());
    selectors.register("ring-offset-2", "".to_string());
    selectors.register("ring-offset-4", "".to_string());
    selectors.register("ring-offset-8", "".to_string());
}*/
