use super::Plugin;
use crate::{config::Config, selector::Modifier};

use std::fmt::Write;

#[derive(Debug)]
pub struct AlignContentPlugin;

impl Plugin for AlignContentPlugin {
    fn namespace(&self) -> &str {
        "content"
    }

    fn get_css_for_modifier(&self, _config: &Config, modifier: &Modifier, css_content: &mut String) -> bool {
        match modifier.content() {
            "start" => write!(css_content, "align-content: flex-start;").is_ok(),
            "center" => write!(css_content, "align-content: center;").is_ok(),
            "end" => write!(css_content, "align-content: flex-end;").is_ok(),
            "between" => write!(css_content, "align-content: space-between;").is_ok(),
            "around" => write!(css_content, "align-content: space-around;").is_ok(),
            "evenly" => write!(css_content, "align-content: space-evenly;").is_ok(),
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct AlignItemsPlugin;

impl Plugin for AlignItemsPlugin {
    fn namespace(&self) -> &str {
        "items"
    }

    fn get_css_for_modifier(&self, _config: &Config, modifier: &Modifier, css_content: &mut String) -> bool {
        match modifier.content() {
            "stretch" => write!(css_content, "align-items: stretch;").is_ok(),
            "start" => write!(css_content, "align-items: flex-start;").is_ok(),
            "center" => write!(css_content, "align-items: center;").is_ok(),
            "end" => write!(css_content, "align-items: flex-end;").is_ok(),
            "baseline" => write!(css_content, "align-items: baseline;").is_ok(),
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct AlignSelfPlugin;

impl Plugin for AlignSelfPlugin {
    fn namespace(&self) -> &str {
        "self"
    }

    fn get_css_for_modifier(&self, _config: &Config, modifier: &Modifier, css_content: &mut String) -> bool {
        match modifier.content() {
            "auto" => write!(css_content, "align-self: auto;").is_ok(),
            "start" => write!(css_content, "align-self: flex-start;").is_ok(),
            "center" => write!(css_content, "align-self: center;").is_ok(),
            "end" => write!(css_content, "align-self: flex-end;").is_ok(),
            "stretch" => write!(css_content, "align-self: stretch;").is_ok(),
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct JustifyContentPlugin;

impl Plugin for JustifyContentPlugin {
    fn namespace(&self) -> &str {
        "justify"
    }

    fn get_css_for_modifier(&self, _config: &Config, modifier: &Modifier, css_content: &mut String) -> bool {
        match modifier.content() {
            "start" => write!(css_content, "justify-content: flex-start;").is_ok(),
            "center" => write!(css_content, "justify-content: center;").is_ok(),
            "end" => write!(css_content, "justify-content: flex-end;").is_ok(),
            "between" => write!(css_content, "justify-content: space-between;").is_ok(),
            "around" => write!(css_content, "justify-content: space-around;").is_ok(),
            "evenly" => write!(css_content, "justify-content: space-evenly;").is_ok(),
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct JustifyItemsPlugin;

impl Plugin for JustifyItemsPlugin {
    fn namespace(&self) -> &str {
        "justify-items"
    }

    fn get_css_for_modifier(&self, _config: &Config, modifier: &Modifier, css_content: &mut String) -> bool {
        match modifier.content() {
            "stretch" => write!(css_content, "justify-items: stretch;").is_ok(),
            "start" => write!(css_content, "justify-items: start;").is_ok(),
            "center" => write!(css_content, "justify-items: center;").is_ok(),
            "end" => write!(css_content, "justify-items: end;").is_ok(),
            "auto" => write!(css_content, "justify-items: auto;").is_ok(),
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct JustifySelfPlugin;

impl Plugin for JustifySelfPlugin {
    fn namespace(&self) -> &str {
        "justify-self"
    }

    fn get_css_for_modifier(&self, _config: &Config, modifier: &Modifier, css_content: &mut String) -> bool {
        match modifier.content() {
            "stretch" => write!(css_content, "justify-self: stretch;").is_ok(),
            "start" => write!(css_content, "justify-self: start;").is_ok(),
            "center" => write!(css_content, "justify-self: center;").is_ok(),
            "end" => write!(css_content, "justify-self: end;").is_ok(),
            "auto" => write!(css_content, "justify-self: auto;").is_ok(),
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct PlaceContentPlugin;

impl Plugin for PlaceContentPlugin {
    fn namespace(&self) -> &str {
        "place-content"
    }

    fn get_css_for_modifier(&self, _config: &Config, modifier: &Modifier, css_content: &mut String) -> bool {
        match modifier.content() {
            "start" => write!(css_content, "place-content: start;").is_ok(),
            "center" => write!(css_content, "place-content: center;").is_ok(),
            "end" => write!(css_content, "place-content: end;").is_ok(),
            "between" => write!(css_content, "place-content: space-between;").is_ok(),
            "around" => write!(css_content, "place-content: space-around;").is_ok(),
            "evenly" => write!(css_content, "place-content: space-evenly;").is_ok(),
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct PlaceItemsPlugin;

impl Plugin for PlaceItemsPlugin {
    fn namespace(&self) -> &str {
        "place-items"
    }

    fn get_css_for_modifier(&self, _config: &Config, modifier: &Modifier, css_content: &mut String) -> bool {
        match modifier.content() {
            "stretch" => write!(css_content, "place-items: stretch;").is_ok(),
            "start" => write!(css_content, "place-items: start;").is_ok(),
            "center" => write!(css_content, "place-items: center;").is_ok(),
            "end" => write!(css_content, "place-items: end;").is_ok(),
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct PlaceSelfPlugin;

impl Plugin for PlaceSelfPlugin {
    fn namespace(&self) -> &str {
        "place-self"
    }

    fn get_css_for_modifier(&self, _config: &Config, modifier: &Modifier, css_content: &mut String) -> bool {
        match modifier.content() {
            "self-auto" => write!(css_content, "place-self: auto;").is_ok(),
            "self-start" => write!(css_content, "place-self: start;").is_ok(),
            "self-center" => write!(css_content, "place-self: center;").is_ok(),
            "self-end" => write!(css_content, "place-self: end;").is_ok(),
            "self-stretch" => write!(css_content, "place-self: stretch;").is_ok(),
            _ => false,
        }
    }
}
