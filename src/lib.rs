//! A TailwindCSS compatible CSS generation library written in Rust
//!
//! # Example
//!
//! ```rust
//! use tailwind_rs::TailwindGenerator;
//!
//! let mut generator = TailwindGenerator::new();
//! generator.scan_content(r#"class="bg-red-500""#);
//!
//! assert!(generator.generate().contains(r#".bg-red-500 {
//!   --tw-bg-opacity: 1;
//!   background-color: rgb(239 68 68 / var(--tw-bg-opacity));
//! }"#));
//! ```
pub mod plugins;
pub mod preflight;
pub mod selector;
pub mod utils;
pub mod variant;

use lazy_static::lazy_static;
use rayon::prelude::*;
use regex::Regex;
use std::{fs, io::Read, path::PathBuf};

use plugins::PLUGINS;
use preflight::TAILWIND_PREFLIGHT_CSS;
use selector::Selector;
use variant::VARIANTS;

// TODO features:
// - Cache (dedup directly by scanning in all files at once (+ use rayon later))
// - Variant stacking (instead let variant = split.next()..., reverse the iterator and collect all
// variants)
// - Find changed files (using timestamp of generated files and timestamp of source files)
// - Real prefix (like tw-)???
// - CSS variant for dark: configurable
// - Configurable preflight
// - Support a safelist in the configuration file

lazy_static! {
    static ref SPLIT_REGEX: Regex = Regex::new(r#"(?-u)[\s'"`;>=]+"#).unwrap();
    static ref URL_REGEX: Regex = Regex::new("^url\\(.*\\)$").unwrap();
}

const WILL_BE_REPLACED_BY_UNDERSCORE: &str = "WILL-BE-REPLACED-BY-UNDERSCORE";

/// Convert an arbitrary value into a CSS value
///
///  -  `_` (underscores) are converted to ` ` (spaces) (not in `url`s)
pub fn to_css_value(val: &str) -> String {
    // Don't replace `_` if it is a URL
    //
    // TODO: Do the same for values **containing** an url (e.g. 10px_5px_1px_2px_url('/hello/world.png'))
    if !URL_REGEX.is_match(val) {
        // Don't replace `_` if prefixed by a `\`
        val.replace("\\_", WILL_BE_REPLACED_BY_UNDERSCORE)
            .replace('_', " ")
            .replace(WILL_BE_REPLACED_BY_UNDERSCORE, "_")
    } else {
        val.to_string() // TODO: Prevent allocating
    }

    // TODO: Add spaces around operators inside calc() that do not follow an operator
    /* or '('.
    return value.replace(
        /(-?\d*\.?\d(?!\b-.+[,)](?![^+\-/*])\D)(?:%|[a-z]+)?|\))([+\-/*])/g,
        '$1 $2 '
    )
    }*/*/*/
}

/// <https://v2.tailwindcss.com/docs/just-in-time-mode#arbitrary-value-support>
pub const VALID_PLUGIN_HINT: [&str; 4] = ["color", "length", "angle", "list"];

/// Generate a complete CSS rule (with a class selector, a rule content and, if requested, some
/// pseudo-elements or `@media` queries)
pub fn gen_css_rule(selector: &Selector, css_content: &str) -> String {
    let css_selector = selector
        .to_string()
        .replace('[', "\\[")
        .replace(']', "\\]")
        .replace('/', "\\/")
        .replace('\"', "\\\"")
        .replace('\'', "\\'")
        .replace('(', "\\(")
        .replace(')', "\\)")
        .replace('#', "\\#")
        .replace(':', "\\:")
        .replace(',', r"\2c ")
        .replace('.', "\\.")
        .replace('!', "\\!");

    let css_content = if selector.is_important() {
        css_content.replace(';', " !important;")
    } else {
        css_content.to_string() // TODO: Prevent?
    };

    if let Some(ref variant) = selector.get_variant() {
        let with_variant = if let Some(result) = VARIANTS.get(variant.as_str()) {
            result
        } else {
            panic!("Unknown variant: {}", variant);
        };

        with_variant
            .replace("{class}", &css_selector)
            .replace("{css}", &css_content)
    } else {
        format!(".{} {{\n  {}\n}}", css_selector, &css_content)
    }
}

/// Main structure used to generate CSS from selectors
#[derive(Default)]
pub struct TailwindGenerator {
    scanned_selectors_without_variant: Vec<Selector>,
    scanned_selectors_with_variant: Vec<Selector>,
}

impl TailwindGenerator {
    pub fn new() -> Self {
        Self {
            scanned_selectors_without_variant: vec![],
            scanned_selectors_with_variant: vec![],
        }
    }

    /// Add a new selector which will have its CSS generated
    ///
    /// This function automatically handles duplicated selectors
    pub fn add_selector(&mut self, val: &str) {
        let selector = Selector::new(val);

        if selector.get_variant().is_some() {
            if !self.scanned_selectors_with_variant.contains(&selector) {
                self.scanned_selectors_with_variant.push(selector);
            }
        } else if !self.scanned_selectors_without_variant.contains(&selector) {
            self.scanned_selectors_without_variant.push(selector);
        }
    }

    /// Scan the content of a file and store all the selectors found
    pub fn scan_content(&mut self, content: &str) {
        for val in SPLIT_REGEX.split(content) {
            self.add_selector(val);
        }
    }

    /// Scan all files given and store all the selectors found
    pub fn scan_files(&mut self, files: impl Iterator<Item = PathBuf>) {
        let mut file_content: String = String::new();

        for file in files {
            let mut file = fs::File::open(file).unwrap();
            file_content.clear();
            file.read_to_string(&mut file_content).unwrap();
            self.scan_content(&file_content);
        }
    }

    /// Generate the CSS styles needed based on the scanned selectors
    ///
    /// NOTE: Don't forget to scan selectors using either [scan_files] or [scan_content] or by
    /// adding individual selectors using [add_selector]
    ///
    /// [scan_files]: TailwindGenerator::scan_files
    /// [scan_content]: TailwindGenerator::scan_content
    /// [add_selector]: TailwindGenerator::add_selector
    pub fn generate(&self) -> String {
        let result = [
            &self.scanned_selectors_without_variant,
            &self.scanned_selectors_with_variant,
        ]
        .par_iter()
        .flat_map(|v| *v)
        .filter_map(|selector| {
            // Find the right plugin to handle this selector (if the resulting CSS is valid,
            // the plugin is good)
            for plugin in PLUGINS.iter() {
                if selector.check_namespace(plugin.namespace()) {
                    let maybe_arbitrary_value = selector.get_arbitrary_value();
                    let arbitrary_value = if let Some(ref arbitrary_value) = maybe_arbitrary_value {
                        let mut split = arbitrary_value.split(':');
                        let maybe_hint = split.next().unwrap();

                        if maybe_hint == arbitrary_value {
                            // No plugin hint
                            Some(("", maybe_hint))
                        } else {
                            let val = split.next();

                            if let Some(val) = val {
                                if VALID_PLUGIN_HINT.contains(&maybe_hint) {
                                    // Valid! Return (hint, stripped arbitrary value)
                                    Some((maybe_hint, val))
                                } else {
                                    // Unknown plugin hint (like `bg-[sth:#333]`)
                                    // TODO: Display a warning
                                    Some(("", val))
                                }
                            } else {
                                // Malformed arbitrary value (like just `bg-[color:]`)
                                // TODO: Display a warning
                                Some(("", maybe_hint))
                            }
                        }
                    } else {
                        None
                    };

                    if let Some(arbitrary_value) = arbitrary_value {
                        let mut css_content = String::new();

                        if plugin.is_matching_value(arbitrary_value.0, arbitrary_value.1)
                            && plugin.css_template_value(
                                &to_css_value(arbitrary_value.1),
                                &mut css_content,
                            )
                        {
                            return Some(gen_css_rule(selector, &css_content));
                        }
                    } else {
                        let mut css_content = String::new();

                        if plugin.get_css_for_modifier(
                            &selector.get_modifier(plugin.namespace()),
                            &mut css_content,
                        ) {
                            return Some(gen_css_rule(selector, &css_content));
                        }
                    }
                }
            }

            None
        })
        .collect::<Vec<String>>();

        format!("{}{}", TAILWIND_PREFLIGHT_CSS, result.join("\n\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn scan_content_test() {
        let mut generator = TailwindGenerator::new();
        generator.scan_content(
            r#"<div class="w-full h-full absolute bg-blue-500 foo-bar sm:focus:ring hover:bg-black border-[#333] text-[color:var(--hello)]"></div>"#
        );

        assert_eq!(
            generator.scanned_selectors_without_variant,
            vec![
                Selector::new("<div"),
                Selector::new("class"),
                Selector::new("w-full"),
                Selector::new("h-full"),
                Selector::new("absolute"),
                Selector::new("bg-blue-500"),
                Selector::new("foo-bar"),
                Selector::new("border-[#333]"),
                Selector::new("text-[color:var(--hello)]"),
                Selector::new("</div"),
                Selector::new(""), // TODO: Why?
            ]
        );

        assert_eq!(
            generator.scanned_selectors_with_variant,
            vec![
                Selector::new("sm:focus:ring"),
                Selector::new("hover:bg-black")
            ]
        );
    }

    #[test]
    fn gen_selector_css_test() {
        let mut generator = TailwindGenerator::new();
        generator.add_selector("w-full");

        assert_eq!(
            generator.generate(),
            format!(
                r#"{}.w-full {{
  width: 100%;
}}"#,
                preflight::TAILWIND_PREFLIGHT_CSS
            )
        );
    }

    #[test]
    fn gen_with_variant_selector_css_test() {
        let mut generator = TailwindGenerator::new();
        generator.add_selector("focus:w-full");

        assert_eq!(
            generator.generate(),
            format!(
                r#"{}.focus\:w-full:focus {{
  width: 100%;
}}"#,
                preflight::TAILWIND_PREFLIGHT_CSS,
            )
        );
    }

    /*#[test]
    fn gen_css_from_files_test() {
        use std::{iter, fs};

        let mut generator = TailwindGenerator::new();
        generator.scan_files(iter::once("tests/fixtures/index.html".into()));

        assert_eq!(
            generator.generate(),
            fs::read_to_string("tests/fixtures/should_be_generated.css").unwrap(),
        );
    }*/
}
