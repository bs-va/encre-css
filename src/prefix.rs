use lazy_static::lazy_static;
use std::collections::BTreeMap;

pub const PREFIX_SEPARATOR: &str = ":";

lazy_static! {
    pub static ref PREFIXES: BTreeMap<String, String> = {
        // NOTE: If there is a prefix starting with the same characters than another complete
        // prefix, the first found will be the first in alphabetic order
        //
        // TODO: Support more prefixes https://tailwindcss.com/docs/hover-focus-and-other-states#highlighted-text
        let mut prefixes = BTreeMap::new();
        prefixes.insert("hover".to_string(), ".{class}:hover {
  {css}
}".to_string());
        prefixes.insert("focus".to_string(), ".{class}:focus {
  {css}
}".to_string());
        prefixes.insert("active".to_string(), ".{class}:active {
  {css}
}".to_string());
        prefixes.insert("focus-within".to_string(), ".{class}:focus-within {
  {css}
}".to_string());
        prefixes.insert("focus-visible".to_string(), ".{class}:focus-visible {
  {css}
}".to_string());
        prefixes.insert("disabled".to_string(), ".{class}:disabled {
  {css}
}".to_string());
        prefixes.insert("visited".to_string(), ".{class}:visited {
  {css}
}".to_string());
        prefixes.insert("checked".to_string(), ".{class}:checked {
  {css}
}".to_string());
        prefixes.insert("dark".to_string(), "body.dark .{class} {
  {css}
}".to_string());
        prefixes.insert("sm".to_string(), "@media (min-width: 640px) {
  .{class} {
    {css}
  }
}".to_string());
        prefixes.insert("md".to_string(), "@media (min-width: 768px) {
  .{class} {
    {css}
  }
}".to_string());
        prefixes.insert("lg".to_string(), "@media (min-width: 1024px) {
  .{class} {
    {css}
  }
}".to_string());
        prefixes.insert("xl".to_string(), "@media (min-width: 1280px) {
  .{class} {
    {css}
  }
}".to_string());
        prefixes.insert("2xl".to_string(), "@media (min-width: 1536px) {
  .{class} {
    {css}
  }
}".to_string());
        prefixes
    };
}
