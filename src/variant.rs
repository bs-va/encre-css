use lazy_static::lazy_static;
use std::collections::BTreeMap;

pub const VARIANT_SEPARATOR: &str = ":";

lazy_static! {
    pub static ref VARIANTS: BTreeMap<&'static str, &'static str> = {
        // NOTE: If there is a variant starting with the same characters than another complete
        // variant, the first found will be the first in alphabetic order
        let mut variants = BTreeMap::new();

        // --- Pseudo element ---

        variants.insert("first-letter", "&::first-letter");
        variants.insert("first-line", "&::first-line");
        variants.insert("marker", "&::marker\n& *::marker");
        variants.insert("selection", "&::selection\n& *::selection");
        variants.insert("file", "&::file-selector-button");
        variants.insert("placeholder", "&::placeholder");
        variants.insert("backdrop", "&::backdrop");
        variants.insert("before", "&::before");
        variants.insert("after", "&::after");

        // --- Pseudo class ---

        // Positional
        variants.insert("first", "&:first-child");
        variants.insert("last", "&:last-child");
        variants.insert("only", "&:only-child");
        variants.insert("odd", "&:nth-child(odd)");
        variants.insert("even", "&:nth-child(even)");
        variants.insert("first-of-type", "&:first-of-type");
        variants.insert("last-of-type", "&:last-of-type");

        // State
        variants.insert("visited", "&:visited");
        variants.insert("target", "&:target");
        variants.insert("open", "&[open]");

        // Forms
        variants.insert("default", "&:default");
        variants.insert("checked", "&:checked");
        variants.insert("indeterminate", "&:indeterminate");
        variants.insert("placeholder-shown", "&:placeholder-shown");
        variants.insert("autofill", "&:autofill");
        variants.insert("required", "&:required");
        variants.insert("valid", "&:valid");
        variants.insert("invalid", "&:invalid");
        variants.insert("in-range", "&:in-range");
        variants.insert("out-of-range", "&:out-of-range");
        variants.insert("read-only", "&:read-only");

        // Content
        variants.insert("empty", "&:empty");

        // Interactive
        variants.insert("focus-within", "&:focus-within");
        variants.insert("hover", "&:hover");
        variants.insert("focus", "&:focus");
        variants.insert("focus-visible", "&:focus-visible");
        variants.insert("active", "&:active");
        variants.insert("enabled", "&:enabled");
        variants.insert("disabled", "&:disabled");

        // --- Direction ---

        variants.insert("ltr", "[dir=\"ltr\"] &");
        variants.insert("rtl", "[dir=\"rtl\"] &");

        // --- Reduced motion ---
        variants.insert("motion-safe", "@media (prefers-reduced-motion: no-preference)");
        variants.insert("motion-reduce", "@media (prefers-reduced-motion: reduce)");

        // --- Dark ---

        // TODO: Config file for choosing mode (`class` or `media`) and className
        /*variants.insert("dark", "body.dark .{class} {
  {css}
}");*/

        // --- Print ---

        variants.insert("print", "@media print");

        // --- Screen ---

        // TODO: Config file for screen breakpoints
        variants.insert("sm", "@media (min-width: 640px)");
        variants.insert("md", "@media (min-width: 768px)");
        variants.insert("lg", "@media (min-width: 1024px)");
        variants.insert("xl", "@media (min-width: 1280px)");
        variants.insert("2xl", "@media (min-width: 1536px)");

        // --- Orientation ---

        variants.insert("portrait", "@media (orientation: portrait)");
        variants.insert("landscape", "@media (orientation: landscape)");

        variants
    };
}
