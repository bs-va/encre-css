use lazy_static::lazy_static;
use std::collections::BTreeMap;

pub const VARIANT_SEPARATOR: &str = ":";

lazy_static! {
    pub static ref VARIANTS: BTreeMap<String, String> = {
        // NOTE: If there is a variant starting with the same characters than another complete
        // variant, the first found will be the first in alphabetic order
        //
        // TODO: Support more variants https://tailwindcss.com/docs/hover-focus-and-other-states#highlighted-text
        let mut variants = BTreeMap::new();
        variants.insert("hover".to_string(), ".{class}:hover {
  {css}
}".to_string());
        variants.insert("focus".to_string(), ".{class}:focus {
  {css}
}".to_string());
        variants.insert("active".to_string(), ".{class}:active {
  {css}
}".to_string());
        variants.insert("focus-within".to_string(), ".{class}:focus-within {
  {css}
}".to_string());
        variants.insert("focus-visible".to_string(), ".{class}:focus-visible {
  {css}
}".to_string());
        variants.insert("disabled".to_string(), ".{class}:disabled {
  {css}
}".to_string());
        variants.insert("visited".to_string(), ".{class}:visited {
  {css}
}".to_string());
        variants.insert("checked".to_string(), ".{class}:checked {
  {css}
}".to_string());
        variants.insert("dark".to_string(), "body.dark .{class} {
  {css}
}".to_string());
        variants.insert("sm".to_string(), "@media (min-width: 640px) {
  .{class} {
    {css}
  }
}".to_string());
        variants.insert("md".to_string(), "@media (min-width: 768px) {
  .{class} {
    {css}
  }
}".to_string());
        variants.insert("lg".to_string(), "@media (min-width: 1024px) {
  .{class} {
    {css}
  }
}".to_string());
        variants.insert("xl".to_string(), "@media (min-width: 1280px) {
  .{class} {
    {css}
  }
}".to_string());
        variants.insert("2xl".to_string(), "@media (min-width: 1536px) {
  .{class} {
    {css}
  }
}".to_string());
        variants
    };
}
