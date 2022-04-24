use super::Plugin;
use crate::utils::{default_colors, value_matchers::*};

use std::fmt::{Result, Write};

#[derive(Debug)]
pub struct ColorPlugin;

impl Plugin for ColorPlugin {
    fn namespace(&self) -> &str {
        "border"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "color" || is_matching_color(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        if val.contains("--tw-opacity") {
            write!(css_content,
                "--tw-border-opacity: 1;
  border-color: {};",
                val.replace("--tw-opacity", "--tw-border-opacity")
            )
        } else {
            write!(css_content, "border-color: {val};")
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
pub struct WidthPlugin;

impl Plugin for WidthPlugin {
    fn namespace(&self) -> &str {
        "border"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "border-width: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if modifier.is_empty() {
            return self.css_template_value("1px", css_content);
        }

        // NOTE: Not-compatible with TailwindCSS, support all values
        if let Ok(width) = modifier.parse::<usize>() {
            self.css_template_value(&format!("{width}px"), css_content)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct WidthXPlugin;

impl Plugin for WidthXPlugin {
    fn namespace(&self) -> &str {
        "border-x"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content,
            "border-left-width: {val};
  border-right-width: {val};"
        )
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if modifier.is_empty() {
            return self.css_template_value("1px", css_content);
        }

        // NOTE: Not-compatible with TailwindCSS, support all values
        if let Ok(width) = modifier.parse::<usize>() {
            self.css_template_value(&format!("{width}px"), css_content)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct WidthYPlugin;

impl Plugin for WidthYPlugin {
    fn namespace(&self) -> &str {
        "border-y"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content,
            "border-top-width: {val};
  border-bottom-width: {val};"
        )
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if modifier.is_empty() {
            return self.css_template_value("1px", css_content);
        }

        // NOTE: Not-compatible with TailwindCSS, support all values
        if let Ok(width) = modifier.parse::<usize>() {
            self.css_template_value(&format!("{width}px"), css_content)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct WidthTopPlugin;

impl Plugin for WidthTopPlugin {
    fn namespace(&self) -> &str {
        "border-t"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "border-top-width: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if modifier.is_empty() {
            return self.css_template_value("1px", css_content);
        }

        // NOTE: Not-compatible with TailwindCSS, support all values
        if let Ok(width) = modifier.parse::<usize>() {
            self.css_template_value(&format!("{width}px"), css_content)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct WidthBottomPlugin;

impl Plugin for WidthBottomPlugin {
    fn namespace(&self) -> &str {
        "border-b"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "border-bottom-width: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if modifier.is_empty() {
            return self.css_template_value("1px", css_content);
        }

        // NOTE: Not-compatible with TailwindCSS, support all values
        if let Ok(width) = modifier.parse::<usize>() {
            self.css_template_value(&format!("{width}px"), css_content)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct WidthLeftPlugin;

impl Plugin for WidthLeftPlugin {
    fn namespace(&self) -> &str {
        "border-l"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "border-left-width: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if modifier.is_empty() {
            return self.css_template_value("1px", css_content);
        }

        // NOTE: Not-compatible with TailwindCSS, support all values
        if let Ok(width) = modifier.parse::<usize>() {
            self.css_template_value(&format!("{width}px"), css_content)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct WidthRightPlugin;

impl Plugin for WidthRightPlugin {
    fn namespace(&self) -> &str {
        "border-r"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "border-right-width: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if modifier.is_empty() {
            return self.css_template_value("1px", css_content);
        }

        // NOTE: Not-compatible with TailwindCSS, support all values
        if let Ok(width) = modifier.parse::<usize>() {
            self.css_template_value(&format!("{width}px"), css_content)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct OpacityPlugin;

impl Plugin for OpacityPlugin {
    fn namespace(&self) -> &str {
        "border-opacity"
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        // NOTE: Not-compatible with TailwindCSS, support all values
        if let Ok(opacity_value) = modifier.parse::<f32>() {
            write!(css_content, "--tw-border-opacity: {};", opacity_value / 100.)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct RadiusPlugin;

impl Plugin for RadiusPlugin {
    fn namespace(&self) -> &str {
        "rounded"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        val.split('_')
            .all(|v| is_matching_length(v) || is_matching_percentage(v))
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "border-radius: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        match modifier {
            "" => self.css_template_value("0.25rem", css_content),
            "none" => self.css_template_value("0", css_content),
            "sm" => self.css_template_value("0.125rem", css_content),
            "md" => self.css_template_value("0.375rem", css_content),
            "lg" => self.css_template_value("0.5rem", css_content),
            "xl" => self.css_template_value("0.75rem", css_content),
            "2xl" => self.css_template_value("1rem", css_content),
            "3xl" => self.css_template_value("1.5rem", css_content),
            "full" => self.css_template_value("9999px", css_content),
            "t-none" => self.css_template_value("", css_content),
            "r-none" => self.css_template_value("", css_content),
            "b-none" => self.css_template_value("", css_content),
            "l-none" => self.css_template_value("", css_content),
            "t-sm" => self.css_template_value("", css_content),
            "r-sm" => self.css_template_value("", css_content),
            "b-sm" => self.css_template_value("", css_content),
            "l-sm" => self.css_template_value("", css_content),
            "t" => self.css_template_value("", css_content),
            "r" => self.css_template_value("", css_content),
            "b" => self.css_template_value("", css_content),
            "l" => self.css_template_value("", css_content),
            "t-md" => self.css_template_value("", css_content),
            "r-md" => self.css_template_value("", css_content),
            "b-md" => self.css_template_value("", css_content),
            "l-md" => self.css_template_value("", css_content),
            "t-lg" => self.css_template_value("", css_content),
            "r-lg" => self.css_template_value("", css_content),
            "b-lg" => self.css_template_value("", css_content),
            "l-lg" => self.css_template_value("", css_content),
            "t-xl" => self.css_template_value("", css_content),
            "r-xl" => self.css_template_value("", css_content),
            "b-xl" => self.css_template_value("", css_content),
            "l-xl" => self.css_template_value("", css_content),
            "t-2xl" => self.css_template_value("", css_content),
            "r-2xl" => self.css_template_value("", css_content),
            "b-2xl" => self.css_template_value("", css_content),
            "l-2xl" => self.css_template_value("", css_content),
            "t-3xl" => self.css_template_value("", css_content),
            "r-3xl" => self.css_template_value("", css_content),
            "b-3xl" => self.css_template_value("", css_content),
            "l-3xl" => self.css_template_value("", css_content),
            "t-full" => self.css_template_value("", css_content),
            "r-full" => self.css_template_value("", css_content),
            "b-full" => self.css_template_value("", css_content),
            "l-full" => self.css_template_value("", css_content),
            "tl-none" => self.css_template_value("", css_content),
            "tr-none" => self.css_template_value("", css_content),
            "br-none" => self.css_template_value("", css_content),
            "bl-none" => self.css_template_value("", css_content),
            "tl-sm" => self.css_template_value("", css_content),
            "tr-sm" => self.css_template_value("", css_content),
            "br-sm" => self.css_template_value("", css_content),
            "bl-sm" => self.css_template_value("", css_content),
            "tl" => self.css_template_value("", css_content),
            "tr" => self.css_template_value("", css_content),
            "br" => self.css_template_value("", css_content),
            "bl" => self.css_template_value("", css_content),
            "tl-md" => self.css_template_value("", css_content),
            "tr-md" => self.css_template_value("", css_content),
            "br-md" => self.css_template_value("", css_content),
            "bl-md" => self.css_template_value("", css_content),
            "tl-lg" => self.css_template_value("", css_content),
            "tr-lg" => self.css_template_value("", css_content),
            "br-lg" => self.css_template_value("", css_content),
            "bl-lg" => self.css_template_value("", css_content),
            "tl-xl" => self.css_template_value("", css_content),
            "tr-xl" => self.css_template_value("", css_content),
            "br-xl" => self.css_template_value("", css_content),
            "bl-xl" => self.css_template_value("", css_content),
            "tl-2xl" => self.css_template_value("", css_content),
            "tr-2xl" => self.css_template_value("", css_content),
            "br-2xl" => self.css_template_value("", css_content),
            "bl-2xl" => self.css_template_value("", css_content),
            "tl-3xl" => self.css_template_value("", css_content),
            "tr-3xl" => self.css_template_value("", css_content),
            "br-3xl" => self.css_template_value("", css_content),
            "bl-3xl" => self.css_template_value("", css_content),
            "tl-full" => self.css_template_value("", css_content),
            "tr-full" => self.css_template_value("", css_content),
            "br-full" => self.css_template_value("", css_content),
            "bl-full" => self.css_template_value("", css_content),
            _ => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct DivideColorPlugin;

impl Plugin for DivideColorPlugin {
    fn namespace(&self) -> &str {
        "divide"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "color" || is_matching_color(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        if val.contains("--tw-opacity") {
            write!(css_content,
                "--tw-divide-opacity: 1;
  border-color: {};",
                val.replace("--tw-opacity", "--tw-divide-opacity")
            )
        } else {
            write!(css_content, "border-color: {val};")
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
pub struct DivideOpacityPlugin;

impl Plugin for DivideOpacityPlugin {
    fn namespace(&self) -> &str {
        "divide-opacity"
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        // NOTE: Not-compatible with TailwindCSS, support all values
        if let Ok(opacity_value) = modifier.parse::<f32>() {
            write!(css_content, "--tw-divide-opacity: {};", opacity_value / 100.)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct RingColorPlugin;

impl Plugin for RingColorPlugin {
    fn namespace(&self) -> &str {
        "ring"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "color" || is_matching_color(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        if val.contains("--tw-opacity") {
            write!(css_content,
                "--tw-ring-opacity: 1;
  --ring-color: {};",
                val.replace("--tw-opacity", "--tw-ring-opacity")
            )
        } else {
            write!(css_content, "--ring-color: {val};")
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
pub struct RingOpacityPlugin;

impl Plugin for RingOpacityPlugin {
    fn namespace(&self) -> &str {
        "ring-opacity"
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        // NOTE: Not-compatible with TailwindCSS, support all values
        if let Ok(opacity_value) = modifier.parse::<f32>() {
            write!(css_content, "--tw-ring-opacity: {};", opacity_value / 100.)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct RingOffsetColorPlugin;

impl Plugin for RingOffsetColorPlugin {
    fn namespace(&self) -> &str {
        "ring-offset"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "color" || is_matching_color(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        if val.contains("--tw-opacity") {
            write!(css_content,
                "--tw-ring-offset-opacity: 1;
  --ring-offset-color: {};",
                val.replace("--tw-opacity", "--tw-ring-offset-opacity")
            )
        } else {
            write!(css_content, "--ring-offset-color: {val};")
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
pub struct RingOffsetOpacityPlugin;

impl Plugin for RingOffsetOpacityPlugin {
    fn namespace(&self) -> &str {
        "ring-offset-opacity"
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        // NOTE: Not-compatible with TailwindCSS, support all values
        if let Ok(opacity_value) = modifier.parse::<f32>() {
            write!(
                css_content,
                "--tw-ring-offset-opacity: {};",
                opacity_value / 100.
            )
        } else {
            Ok(())
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
