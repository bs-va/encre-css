use super::Plugin;
use crate::utils::{default_colors, value_matchers::*};

use std::fmt::{Result, Write};

const CSS_RING_OFFSET_SHADOW: &str = "--tw-ring-offset-shadow: var(--tw-ring-inset) 0 0 0 var(--tw-ring-offset-width) var(--tw-ring-offset-color);";

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
            write!(
                css_content,
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
            _ => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct RadiusTopPlugin;

impl Plugin for RadiusTopPlugin {
    fn namespace(&self) -> &str {
        "rounded-t"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        val.split('_')
            .all(|v| is_matching_length(v) || is_matching_percentage(v))
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(
            css_content,
            "border-top-left-radius: {val};\n  border-top-right-radius: {val};"
        )
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
            _ => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct RadiusBottomPlugin;

impl Plugin for RadiusBottomPlugin {
    fn namespace(&self) -> &str {
        "rounded-b"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        val.split('_')
            .all(|v| is_matching_length(v) || is_matching_percentage(v))
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(
            css_content,
            "border-bottom-left-radius: {val};\n  border-bottom-right-radius: {val};"
        )
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
            _ => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct RadiusLeftPlugin;

impl Plugin for RadiusLeftPlugin {
    fn namespace(&self) -> &str {
        "rounded-l"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        val.split('_')
            .all(|v| is_matching_length(v) || is_matching_percentage(v))
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(
            css_content,
            "border-top-left-radius: {val};\n  border-bottom-left-radius: {val};"
        )
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
            _ => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct RadiusRightPlugin;

impl Plugin for RadiusRightPlugin {
    fn namespace(&self) -> &str {
        "rounded-r"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        val.split('_')
            .all(|v| is_matching_length(v) || is_matching_percentage(v))
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(
            css_content,
            "border-top-right-radius: {val};\n  border-bottom-right-radius: {val};"
        )
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
            _ => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct RadiusTopLeftPlugin;

impl Plugin for RadiusTopLeftPlugin {
    fn namespace(&self) -> &str {
        "rounded-tl"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        val.split('_')
            .all(|v| is_matching_length(v) || is_matching_percentage(v))
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "border-top-left-radius: {val};")
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
            _ => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct RadiusBottomLeftPlugin;

impl Plugin for RadiusBottomLeftPlugin {
    fn namespace(&self) -> &str {
        "rounded-bl"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        val.split('_')
            .all(|v| is_matching_length(v) || is_matching_percentage(v))
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "border-bottom-left-radius: {val};")
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
            _ => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct RadiusBottomRightPlugin;

impl Plugin for RadiusBottomRightPlugin {
    fn namespace(&self) -> &str {
        "rounded-br"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        val.split('_')
            .all(|v| is_matching_length(v) || is_matching_percentage(v))
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "border-bottom-right-radius: {val};")
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
            _ => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct RadiusTopRightPlugin;

impl Plugin for RadiusTopRightPlugin {
    fn namespace(&self) -> &str {
        "rounded-tr"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        val.split('_')
            .all(|v| is_matching_length(v) || is_matching_percentage(v))
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "border-top-right-radius: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        match modifier {
            "" => self.css_template_value("", css_content),
            "none" => self.css_template_value("", css_content),
            "sm" => self.css_template_value("", css_content),
            "md" => self.css_template_value("", css_content),
            "lg" => self.css_template_value("", css_content),
            "xl" => self.css_template_value("", css_content),
            "2xl" => self.css_template_value("", css_content),
            "3xl" => self.css_template_value("", css_content),
            "full" => self.css_template_value("", css_content),
            _ => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct StylePlugin;

impl Plugin for StylePlugin {
    fn namespace(&self) -> &str {
        "border"
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if ["solid", "dashed", "dotted", "double", "none"].contains(&modifier) {
            write!(css_content, "border-style: {};", modifier)
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
        write!(
            css_content,
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
        write!(
            css_content,
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
            write!(
                css_content,
                "--tw-border-opacity: {};",
                opacity_value / 100.
            )
        } else {
            Ok(())
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
            write!(
                css_content,
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
pub struct DivideXPlugin;

impl Plugin for DivideXPlugin {
    fn namespace(&self) -> &str {
        "divide-x"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        if !css_content.contains("--tw-divide-x-reverse") {
            write!(css_content, "--tw-divide-x-reverse: 0;\n  ")?;
        }

        write!(
            css_content,
            "border-left-width: calc({val} * calc(1 - var(--tw-divide-x-reverse)));
  border-right-width: calc({val} * var(--tw-divide-x-reverse));"
        )
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        // TODO: class with `> :not([hidden]) ~ :not([hidden])`
        if modifier.is_empty() {
            write!(css_content, "--tw-divide-x-reverse: 0;\n  ")?;
            return self.css_template_value("1px", css_content);
        } else if modifier == "reverse" {
            return write!(css_content, "--tw-divide-x-reverse: 1;");
        }

        // NOTE: Not-compatible with TailwindCSS, support all values
        if modifier.parse::<usize>().is_ok() {
            write!(css_content, "--tw-divide-x-reverse: 0;\n  ")?;
            self.css_template_value(&format!("{}px", modifier), css_content)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct DivideYPlugin;

impl Plugin for DivideYPlugin {
    fn namespace(&self) -> &str {
        "divide-y"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        if !css_content.contains("--tw-divide-y-reverse") {
            write!(css_content, "--tw-divide-y-reverse: 0;\n  ")?;
        }

        write!(
            css_content,
            "border-top-width: calc({val} * calc(1 - var(--tw-divide-y-reverse)));
  border-bottom-width: calc({val} * var(--tw-divide-y-reverse));"
        )
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        // TODO: class with `> :not([hidden]) ~ :not([hidden])`
        if modifier.is_empty() {
            write!(css_content, "--tw-divide-y-reverse: 0;\n  ")?;
            return self.css_template_value("1px", css_content);
        } else if modifier == "reverse" {
            return write!(css_content, "--tw-divide-y-reverse: 1;");
        }

        // NOTE: Not-compatible with TailwindCSS, support all values
        if modifier.parse::<usize>().is_ok() {
            write!(css_content, "--tw-divide-y-reverse: 0;\n  ")?;
            self.css_template_value(&format!("{}px", modifier), css_content)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct DivideStylePlugin;

impl Plugin for DivideStylePlugin {
    fn namespace(&self) -> &str {
        "divide"
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        match modifier {
            "solid" => write!(css_content, "border-style: solid;"),
            "dashed" => write!(css_content, "border-style: dashed;"),
            "dotted" => write!(css_content, "border-style: dotted;"),
            "double" => write!(css_content, "border-style: double;"),
            "none" => write!(css_content, "border-style: none;"),
            _ => Ok(()),
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
            write!(
                css_content,
                "--tw-divide-opacity: {};",
                opacity_value / 100.
            )
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
            write!(
                css_content,
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
pub struct RingWidthPlugin;

impl Plugin for RingWidthPlugin {
    fn namespace(&self) -> &str {
        "ring"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "{}
  --tw-ring-shadow: var(--tw-ring-inset) 0 0 0 calc({val} + var(--tw-ring-offset-width)) var(--tw-ring-color);
  box-shadow: var(--tw-ring-offset-shadow), var(--tw-ring-shadow), var(--tw-shadow, 0 0 #0000);", CSS_RING_OFFSET_SHADOW)
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if modifier.is_empty() {
            return self.css_template_value("3px", css_content);
        } else if modifier == "inset" {
            return write!(css_content, "--tw-ring-inset: inset;");
        }

        // NOTE: Not-compatible with TailwindCSS, support all values
        if modifier.parse::<usize>().is_ok() {
            self.css_template_value(&format!("{}px", modifier), css_content)
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
            write!(
                css_content,
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

#[derive(Debug)]
pub struct RingOffsetWidthPlugin;

impl Plugin for RingOffsetWidthPlugin {
    fn namespace(&self) -> &str {
        "ring-offset"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "--tw-ring-offset-width: {val};",)
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        // NOTE: Not-compatible with TailwindCSS, support all values
        if modifier.parse::<usize>().is_ok() {
            self.css_template_value(&format!("{}px", modifier), css_content)
        } else {
            Ok(())
        }
    }
}
