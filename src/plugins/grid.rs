use super::Plugin;
use crate::utils::{default_lengths, value_matchers::*};

#[derive(Debug)]
pub struct ColumnsPlugin;

impl Plugin for ColumnsPlugin {
    fn namespace(&self) -> String {
        "grid-cols".to_string()
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        is_matching_all(val)
    }

    fn css_template_value(&self, val: &str) -> String {
        format!("grid-template-columns: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        if modifier == "none" {
            return Some(self.css_template_value("none"));
        }

        // NOTE: Not-compatible with TailwindCSS, support all values
        if let Ok(num_cols) = modifier.parse::<usize>() {
            Some(self.css_template_value(&format!("repeat({num_cols}, minmax(0, 1fr))")))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct RowsPlugin;

impl Plugin for RowsPlugin {
    fn namespace(&self) -> String {
        "grid-rows".to_string()
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        is_matching_all(val)
    }

    fn css_template_value(&self, val: &str) -> String {
        format!("grid-template-rows: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        if modifier == "none" {
            return Some(self.css_template_value("none"));
        }

        // NOTE: Not-compatible with TailwindCSS, support all values
        if let Ok(num_cols) = modifier.parse::<usize>() {
            Some(self.css_template_value(&format!("repeat({num_cols}, minmax(0, 1fr))")))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct GapPlugin;

impl Plugin for GapPlugin {
    fn namespace(&self) -> String {
        "gap".to_string()
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        is_matching_length(val)
    }

    fn css_template_value(&self, val: &str) -> String {
        format!("gap: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        default_lengths::get_basic(modifier).map(|c| self.css_template_value(&c))
    }
}

/*use super::SelectorList;

pub fn init(selectors: &mut SelectorList) {
    selectors.register("col-auto", "".to_string());
    selectors.register("col-span-1", "".to_string());
    selectors.register("col-span-2", "".to_string());
    selectors.register("col-span-3", "".to_string());
    selectors.register("col-span-4", "".to_string());
    selectors.register("col-span-5", "".to_string());
    selectors.register("col-span-6", "".to_string());
    selectors.register("col-span-7", "".to_string());
    selectors.register("col-span-8", "".to_string());
    selectors.register("col-span-9", "".to_string());
    selectors.register("col-span-10", "".to_string());
    selectors.register("col-span-11", "".to_string());
    selectors.register("col-span-12", "".to_string());
    selectors.register("col-start-1", "".to_string());
    selectors.register("col-start-2", "".to_string());
    selectors.register("col-start-3", "".to_string());
    selectors.register("col-start-4", "".to_string());
    selectors.register("col-start-5", "".to_string());
    selectors.register("col-start-6", "".to_string());
    selectors.register("col-start-7", "".to_string());
    selectors.register("col-start-8", "".to_string());
    selectors.register("col-start-9", "".to_string());
    selectors.register("col-start-10", "".to_string());
    selectors.register("col-start-11", "".to_string());
    selectors.register("col-start-12", "".to_string());
    selectors.register("col-start-13", "".to_string());
    selectors.register("col-start-auto", "".to_string());
    selectors.register("col-end-1", "".to_string());
    selectors.register("col-end-2", "".to_string());
    selectors.register("col-end-3", "".to_string());
    selectors.register("col-end-4", "".to_string());
    selectors.register("col-end-5", "".to_string());
    selectors.register("col-end-6", "".to_string());
    selectors.register("col-end-7", "".to_string());
    selectors.register("col-end-8", "".to_string());
    selectors.register("col-end-9", "".to_string());
    selectors.register("col-end-10", "".to_string());
    selectors.register("col-end-11", "".to_string());
    selectors.register("col-end-12", "".to_string());
    selectors.register("col-end-13", "".to_string());
    selectors.register("col-end-auto", "".to_string());
    selectors.register("grid-rows-1", "".to_string());
    selectors.register("grid-rows-2", "".to_string());
    selectors.register("grid-rows-3", "".to_string());
    selectors.register("grid-rows-4", "".to_string());
    selectors.register("grid-rows-5", "".to_string());
    selectors.register("grid-rows-6", "".to_string());
    selectors.register("grid-rows-none", "".to_string());
    selectors.register("row-auto", "".to_string());
    selectors.register("row-span-1", "".to_string());
    selectors.register("row-span-2", "".to_string());
    selectors.register("row-span-3", "".to_string());
    selectors.register("row-span-4", "".to_string());
    selectors.register("row-span-5", "".to_string());
    selectors.register("row-span-6", "".to_string());
    selectors.register("row-start-1", "".to_string());
    selectors.register("row-start-2", "".to_string());
    selectors.register("row-start-3", "".to_string());
    selectors.register("row-start-4", "".to_string());
    selectors.register("row-start-5", "".to_string());
    selectors.register("row-start-6", "".to_string());
    selectors.register("row-start-7", "".to_string());
    selectors.register("row-start-auto", "".to_string());
    selectors.register("row-end-1", "".to_string());
    selectors.register("row-end-2", "".to_string());
    selectors.register("row-end-3", "".to_string());
    selectors.register("row-end-4", "".to_string());
    selectors.register("row-end-5", "".to_string());
    selectors.register("row-end-6", "".to_string());
    selectors.register("row-end-7", "".to_string());
    selectors.register("row-end-auto", "".to_string());
    selectors.register("gap-x-0", "".to_string());
    selectors.register("gap-x-0.5", "".to_string());
    selectors.register("gap-x-1", "".to_string());
    selectors.register("gap-x-1.5", "".to_string());
    selectors.register("gap-x-2", "".to_string());
    selectors.register("gap-x-2.5", "".to_string());
    selectors.register("gap-x-3", "".to_string());
    selectors.register("gap-x-3.5", "".to_string());
    selectors.register("gap-x-4", "".to_string());
    selectors.register("gap-x-5", "".to_string());
    selectors.register("gap-x-6", "".to_string());
    selectors.register("gap-x-8", "".to_string());
    selectors.register("gap-x-10", "".to_string());
    selectors.register("gap-x-11", "".to_string());
    selectors.register("gap-x-12", "".to_string());
    selectors.register("gap-x-14", "".to_string());
    selectors.register("gap-x-16", "".to_string());
    selectors.register("gap-x-20", "".to_string());
    selectors.register("gap-x-24", "".to_string());
    selectors.register("gap-x-28", "".to_string());
    selectors.register("gap-x-32", "".to_string());
    selectors.register("gap-x-36", "".to_string());
    selectors.register("gap-x-40", "".to_string());
    selectors.register("gap-x-44", "".to_string());
    selectors.register("gap-x-48", "".to_string());
    selectors.register("gap-x-52", "".to_string());
    selectors.register("gap-x-56", "".to_string());
    selectors.register("gap-x-64", "".to_string());
    selectors.register("gap-x-72", "".to_string());
    selectors.register("gap-x-80", "".to_string());
    selectors.register("gap-x-96", "".to_string());
    selectors.register("gap-x-px", "".to_string());
    selectors.register("gap-y-0", "".to_string());
    selectors.register("gap-y-0.5", "".to_string());
    selectors.register("gap-y-1", "".to_string());
    selectors.register("gap-y-1.5", "".to_string());
    selectors.register("gap-y-2", "".to_string());
    selectors.register("gap-y-2.5", "".to_string());
    selectors.register("gap-y-3", "".to_string());
    selectors.register("gap-y-3.5", "".to_string());
    selectors.register("gap-y-4", "".to_string());
    selectors.register("gap-y-5", "".to_string());
    selectors.register("gap-y-6", "".to_string());
    selectors.register("gap-y-8", "".to_string());
    selectors.register("gap-y-10", "".to_string());
    selectors.register("gap-y-11", "".to_string());
    selectors.register("gap-y-12", "".to_string());
    selectors.register("gap-y-14", "".to_string());
    selectors.register("gap-y-16", "".to_string());
    selectors.register("gap-y-20", "".to_string());
    selectors.register("gap-y-24", "".to_string());
    selectors.register("gap-y-28", "".to_string());
    selectors.register("gap-y-32", "".to_string());
    selectors.register("gap-y-36", "".to_string());
    selectors.register("gap-y-40", "".to_string());
    selectors.register("gap-y-44", "".to_string());
    selectors.register("gap-y-48", "".to_string());
    selectors.register("gap-y-52", "".to_string());
    selectors.register("gap-y-56", "".to_string());
    selectors.register("gap-y-64", "".to_string());
    selectors.register("gap-y-72", "".to_string());
    selectors.register("gap-y-80", "".to_string());
    selectors.register("gap-y-96", "".to_string());
    selectors.register("gap-y-px", "".to_string());
    selectors.register("grid-flow-row", "".to_string());
    selectors.register("grid-flow-col", "".to_string());
    selectors.register("grid-flow-row-dense", "".to_string());
    selectors.register("grid-flow-col-dense", "".to_string());
    selectors.register("auto-cols-auto", "".to_string());
    selectors.register("auto-cols-min", "".to_string());
    selectors.register("auto-cols-max", "".to_string());
    selectors.register("auto-cols-fr", "".to_string());
    selectors.register("auto-rows-auto", "".to_string());
    selectors.register("auto-rows-min", "".to_string());
    selectors.register("auto-rows-max", "".to_string());
    selectors.register("auto-rows-fr", "".to_string());
}*/
