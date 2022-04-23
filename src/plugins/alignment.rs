use super::Plugin;

#[derive(Debug)]
pub struct AlignItemsPlugin;

impl Plugin for AlignItemsPlugin {
    fn namespace(&self) -> String {
        "items".to_string()
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        match modifier {
            "stretch" => Some("align-items: stretch;".to_string()),
            "start" => Some("align-items: flex-start;".to_string()),
            "center" => Some("align-items: center;".to_string()),
            "end" => Some("align-items: flex-end;".to_string()),
            "baseline" => Some("align-items: baseline;".to_string()),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct JustifyContentPlugin;

impl Plugin for JustifyContentPlugin {
    fn namespace(&self) -> String {
        "justify".to_string()
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        match modifier {
            "start" => Some("justify-content: flex-start;".to_string()),
            "center" => Some("justify-content: center;".to_string()),
            "end" => Some("justify-content: flex-end;".to_string()),
            "between" => Some("justify-content: space-between;".to_string()),
            "around" => Some("justify-content: space-around;".to_string()),
            "evenly" => Some("justify-content: space-evenly;".to_string()),
            _ => None,
        }
    }
}

/*use super::SelectorList;

pub fn init(selectors: &mut SelectorList) {
    selectors.register(
        "justify-items-stretch",
        "justify-items: stretch;".to_string(),
    );
    selectors.register("justify-items-start", "justify-items: start;".to_string());
    selectors.register("justify-items-center", "justify-items: center;".to_string());
    selectors.register("justify-items-end", "justify-items: end;".to_string());
    selectors.register("justify-items-auto", "justify-items: auto;".to_string());
    selectors.register("justify-self-stretch", "justify-self: stretch;".to_string());
    selectors.register("justify-self-start", "justify-self: start;".to_string());
    selectors.register("justify-self-center", "justify-self: center;".to_string());
    selectors.register("justify-self-end", "justify-self: end;".to_string());
    selectors.register("justify-self-auto", "justify-self: auto;".to_string());
    selectors.register("content-start", "align-content: flex-start;".to_string());
    selectors.register("content-center", "align-content: center;".to_string());
    selectors.register("content-end", "align-content: flex-end;".to_string());
    selectors.register(
        "content-between",
        "align-content: space-between;".to_string(),
    );
    selectors.register("content-around", "align-content: space-around;".to_string());
    selectors.register("content-evenly", "align-content: space-evenly;".to_string());
    selectors.register("self-auto", "align-self: auto;".to_string());
    selectors.register("self-start", "align-self: flex-start;".to_string());
    selectors.register("self-center", "align-self: center;".to_string());
    selectors.register("self-end", "align-self: flex-end;".to_string());
    selectors.register("self-stretch", "align-self: stretch;".to_string());
    selectors.register("place-content-start", "place-content: start;".to_string());
    selectors.register("place-content-center", "place-content: center;".to_string());
    selectors.register("place-content-end", "place-content: end;".to_string());
    selectors.register(
        "place-content-between",
        "place-content: space-between;".to_string(),
    );
    selectors.register(
        "place-content-around",
        "place-content: space-around;".to_string(),
    );
    selectors.register(
        "place-content-evenly",
        "place-content: space-evenly;".to_string(),
    );
    selectors.register("place-items-stretch", "place-items: stretch;".to_string());
    selectors.register("place-items-start", "place-items: start;".to_string());
    selectors.register("place-items-center", "place-items: center;".to_string());
    selectors.register("place-items-end", "place-items: end;".to_string());
    selectors.register("place-self-auto", "place-self: auto;".to_string());
    selectors.register("place-self-start", "place-self: start;".to_string());
    selectors.register("place-self-center", "place-self: center;".to_string());
    selectors.register("place-self-end", "place-self: end;".to_string());
    selectors.register("place-self-stretch", "place-self: stretch;".to_string());
}*/
