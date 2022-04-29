use super::Plugin;
use crate::utils::{default_lengths, value_matchers::*};

use std::fmt::{Result, Write};

#[derive(Debug)]
pub struct ColumnsPlugin;

impl Plugin for ColumnsPlugin {
    fn namespace(&self) -> &str {
        "grid-cols"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        is_matching_all(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "grid-template-columns: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if modifier == "none" {
            return self.css_template_value("none", css_content);
        }

        // NOTE: Not-compatible with TailwindCSS, support all values
        if let Ok(num_cols) = modifier.parse::<usize>() {
            self.css_template_value(&format!("repeat({num_cols}, minmax(0, 1fr))"), css_content)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct RowsPlugin;

impl Plugin for RowsPlugin {
    fn namespace(&self) -> &str {
        "grid-rows"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        is_matching_all(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "grid-template-rows: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if modifier == "none" {
            return self.css_template_value("none", css_content);
        }

        // NOTE: Not-compatible with TailwindCSS, support all values
        if let Ok(num_cols) = modifier.parse::<usize>() {
            self.css_template_value(&format!("repeat({num_cols}, minmax(0, 1fr))"), css_content)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct StartEndSpanColumnPlugin;

impl Plugin for StartEndSpanColumnPlugin {
    fn namespace(&self) -> &str {
        "col"
    }

    fn is_matching_value(&self, _hint: &str, _val: &str) -> bool {
        true
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "grid-column: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if is_matching_auto(modifier) {
            return self.css_template_value("auto", css_content);
        }

        if let Some(val) = modifier.strip_prefix("span-") {
            if val == "full" {
                return self.css_template_value("1 / -1", css_content);
            }

            // NOTE: Not-compatible with TailwindCSS, support all values
            if let Ok(span_num) = val.parse::<usize>() {
                return self.css_template_value(&format!("span {span_num} / span {span_num}"), css_content);
            }
        } else if let Some(val) = modifier.strip_prefix("start-") {
            if is_matching_auto(val) {
                return write!(css_content, "grid-column-start: auto;");
            }

            // NOTE: Not-compatible with TailwindCSS, support all values
            if val.parse::<usize>().is_ok() {
                return write!(css_content, "grid-column-start: {val};");
            }
        } else if let Some(val) = modifier.strip_prefix("end-") {
            if is_matching_auto(val) {
                return write!(css_content, "grid-column-end: auto;");
            }

            // NOTE: Not-compatible with TailwindCSS, support all values
            if val.parse::<usize>().is_ok() {
                return write!(css_content, "grid-column-end: {val};");
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct StartEndSpanRowPlugin;

impl Plugin for StartEndSpanRowPlugin {
    fn namespace(&self) -> &str {
        "row"
    }

    fn is_matching_value(&self, _hint: &str, _val: &str) -> bool {
        true
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "grid-row: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if is_matching_auto(modifier) {
            return self.css_template_value("auto", css_content);
        }

        if let Some(val) = modifier.strip_prefix("span-") {
            if val == "full" {
                return self.css_template_value("1 / -1", css_content);
            }

            // NOTE: Not-compatible with TailwindCSS, support all values
            if let Ok(span_num) = val.parse::<usize>() {
                return self.css_template_value(&format!("span {span_num} / span {span_num}"), css_content);
            }
        } else if let Some(val) = modifier.strip_prefix("start-") {
            if is_matching_auto(val) {
                return write!(css_content, "grid-row-start: auto;");
            }

            // NOTE: Not-compatible with TailwindCSS, support all values
            if val.parse::<usize>().is_ok() {
                return write!(css_content, "grid-row-start: {val};");
            }
        } else if let Some(val) = modifier.strip_prefix("end-") {
            if is_matching_auto(val) {
                return write!(css_content, "grid-row-end: auto;");
            }

            // NOTE: Not-compatible with TailwindCSS, support all values
            if val.parse::<usize>().is_ok() {
                return write!(css_content, "grid-row-end: {val};");
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct AutoFlowPlugin;

impl Plugin for AutoFlowPlugin {
    fn namespace(&self) -> &str {
        "grid-flow"
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        match modifier {
            "row" => write!(css_content, "grid-auto-flow: row;"),
            "col" => write!(css_content, "grid-auto-flow: column;"),
            "row-dense" => write!(css_content, "grid-auto-flow: row dense;"),
            "col-dense" => write!(css_content, "grid-auto-flow: column dense;"),
            _ => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct AutoColumnsPlugin;

impl Plugin for AutoColumnsPlugin {
    fn namespace(&self) -> &str {
        "auto-cols"
    }

    fn is_matching_value(&self, _hint: &str, _val: &str) -> bool {
        true
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "grid-auto-columns: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        match modifier {
            "auto" => self.css_template_value("auto", css_content),
            "min" => self.css_template_value("min-content", css_content),
            "max" => self.css_template_value("max-content", css_content),
            "fr" => self.css_template_value("minmax(0, 1fr)", css_content),
            _ => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct AutoRowsPlugin;

impl Plugin for AutoRowsPlugin {
    fn namespace(&self) -> &str {
        "auto-rows"
    }

    fn is_matching_value(&self, _hint: &str, _val: &str) -> bool {
        true
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "grid-auto-rows: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        match modifier {
            "auto" => self.css_template_value("auto", css_content),
            "min" => self.css_template_value("min-content", css_content),
            "max" => self.css_template_value("max-content", css_content),
            "fr" => self.css_template_value("minmax(0, 1fr)", css_content),
            _ => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct GapPlugin;

impl Plugin for GapPlugin {
    fn namespace(&self) -> &str {
        "gap"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "gap: {val};")
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
pub struct GapXPlugin;

impl Plugin for GapXPlugin {
    fn namespace(&self) -> &str {
        "gap-x"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "column-gap: {val};")
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
pub struct GapYPlugin;

impl Plugin for GapYPlugin {
    fn namespace(&self) -> &str {
        "gap-y"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "row-gap: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if let Some(length) = default_lengths::get_basic(modifier) {
            self.css_template_value(&length, css_content)
        } else {
            Ok(())
        }
    }
}
