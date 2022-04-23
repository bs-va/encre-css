use super::Plugin;
use crate::utils::value_matchers::*;

#[derive(Debug)]
pub struct OrderPlugin;

impl Plugin for OrderPlugin {
    fn namespace(&self) -> String {
        "order".to_string()
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        is_matching_number(val)
    }

    fn css_template_value(&self, val: &str) -> String {
        format!("order: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        match modifier {
            "first" => return Some(self.css_template_value("-9999")),
            "last" => return Some(self.css_template_value("9999")),
            "none" => return Some(self.css_template_value("0")),
            _ => (),
        }

        // NOTE: Not-compatible with TailwindCSS, support all values
        if modifier.parse::<f32>().is_ok() {
            Some(self.css_template_value(modifier))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct DirectionPlugin;

impl Plugin for DirectionPlugin {
    fn namespace(&self) -> String {
        "flex".to_string()
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        match modifier {
            "row" => Some("flex-direction: row;".to_string()),
            "row-reverse" => Some("flex-direction: row-reverse;".to_string()),
            "col" => Some("flex-direction: column;".to_string()),
            "col-reverse" => Some("flex-direction: column-reverse;".to_string()),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct WrapPlugin;

impl Plugin for WrapPlugin {
    fn namespace(&self) -> String {
        "flex".to_string()
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        match modifier {
            "nowrap" => Some("flex-wrap: nowrap;".to_string()),
            "wrap" => Some("flex-wrap: wrap;".to_string()),
            "wrap-reverse" => Some("flex-wrap: wrap-reverse;".to_string()),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct GrowShrinkBasisPlugin;

impl Plugin for GrowShrinkBasisPlugin {
    fn namespace(&self) -> String {
        "flex".to_string()
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        if hint == "list" {
            return true;
        }

        let mut split = val.split('_');
        let mut is_matching = (false, false, false);

        // flex-grow
        // https://developer.mozilla.org/en-US/docs/Web/CSS/flex-grow#values
        if let Some(val) = split.next() {
            if is_matching_number(val) {
                is_matching.0 = true;
            }
        }

        // flex-shrink
        // https://developer.mozilla.org/en-US/docs/Web/CSS/flex-shrink#values
        if let Some(val) = split.next() {
            if is_matching_number(val) {
                is_matching.1 = true;
            }
        }

        // flex-basis
        // https://developer.mozilla.org/en-US/docs/Web/CSS/flex-basis#values
        if let Some(val) = split.next() {
            if is_matching_length(val) || is_matching_percentage(val) || is_matching_auto(val) {
                is_matching.2 = true;
            }
        }

        is_matching.0 && is_matching.1 && is_matching.2
    }

    fn css_template_value(&self, val: &str) -> String {
        format!("flex: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        match modifier {
            "flex-1" => Some(self.css_template_value("1 1 0%")),
            "flex-auto" => Some(self.css_template_value("1 1 auto")),
            "flex-initial" => Some(self.css_template_value("0 1 auto")),
            "flex-none" => Some(self.css_template_value("none")),
            "flex-grow" => Some("flex-grow: 1;".to_string()),
            "flex-grow-0" => Some("flex-grow: 0;".to_string()),
            "flex-shrink" => Some("flex-shrink: 1;".to_string()),
            "flex-shrink-0" => Some("flex-shrink: 0;".to_string()),
            _ => None,
        }
    }
}
