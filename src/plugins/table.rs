use super::Plugin;

#[derive(Debug)]
pub struct BorderCollapsePlugin;

impl Plugin for BorderCollapsePlugin {
    fn namespace(&self) -> &str {
        "border"
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        match modifier {
            "collapse" => Some("border-collapse: collapse;".to_string()),
            "separate" => Some("border-collapse: separate;".to_string()),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct TableLayoutPlugin;

impl Plugin for TableLayoutPlugin {
    fn namespace(&self) -> &str {
        "table"
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        match modifier {
            "auto" => Some("table-layout: auto;".to_string()),
            "fixed" => Some("table-layout: fixed;".to_string()),
            _ => None,
        }
    }
}
