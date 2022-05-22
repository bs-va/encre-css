use super::Plugin;
use crate::{config::Config, selector::Modifier};
use crate::utils::{default_lengths, value_matchers::*};

use std::fmt::Write;

#[derive(Debug)]
pub struct ColumnsPlugin;

impl Plugin for ColumnsPlugin {
    fn namespace(&self) -> &str {
        "grid-cols"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        is_matching_all(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "grid-template-columns: {val};").is_ok()
    }

    fn get_css_for_modifier(&self, _config: &Config, modifier: &Modifier, css_content: &mut String, _custom_css: &mut String) -> bool {
        if modifier.is("none") {
            return self.css_template_value("none", css_content);
        }

        // NOTE: Not-compatible with TailwindCSS, support all values
        if let Ok(num_cols) = modifier.to_usize() {
            self.css_template_value(&format!("repeat({num_cols}, minmax(0, 1fr))"), css_content)
        } else {
            false
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

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "grid-template-rows: {val};").is_ok()
    }

    fn get_css_for_modifier(&self, _config: &Config, modifier: &Modifier, css_content: &mut String, _custom_css: &mut String) -> bool {
        if modifier.is("none") {
            return self.css_template_value("none", css_content);
        }

        // NOTE: Not-compatible with TailwindCSS, support all values
        if let Ok(num_cols) = modifier.to_usize() {
            self.css_template_value(&format!("repeat({num_cols}, minmax(0, 1fr))"), css_content)
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct StartEndSpanColumnPlugin;

impl Plugin for StartEndSpanColumnPlugin {
    fn namespace(&self) -> &str {
        "col"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        is_matching_all(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "grid-column: {val};").is_ok()
    }

    fn get_css_for_modifier(&self, _config: &Config, modifier: &Modifier, css_content: &mut String, _custom_css: &mut String) -> bool {
        if modifier.is("auto") {
            return self.css_template_value("auto", css_content);
        }

        if let Some(val) = modifier.strip_prefix("span-") {
            if val == "full" {
                return self.css_template_value("1 / -1", css_content);
            }

            // NOTE: Not-compatible with TailwindCSS, support all values
            if let Ok(span_num) = val.parse::<usize>() {
                return self.css_template_value(
                    &format!("span {span_num} / span {span_num}"),
                    css_content,
                );
            }
        } else if let Some(val) = modifier.strip_prefix("start-") {
            if val == "auto" {
                return write!(css_content, "grid-column-start: auto;").is_ok();
            }

            // NOTE: Not-compatible with TailwindCSS, support all values
            if val.parse::<usize>().is_ok() {
                return write!(css_content, "grid-column-start: {val};").is_ok();
            }
        } else if let Some(val) = modifier.strip_prefix("end-") {
            if val == "auto" {
                return write!(css_content, "grid-column-end: auto;").is_ok();
            }

            // NOTE: Not-compatible with TailwindCSS, support all values
            if val.parse::<usize>().is_ok() {
                return write!(css_content, "grid-column-end: {val};").is_ok();
            }
        }

        false
    }
}

#[derive(Debug)]
pub struct StartEndSpanRowPlugin;

impl Plugin for StartEndSpanRowPlugin {
    fn namespace(&self) -> &str {
        "row"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        is_matching_all(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "grid-row: {val};").is_ok()
    }

    fn get_css_for_modifier(&self, _config: &Config, modifier: &Modifier, css_content: &mut String, _custom_css: &mut String) -> bool {
        if modifier.is("auto") {
            return self.css_template_value("auto", css_content);
        }

        if let Some(val) = modifier.strip_prefix("span-") {
            if val == "full" {
                return self.css_template_value("1 / -1", css_content);
            }

            // NOTE: Not-compatible with TailwindCSS, support all values
            if let Ok(span_num) = val.parse::<usize>() {
                return self.css_template_value(
                    &format!("span {span_num} / span {span_num}"),
                    css_content,
                );
            }
        } else if let Some(val) = modifier.strip_prefix("start-") {
            if val == "auto" {
                return write!(css_content, "grid-row-start: auto;").is_ok();
            }

            // NOTE: Not-compatible with TailwindCSS, support all values
            if val.parse::<usize>().is_ok() {
                return write!(css_content, "grid-row-start: {val};").is_ok();
            }
        } else if let Some(val) = modifier.strip_prefix("end-") {
            if val == "auto" {
                return write!(css_content, "grid-row-end: auto;").is_ok();
            }

            // NOTE: Not-compatible with TailwindCSS, support all values
            if val.parse::<usize>().is_ok() {
                return write!(css_content, "grid-row-end: {val};").is_ok();
            }
        }

        false
    }
}

#[derive(Debug)]
pub struct AutoFlowPlugin;

impl Plugin for AutoFlowPlugin {
    fn namespace(&self) -> &str {
        "grid-flow"
    }

    fn get_css_for_modifier(&self, _config: &Config, modifier: &Modifier, css_content: &mut String, _custom_css: &mut String) -> bool {
        match modifier.content() {
            "row" => write!(css_content, "grid-auto-flow: row;").is_ok(),
            "col" => write!(css_content, "grid-auto-flow: column;").is_ok(),
            "row-dense" => write!(css_content, "grid-auto-flow: row dense;").is_ok(),
            "col-dense" => write!(css_content, "grid-auto-flow: column dense;").is_ok(),
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct AutoColumnsPlugin;

impl Plugin for AutoColumnsPlugin {
    fn namespace(&self) -> &str {
        "auto-cols"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        is_matching_all(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "grid-auto-columns: {val};").is_ok()
    }

    fn get_css_for_modifier(&self, _config: &Config, modifier: &Modifier, css_content: &mut String, _custom_css: &mut String) -> bool {
        match modifier.content() {
            "auto" => self.css_template_value("auto", css_content),
            "min" => self.css_template_value("min-content", css_content),
            "max" => self.css_template_value("max-content", css_content),
            "fr" => self.css_template_value("minmax(0, 1fr)", css_content),
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct AutoRowsPlugin;

impl Plugin for AutoRowsPlugin {
    fn namespace(&self) -> &str {
        "auto-rows"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        is_matching_all(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "grid-auto-rows: {val};").is_ok()
    }

    fn get_css_for_modifier(&self, _config: &Config, modifier: &Modifier, css_content: &mut String, _custom_css: &mut String) -> bool {
        match modifier.content() {
            "auto" => self.css_template_value("auto", css_content),
            "min" => self.css_template_value("min-content", css_content),
            "max" => self.css_template_value("max-content", css_content),
            "fr" => self.css_template_value("minmax(0, 1fr)", css_content),
            _ => false,
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

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "gap: {val};").is_ok()
    }

    fn get_css_for_modifier(&self, _config: &Config, modifier: &Modifier, css_content: &mut String, _custom_css: &mut String) -> bool {
        if let Some(length) = default_lengths::get_basic(modifier.content(), modifier.is_negative()) {
            self.css_template_value(&length, css_content)
        } else {
            false
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

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "column-gap: {val};").is_ok()
    }

    fn get_css_for_modifier(&self, _config: &Config, modifier: &Modifier, css_content: &mut String, _custom_css: &mut String) -> bool {
        if let Some(length) = default_lengths::get_basic(modifier.content(), modifier.is_negative()) {
            self.css_template_value(&length, css_content)
        } else {
            false
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

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "row-gap: {val};").is_ok()
    }

    fn get_css_for_modifier(&self, _config: &Config, modifier: &Modifier, css_content: &mut String, _custom_css: &mut String) -> bool {
        if let Some(length) = default_lengths::get_basic(modifier.content(), modifier.is_negative()) {
            self.css_template_value(&length, css_content)
        } else {
            false
        }
    }
}
