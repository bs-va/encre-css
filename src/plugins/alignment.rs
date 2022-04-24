use super::Plugin;

use std::fmt::{Result, Write};

#[derive(Debug)]
pub struct AlignContentPlugin;

impl Plugin for AlignContentPlugin {
    fn namespace(&self) -> &str {
        "items"
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        match modifier {
            "start" => write!(css_content, "align-content: flex-start;"),
            "center" => write!(css_content, "align-content: center;"),
            "end" => write!(css_content, "align-content: flex-end;"),
            "between" => write!(css_content, "align-content: space-between;"),
            "around" => write!(css_content, "align-content: space-around;"),
            "evenly" => write!(css_content, "align-content: space-evenly;"),
            _ => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct AlignItemsPlugin;

impl Plugin for AlignItemsPlugin {
    fn namespace(&self) -> &str {
        "items"
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        match modifier {
            "stretch" => write!(css_content, "align-items: stretch;"),
            "start" => write!(css_content, "align-items: flex-start;"),
            "center" => write!(css_content, "align-items: center;"),
            "end" => write!(css_content, "align-items: flex-end;"),
            "baseline" => write!(css_content, "align-items: baseline;"),
            _ => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct AlignSelfPlugin;

impl Plugin for AlignSelfPlugin {
    fn namespace(&self) -> &str {
        "self"
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        match modifier {
            "auto" => write!(css_content, "align-self: auto;"),
            "start" => write!(css_content, "align-self: flex-start;"),
            "center" => write!(css_content, "align-self: center;"),
            "end" => write!(css_content, "align-self: flex-end;"),
            "stretch" => write!(css_content, "align-self: stretch;"),
            _ => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct JustifyContentPlugin;

impl Plugin for JustifyContentPlugin {
    fn namespace(&self) -> &str {
        "justify"
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        match modifier {
            "start" => write!(css_content, "justify-content: flex-start;"),
            "center" => write!(css_content, "justify-content: center;"),
            "end" => write!(css_content, "justify-content: flex-end;"),
            "between" => write!(css_content, "justify-content: space-between;"),
            "around" => write!(css_content, "justify-content: space-around;"),
            "evenly" => write!(css_content, "justify-content: space-evenly;"),
            _ => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct JustifyItemsPlugin;

impl Plugin for JustifyItemsPlugin {
    fn namespace(&self) -> &str {
        "justify-items"
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        match modifier {
            "stretch" => write!(css_content, "justify-items: stretch;"),
            "start" => write!(css_content, "justify-items: start;"),
            "center" => write!(css_content, "justify-items: center;"),
            "end" => write!(css_content, "justify-items: end;"),
            "auto" => write!(css_content, "justify-items: auto;"),
            _ => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct JustifySelfPlugin;

impl Plugin for JustifySelfPlugin {
    fn namespace(&self) -> &str {
        "justify-self"
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        match modifier {
            "stretch" => write!(css_content, "justify-self: stretch;"),
            "start" => write!(css_content, "justify-self: start;"),
            "center" => write!(css_content, "justify-self: center;"),
            "end" => write!(css_content, "justify-self: end;"),
            "auto" => write!(css_content, "justify-self: auto;"),
            _ => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct PlaceContentPlugin;

impl Plugin for PlaceContentPlugin {
    fn namespace(&self) -> &str {
        "place-content"
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        match modifier {
            "start" => write!(css_content, "place-content: start;"),
            "center" => write!(css_content, "place-content: center;"),
            "end" => write!(css_content, "place-content: end;"),
            "between" => write!(css_content, "place-content: space-between;"),
            "around" => write!(css_content, "place-content: space-around;"),
            "evenly" => write!(css_content, "place-content: space-evenly;"),
            _ => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct PlaceItemsPlugin;

impl Plugin for PlaceItemsPlugin {
    fn namespace(&self) -> &str {
        "place-items"
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        match modifier {
            "stretch" => write!(css_content, "place-items: stretch;"),
            "start" => write!(css_content, "place-items: start;"),
            "center" => write!(css_content, "place-items: center;"),
            "end" => write!(css_content, "place-items: end;"),
            _ => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct PlaceSelfPlugin;

impl Plugin for PlaceSelfPlugin {
    fn namespace(&self) -> &str {
        "place-self"
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        match modifier {
            "self-auto" => write!(css_content, "place-self: auto;"),
            "self-start" => write!(css_content, "place-self: start;"),
            "self-center" => write!(css_content, "place-self: center;"),
            "self-end" => write!(css_content, "place-self: end;"),
            "self-stretch" => write!(css_content, "place-self: stretch;"),
            _ => Ok(()),
        }
    }
}
