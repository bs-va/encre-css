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
        if modifier == "inherit" {
            return Some(self.css_template_value("inherit"));
        }

        default_colors::get(modifier).map(|c| self.css_template_value(&c))
    }
}

#[derive(Debug)]
pub struct BorderWidthPlugin;

impl Plugin for BorderWidthPlugin {
    fn namespace(&self) -> String {
        "border".to_string()
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str) -> String {
        format!("border-width: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        if modifier.is_empty() {
            return Some(self.css_template_value("1px"));
        }

        // NOTE: Not-compatible with TailwindCSS, support all values
        if let Ok(width) = modifier.parse::<usize>() {
            Some(self.css_template_value(&format!("{width}px")))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct BorderWidthXPlugin;

impl Plugin for BorderWidthXPlugin {
    fn namespace(&self) -> String {
        "border-x".to_string()
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str) -> String {
        format!(
            "border-left-width: {val};
  border-right-width: {val};"
        )
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        if modifier.is_empty() {
            return Some(self.css_template_value("1px"));
        }

        // NOTE: Not-compatible with TailwindCSS, support all values
        if let Ok(width) = modifier.parse::<usize>() {
            Some(self.css_template_value(&format!("{width}px")))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct BorderWidthYPlugin;

impl Plugin for BorderWidthYPlugin {
    fn namespace(&self) -> String {
        "border-y".to_string()
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str) -> String {
        format!(
            "border-top-width: {val};
  border-bottom-width: {val};"
        )
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        if modifier.is_empty() {
            return Some(self.css_template_value("1px"));
        }

        // NOTE: Not-compatible with TailwindCSS, support all values
        if let Ok(width) = modifier.parse::<usize>() {
            Some(self.css_template_value(&format!("{width}px")))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct BorderWidthTopPlugin;

impl Plugin for BorderWidthTopPlugin {
    fn namespace(&self) -> String {
        "border-t".to_string()
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str) -> String {
        format!("border-top-width: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        if modifier.is_empty() {
            return Some(self.css_template_value("1px"));
        }

        // NOTE: Not-compatible with TailwindCSS, support all values
        if let Ok(width) = modifier.parse::<usize>() {
            Some(self.css_template_value(&format!("{width}px")))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct BorderWidthBottomPlugin;

impl Plugin for BorderWidthBottomPlugin {
    fn namespace(&self) -> String {
        "border-b".to_string()
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str) -> String {
        format!("border-bottom-width: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        if modifier.is_empty() {
            return Some(self.css_template_value("1px"));
        }

        // NOTE: Not-compatible with TailwindCSS, support all values
        if let Ok(width) = modifier.parse::<usize>() {
            Some(self.css_template_value(&format!("{width}px")))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct BorderWidthLeftPlugin;

impl Plugin for BorderWidthLeftPlugin {
    fn namespace(&self) -> String {
        "border-l".to_string()
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str) -> String {
        format!("border-left-width: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        if modifier.is_empty() {
            return Some(self.css_template_value("1px"));
        }

        // NOTE: Not-compatible with TailwindCSS, support all values
        if let Ok(width) = modifier.parse::<usize>() {
            Some(self.css_template_value(&format!("{width}px")))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct BorderWidthRightPlugin;

impl Plugin for BorderWidthRightPlugin {
    fn namespace(&self) -> String {
        "border-r".to_string()
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str) -> String {
        format!("border-right-width: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        if modifier.is_empty() {
            return Some(self.css_template_value("1px"));
        }

        // NOTE: Not-compatible with TailwindCSS, support all values
        if let Ok(width) = modifier.parse::<usize>() {
            Some(self.css_template_value(&format!("{width}px")))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct BorderOpacityPlugin;

impl Plugin for BorderOpacityPlugin {
    fn namespace(&self) -> String {
        "border-opacity".to_string()
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        // NOTE: Not-compatible with TailwindCSS, support all values
        if let Ok(opacity_value) = modifier.parse::<f32>() {
            Some(format!("--tw-border-opacity: {};", opacity_value / 100.))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct BorderRadiusPlugin;

impl Plugin for BorderRadiusPlugin {
    fn namespace(&self) -> String {
        "rounded".to_string()
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        val.split('_')
            .all(|v| is_matching_length(v) || is_matching_percentage(v))
    }

    fn css_template_value(&self, val: &str) -> String {
        format!("border-radius: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        match modifier {
            "" => Some(self.css_template_value("0.25rem")),
            "none" => Some(self.css_template_value("0")),
            "sm" => Some(self.css_template_value("0.125rem")),
            "md" => Some(self.css_template_value("0.375rem")),
            "lg" => Some(self.css_template_value("0.5rem")),
            "xl" => Some(self.css_template_value("0.75rem")),
            "2xl" => Some(self.css_template_value("1rem")),
            "3xl" => Some(self.css_template_value("1.5rem")),
            "full" => Some(self.css_template_value("9999px")),
            "t-none" => Some(self.css_template_value("")),
            "r-none" => Some(self.css_template_value("")),
            "b-none" => Some(self.css_template_value("")),
            "l-none" => Some(self.css_template_value("")),
            "t-sm" => Some(self.css_template_value("")),
            "r-sm" => Some(self.css_template_value("")),
            "b-sm" => Some(self.css_template_value("")),
            "l-sm" => Some(self.css_template_value("")),
            "t" => Some(self.css_template_value("")),
            "r" => Some(self.css_template_value("")),
            "b" => Some(self.css_template_value("")),
            "l" => Some(self.css_template_value("")),
            "t-md" => Some(self.css_template_value("")),
            "r-md" => Some(self.css_template_value("")),
            "b-md" => Some(self.css_template_value("")),
            "l-md" => Some(self.css_template_value("")),
            "t-lg" => Some(self.css_template_value("")),
            "r-lg" => Some(self.css_template_value("")),
            "b-lg" => Some(self.css_template_value("")),
            "l-lg" => Some(self.css_template_value("")),
            "t-xl" => Some(self.css_template_value("")),
            "r-xl" => Some(self.css_template_value("")),
            "b-xl" => Some(self.css_template_value("")),
            "l-xl" => Some(self.css_template_value("")),
            "t-2xl" => Some(self.css_template_value("")),
            "r-2xl" => Some(self.css_template_value("")),
            "b-2xl" => Some(self.css_template_value("")),
            "l-2xl" => Some(self.css_template_value("")),
            "t-3xl" => Some(self.css_template_value("")),
            "r-3xl" => Some(self.css_template_value("")),
            "b-3xl" => Some(self.css_template_value("")),
            "l-3xl" => Some(self.css_template_value("")),
            "t-full" => Some(self.css_template_value("")),
            "r-full" => Some(self.css_template_value("")),
            "b-full" => Some(self.css_template_value("")),
            "l-full" => Some(self.css_template_value("")),
            "tl-none" => Some(self.css_template_value("")),
            "tr-none" => Some(self.css_template_value("")),
            "br-none" => Some(self.css_template_value("")),
            "bl-none" => Some(self.css_template_value("")),
            "tl-sm" => Some(self.css_template_value("")),
            "tr-sm" => Some(self.css_template_value("")),
            "br-sm" => Some(self.css_template_value("")),
            "bl-sm" => Some(self.css_template_value("")),
            "tl" => Some(self.css_template_value("")),
            "tr" => Some(self.css_template_value("")),
            "br" => Some(self.css_template_value("")),
            "bl" => Some(self.css_template_value("")),
            "tl-md" => Some(self.css_template_value("")),
            "tr-md" => Some(self.css_template_value("")),
            "br-md" => Some(self.css_template_value("")),
            "bl-md" => Some(self.css_template_value("")),
            "tl-lg" => Some(self.css_template_value("")),
            "tr-lg" => Some(self.css_template_value("")),
            "br-lg" => Some(self.css_template_value("")),
            "bl-lg" => Some(self.css_template_value("")),
            "tl-xl" => Some(self.css_template_value("")),
            "tr-xl" => Some(self.css_template_value("")),
            "br-xl" => Some(self.css_template_value("")),
            "bl-xl" => Some(self.css_template_value("")),
            "tl-2xl" => Some(self.css_template_value("")),
            "tr-2xl" => Some(self.css_template_value("")),
            "br-2xl" => Some(self.css_template_value("")),
            "bl-2xl" => Some(self.css_template_value("")),
            "tl-3xl" => Some(self.css_template_value("")),
            "tr-3xl" => Some(self.css_template_value("")),
            "br-3xl" => Some(self.css_template_value("")),
            "bl-3xl" => Some(self.css_template_value("")),
            "tl-full" => Some(self.css_template_value("")),
            "tr-full" => Some(self.css_template_value("")),
            "br-full" => Some(self.css_template_value("")),
            "bl-full" => Some(self.css_template_value("")),
            _ => None,
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
        if modifier == "inherit" {
            return Some(self.css_template_value("inherit"));
        }

        default_colors::get(modifier).map(|c| self.css_template_value(&c))
    }
}

#[derive(Debug)]
pub struct DivideOpacityPlugin;

impl Plugin for DivideOpacityPlugin {
    fn namespace(&self) -> String {
        "divide-opacity".to_string()
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        // NOTE: Not-compatible with TailwindCSS, support all values
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
        if modifier == "inherit" {
            return Some(self.css_template_value("inherit"));
        }

        default_colors::get(modifier).map(|c| self.css_template_value(&c))
    }
}

#[derive(Debug)]
pub struct RingOpacityPlugin;

impl Plugin for RingOpacityPlugin {
    fn namespace(&self) -> String {
        "ring-opacity".to_string()
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        // NOTE: Not-compatible with TailwindCSS, support all values
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
        if modifier == "inherit" {
            return Some(self.css_template_value("inherit"));
        }

        default_colors::get(modifier).map(|c| self.css_template_value(&c))
    }
}

#[derive(Debug)]
pub struct RingOffsetOpacityPlugin;

impl Plugin for RingOffsetOpacityPlugin {
    fn namespace(&self) -> String {
        "ring-offset-opacity".to_string()
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        // NOTE: Not-compatible with TailwindCSS, support all values
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
