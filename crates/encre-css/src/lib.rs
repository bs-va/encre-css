//! A TailwindCSS-compatible CSS generation library written in Rust
//!
//! # Example
//!
//! ```rust
//! use encre_css::{EncreGenerator, Config};
//!
//! let mut generator = EncreGenerator::from_config(Config::default());
//! // Or let mut generator = EncreGenerator::new("encre.toml".into()); if your current directory contains an `encre.toml` file
//! generator.scan_content(r#"class="bg-red-500""#);
//!
//! assert!(generator.generate().contains(r#".bg-red-500 {
//!   --en-bg-opacity: 1;
//!   background-color: rgb(239 68 68 / var(--en-bg-opacity));
//! }"#));
//! ```
pub mod plugins;
pub mod preflight;
pub mod selector;
pub mod utils;
pub mod variant;
pub mod config;
pub mod error;

use lazy_static::lazy_static;
use regex::Regex;
use std::{fs, iter, borrow::Cow, collections::BTreeMap, io::Read, path::PathBuf};
use wax::Glob;

#[cfg(not(target_arch = "wasm32"))]
use rayon::prelude::*;

use plugins::PLUGINS;
use preflight::ENCRE_PREFLIGHT_CSS;
use selector::Selector;
use variant::{Variant, init_variants};

pub use config::Config;
pub use error::Error;

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
    let val = if !URL_REGEX.is_match(val) {
        // Don't replace `_` if prefixed by a `\`
        val.replace("\\_", WILL_BE_REPLACED_BY_UNDERSCORE)
            .replace('_', " ")
            .replace(WILL_BE_REPLACED_BY_UNDERSCORE, "_")
    } else {
        val.to_string()
    };

    if val.contains("calc") {
        val.replace('-', " - ")
            .replace('+', " + ")
            .replace('/', " / ")
            .replace('*', "*")
    } else {
        val
    }
}

pub const VALID_PLUGIN_HINT: [&str; 4] = ["color", "length", "angle", "list"];

const WILL_BE_REPLACED_BY_CSS_SELECTOR: &str = "WILL_BE_REPLACED_BY_CSS_SELECTOR";

pub fn indent(val: String) -> String {
    val.replace('\n', "\n  ")
}

/// Main structure used to generate CSS from selectors
#[derive(Default)]
pub struct EncreGenerator {
    config: Config,
    variants: BTreeMap<Cow<'static, str>, Variant>,
    scanned_selectors_without_variant: Vec<Selector>,
    scanned_selectors_with_variant: Vec<Selector>,
}

impl EncreGenerator {
    /// Create a new [`EncreGenerator`] by trying to read a configuration file
    ///
    /// If the file does not exist, a warning will be emitted and the default configuration will be
    /// used
    pub fn new(path: PathBuf) -> Self {
        let config = match Config::from_file(path) {
            Ok(config) => config,
            Err(e) => {
                eprintln!("{}", e);
                Config::default()
            },
        };

        Self::from_config(config)
    }

    /// Create a new [`EncreGenerator`] using a given configuration
    ///
    /// The paths in the [`Config::content`] field of the configuration will be scanned
    pub fn from_config(config: Config) -> Self {
        let input = config.input.clone();

        let mut result = Self {
            variants: init_variants(&config),
            config,
            scanned_selectors_without_variant: vec![],
            scanned_selectors_with_variant: vec![],
        };

        // TODO: Use rayon to make this part parallel
        for path in input {
            result.scan_path(&path);
        }

        result
    }

    /// Add a new selector which will have its CSS generated
    ///
    /// This function automatically handles duplicated selectors
    pub fn add_selector(&mut self, val: &str) {
        let selector = Selector::new(val);

        if !selector.get_variants().is_empty() {
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

            if file.read_to_string(&mut file_content).is_ok() {
                self.scan_content(&file_content);
            }
            // TODO: Display a warning otherwise
        }
    }

    /// Scan all files in a path using the glob syntax
    pub fn scan_path(&mut self, glob_path: &PathBuf) {
        let (prefix, glob) = Glob::partitioned(glob_path.to_str().expect("failed to convert the glob to a PathBuf")).unwrap();

        if prefix == *glob_path {
            self.scan_files(iter::once(glob_path.clone()));
        } else {
            self.scan_files(
                glob.walk(prefix, usize::MAX)
                    .map(|e| e.unwrap().into_path()),
            );
        }
    }

    /// Generate the CSS styles needed based on the scanned selectors
    ///
    /// NOTE: Don't forget to scan selectors using either [scan_files] or [scan_content] or by
    /// adding individual selectors using [add_selector]
    ///
    /// [scan_files]: EncreGenerator::scan_files
    /// [scan_content]: EncreGenerator::scan_content
    /// [add_selector]: EncreGenerator::add_selector
    pub fn generate(&self) -> String {
        let selectors = [
            &self.scanned_selectors_without_variant,
            &self.scanned_selectors_with_variant,
        ];

        #[cfg(target_arch = "wasm32")]
        let iter = selectors.iter();

        #[cfg(not(target_arch = "wasm32"))]
        let iter = selectors.par_iter();

        let result = iter
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

                        let mut css_content = String::new();

                        if let Some(arbitrary_value) = arbitrary_value {
                            if plugin.is_matching_value(arbitrary_value.0, arbitrary_value.1)
                                && plugin.css_template_value(
                                    &to_css_value(arbitrary_value.1),
                                    &mut css_content,
                                )
                            {
                                return Some(self.gen_css_rule(selector, &css_content));
                            }
                        } else if plugin.get_css_for_modifier(
                            &self.config,
                            &selector.get_modifier(plugin.namespace()),
                            &mut css_content,
                        ) {
                            return Some(self.gen_css_rule(selector, &css_content));
                        }
                    }
                }

                None
            })
            .collect::<Vec<String>>();

        format!("{}{}", ENCRE_PREFLIGHT_CSS, result.join("\n\n"))
    }

    /// Generate a complete CSS rule (with a class selector, a rule content and, if requested, some
    /// pseudo-elements or `@media` queries)
    pub fn gen_css_rule(&self, selector: &Selector, css_content: &str) -> String {
        let mut css_selector = selector
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
            .replace('!', "\\!")
            .replace('%', "\\%");

        if css_selector.starts_with(char::is_numeric) {
            // CSS classes are not supposed to start with a number, we need to escape it
            css_selector.insert_str(0, "\\3");
        }

        // A CSS class starts with a `.`
        css_selector.insert(0, '.');

        let css_content = if selector.is_important() {
            css_content.replace(';', " !important;")
        } else {
            css_content.to_string()
        };

        let variants = selector.get_variants();
        if !variants.is_empty() {
            let rule = variants.iter().fold(
                format!(
                    "{} {{\n  {}\n}}",
                    WILL_BE_REPLACED_BY_CSS_SELECTOR,
                    indent(css_content),
                ),
                |acc, variant| {
                    let right_variant = if let Some(result) = self.variants.get(variant.as_str()) {
                        result
                    } else {
                        println!("Unknown variant: {}", variant);
                        return acc;
                    };

                    match right_variant {
                        Variant::PseudoClass(name) => {
                            css_selector.push_str(&format!(":{}", name));
                            acc
                        }
                        Variant::PseudoElement(name) => {
                            css_selector.push_str(&format!("::{}", name));
                            acc
                        }
                        Variant::WrapSelector(template) => {
                            css_selector = template.replace('&', &css_selector);
                            acc
                        }
                        Variant::AtRule(at_rule) => {
                            format!("{} {{\n  {}\n}}", at_rule, indent(acc),)
                        }
                    }
                },
            );

            rule.replace(WILL_BE_REPLACED_BY_CSS_SELECTOR, &css_selector)
        } else {
            format!("{} {{\n  {}\n}}", css_selector, indent(css_content))
        }
    }

    /// Forget the scanned selectors (useful when repeatedly calling [`EncreGenerator::generate`])
    pub fn clear_scanned_selectors(&mut self) {
        self.scanned_selectors_without_variant.clear();
        self.scanned_selectors_with_variant.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::DarkModeConfig;

    use std::borrow::Cow;
    use pretty_assertions::assert_eq;

    #[test]
    fn scan_content_test() {
        let mut generator = EncreGenerator::from_config(Config::default());
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
        let mut generator = EncreGenerator::from_config(Config::default());
        generator.add_selector("w-full");

        assert_eq!(
            generator.generate(),
            format!(
                r#"{}.w-full {{
  width: 100%;
}}"#,
                preflight::ENCRE_PREFLIGHT_CSS
            )
        );
    }

    #[test]
    fn gen_with_variant_selector_css_test() {
        let mut generator = EncreGenerator::from_config(Config::default());
        generator.add_selector("focus:w-full");

        assert_eq!(
            generator.generate(),
            format!(
                r#"{}.focus\:w-full:focus {{
  width: 100%;
}}"#,
                preflight::ENCRE_PREFLIGHT_CSS,
            )
        );
    }

    #[test]
    fn gen_selector_css_variants_test() {
        let mut generator = EncreGenerator::from_config(Config::default());
        generator.add_selector("sm:hover:bg-red-400");
        generator.add_selector("focus:hover:bg-red-600");
        generator.add_selector("active:rtl:bg-red-800");
        generator.add_selector("md:focus:selection:bg-blue-100");
        generator.add_selector("rtl:active:focus:lg:underline");
        generator.add_selector("print:ltr:xl:hover:focus:active:text-yellow-300");
        generator.add_selector("2xl:motion-safe:landscape:focus-within:visited:first:odd:checked:open:rtl:bg-purple-100");
        generator.add_selector("hover:file:bg-pink-600");
        generator.add_selector("file:hover:bg-pink-600");
        generator.add_selector("sm:before:target:content-[Hello_world!]");
        generator.add_selector("marker:selection:hover:bg-green-200");

        assert_eq!(
            generator.generate(),
            format!(
                r#"{}@media (min-width: 640px) {{
  .sm\:hover\:bg-red-400:hover {{
    --en-bg-opacity: 1;
    background-color: rgb(248 113 113 / var(--en-bg-opacity));
  }}
}}

.focus\:hover\:bg-red-600:hover:focus {{
  --en-bg-opacity: 1;
  background-color: rgb(220 38 38 / var(--en-bg-opacity));
}}

[dir="rtl"] .active\:rtl\:bg-red-800:active {{
  --en-bg-opacity: 1;
  background-color: rgb(153 27 27 / var(--en-bg-opacity));
}}

@media (min-width: 768px) {{
  .md\:focus\:selection\:bg-blue-100 *::selection, .md\:focus\:selection\:bg-blue-100::selection:focus {{
    --en-bg-opacity: 1;
    background-color: rgb(219 234 254 / var(--en-bg-opacity));
  }}
}}

@media (min-width: 1024px) {{
  [dir="rtl"] .rtl\:active\:focus\:lg\:underline:focus:active {{
    -webkit-text-decoration-line: underline;
    text-decoration-line: underline;
  }}
}}

@media print {{
  @media (min-width: 1280px) {{
    [dir="ltr"] .print\:ltr\:xl\:hover\:focus\:active\:text-yellow-300:active:focus:hover {{
      --en-text-opacity: 1;
      color: rgb(253 224 71 / var(--en-text-opacity));
    }}
  }}
}}

@media (min-width: 1536px) {{
  @media (prefers-reduced-motion: no-preference) {{
    @media (orientation: landscape) {{
      [dir="rtl"] .\32xl\:motion-safe\:landscape\:focus-within\:visited\:first\:odd\:checked\:open\:rtl\:bg-purple-100[open]:checked:nth-child(odd):first-child:visited:focus-within {{
        --en-bg-opacity: 1;
        background-color: rgb(243 232 255 / var(--en-bg-opacity));
      }}
    }}
  }}
}}

.hover\:file\:bg-pink-600::file-selector-button:hover {{
  --en-bg-opacity: 1;
  background-color: rgb(219 39 119 / var(--en-bg-opacity));
}}

.file\:hover\:bg-pink-600:hover::file-selector-button {{
  --en-bg-opacity: 1;
  background-color: rgb(219 39 119 / var(--en-bg-opacity));
}}

@media (min-width: 640px) {{
  .sm\:before\:target\:content-\[Hello_world\!\]:target::before {{
    --en-content: "Hello world!";
    content: var(--en-content);
  }}
}}

.marker\:selection\:hover\:bg-green-200:hover *::selection, .marker\:selection\:hover\:bg-green-200:hover::selection *::marker, .marker\:selection\:hover\:bg-green-200:hover *::selection, .marker\:selection\:hover\:bg-green-200:hover::selection::marker {{
  --en-bg-opacity: 1;
  background-color: rgb(187 247 208 / var(--en-bg-opacity));
}}"#,
                preflight::ENCRE_PREFLIGHT_CSS
            )
        );
    }

    #[test]
    fn gen_selector_css_negative_values_test() {
        let mut generator = EncreGenerator::from_config(Config::default());
        generator.add_selector("-translate-x-52");
        generator.add_selector("-mb-8");
        generator.add_selector("-hue-rotate-60");

        assert_eq!(
            generator.generate(),
            format!(r#"{}.-translate-x-52 {{
  --en-translate-x: -13rem;
  transform: translate(var(--en-translate-x), var(--en-translate-y)) rotate(var(--en-rotate)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scaleX(var(--en-scale-x)) scaleY(var(--en-scale-y));
}}

.-mb-8 {{
  margin-bottom: -2rem;
}}

.-hue-rotate-60 {{
  --en-hue-rotate: hue-rotate(-60deg);
  filter: var(--en-blur) var(--en-brightness) var(--en-contrast) var(--en-grayscale) var(--en-hue-rotate) var(--en-invert) var(--en-saturate) var(--en-sepia) var(--en-drop-shadow);
}}"#, preflight::ENCRE_PREFLIGHT_CSS));
    }

    #[test]
    fn gen_selector_css_prevent_duplication_test() {
        let mut generator = EncreGenerator::from_config(Config::default());
        generator.add_selector("bg-red-500");
        generator.add_selector("bg-red-500");
        generator.add_selector("bg-red-500");

        assert_eq!(
            generator.generate(),
            format!(r#"{}.bg-red-500 {{
  --en-bg-opacity: 1;
  background-color: rgb(239 68 68 / var(--en-bg-opacity));
}}"#, preflight::ENCRE_PREFLIGHT_CSS));
    }

    #[test]
    fn gen_selector_css_with_dark_variant_test() {
        let mut generator = EncreGenerator::from_config(Config::default());
        generator.add_selector("dark:mt-px");

        assert_eq!(
            generator.generate(),
            format!(r#"{}@media (prefers-color-scheme: dark) {{
  .dark\:mt-px {{
    margin-top: 1px;
  }}
}}"#, preflight::ENCRE_PREFLIGHT_CSS)
        );

        let mut config = Config::default();
        config.theme.dark_mode = DarkModeConfig::Class(Cow::from(".dark"));

        let mut generator = EncreGenerator::from_config(config);
        generator.add_selector("dark:mt-px");

        assert_eq!(
            generator.generate(),
            format!(r#"{}.dark .dark\:mt-px {{
  margin-top: 1px;
}}"#, preflight::ENCRE_PREFLIGHT_CSS)
        );
    }
}
