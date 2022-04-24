use super::Plugin;

#[derive(Debug)]
pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn namespace(&self) -> &str {
        "cursor"
    }

    fn is_matching_value(&self, _hint: &str, _val: &str) -> bool {
        true
    }

    fn css_template_value(&self, val: &str) -> String {
        format!("cursor: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        if [
            "auto",
            "default",
            "pointer",
            "wait",
            "text",
            "move",
            "help",
            "not-allowed",
        ]
        .contains(&modifier)
        {
            Some(self.css_template_value(modifier))
        } else {
            None
        }
    }
}

/*use super::SelectorList;

pub fn init(selectors: &mut SelectorList) {
    selectors.register("appearance-none", "appearance: none;".to_string());
    selectors.register("outline-none", "outline: 0;".to_string());
    selectors.register(
        "outline-white",
        "outline: 2px dotted white; outline-offset: 2px;".to_string(),
    );
    selectors.register(
        "outline-black",
        "outline: 2px dotted black; outline-offset: 2px;".to_string(),
    );
    selectors.register("pointer-events-none", "pointer-events: none;".to_string());
    selectors.register("pointer-events-auto", "pointer-events: auto;".to_string());
    selectors.register("resize-none", "resize: none;".to_string());
    selectors.register("resize", "resize: both;".to_string());
    selectors.register("resize-y", "resize: vertical;".to_string());
    selectors.register("resize-x", "resize: horizontal;".to_string());
    selectors.register("select-none", "user-select: none;".to_string());
    selectors.register("select-text", "user-select: text;".to_string());
    selectors.register("select-all", "user-select: all;".to_string());
    selectors.register("select-auto", "user-select: auto;".to_string());
    selectors.register("sr-only", "position: absolute; width: 1px; height: 1px; padding: 0; margin: -1px; overflow: hidden; clip: rect(0, 0, 0, 0); whiteSpace: nowrap; borderWidth: 0;".to_string());
    selectors.register("not-sr-only", "position: static; width: auto; height: auto; padding: 0; margin: 0; overflow: visible; clip: auto; whiteSpace: normal;".to_string());
}*/
