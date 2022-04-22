use super::Plugin;
use crate::utils::{default_colors, value_matchers::*};

use regex::Regex;

#[derive(Debug)]
pub struct TypographyColorPlugin;

impl Plugin for TypographyColorPlugin {
    fn namespace(&self) -> String {
        "text".to_string()
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "color" || is_matching_color(val)
    }

    fn css_template_value(&self, val: &str) -> String {
        if val.contains("--tw-opacity") {
            format!(
                "--tw-text-opacity: 1;
  color: {};",
                val.replace("--tw-opacity", "--tw-text-opacity")
            )
        } else {
            format!("color: {val};")
        }
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        default_colors::get(modifier).map(|c| self.css_template_value(&c))
    }
}

#[derive(Debug)]
pub struct TypographyOpacityPlugin;

impl Plugin for TypographyOpacityPlugin {
    fn namespace(&self) -> String {
        "text-opacity".to_string()
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        // Support all values
        if let Ok(opacity_value) = modifier.parse::<f32>() {
            Some(format!("--tw-text-opacity: {};", opacity_value / 100.))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct TypographyFontFamilyPlugin;

impl Plugin for TypographyFontFamilyPlugin {
    fn namespace(&self) -> String {
        "font".to_string()
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        val.split(',').all(|v| {
            if is_matching_generic_name(v) || is_matching_var(v) {
                true
            } else {
                !(v.contains('_')
                    && fancy_regex::Regex::new(r#"!/(['"])([^"']+)\1"#)
                        .unwrap()
                        .is_match(v)
                        .unwrap())
                    || Regex::new(r"^\d").unwrap().is_match(v)
            }
        })
    }

    fn css_template_value(&self, val: &str) -> String {
        format!("font-family: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        match modifier {
            "sans" => Some(r#"font-family: ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, "Noto Sans", sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji";"#.to_string()),
            "serif" => Some(r#"font-family: Georgia, Cambria, "Times New Roman", Times, serif;"#.to_string()),
            "mono" => Some(r#"font-family: Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;"#.to_string()),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct TypographyFontSizePlugin;

impl Plugin for TypographyFontSizePlugin {
    fn namespace(&self) -> String {
        "text".to_string()
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str) -> String {
        format!("font-size: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        match modifier {
            "xs" => Some("font-size: 0.75rem;
  line-height: 1rem;".to_string()),
            "sm" => Some("font-size: 0.875rem;
  line-height: 1.25rem;".to_string()),
            "base" => Some("font-size: 1rem;
  line-height: 1.5rem;".to_string()),
            "lg" => Some("font-size: 1.125rem;
  line-height: 1.75rem;".to_string()),
            "xl" => Some("font-size: 1.25rem;
  line-height: 1.75rem;".to_string()),
            "2xl" => Some("font-size: 1.5rem;
  line-height: 2rem;".to_string()),
            "3xl" => Some("font-size: 1.875rem;
  line-height: 2.25rem;".to_string()),
            "4xl" => Some("font-size: 2.25rem;
  line-height: 2.5rem;".to_string()),
            "5xl" => Some("font-size: 3rem;
  line-height: 1;".to_string()),
            "6xl" => Some("font-size: 3.75rem;
  line-height: 1;".to_string()),
            "7xl" => Some("font-size: 4.5rem;
  line-height: 1;".to_string()),
            "8xl" => Some("font-size: 6rem;
  line-height: 1;".to_string()),
            "9xl" => Some("font-size: 8rem;
  line-height: 1;".to_string()),
            _ => None,
        }
    }
}

/*pub fn init(selectors: &mut SelectorList) {
    selectors.register(
        "antialiased",
        "-webkit-font-smoothing: antialiased;
-moz-osx-font-smoothing: grayscale;"
            .to_string(),
    );
    selectors.register(
        "subpixel-antialiased",
        "-webkit-font-smoothing: auto;
-moz-osx-font-smoothing: auto;"
            .to_string(),
    );
    selectors.register("italic", "font-style: italic;".to_string());
    selectors.register("not-italic", "font-style: normal;".to_string());
    selectors.register("font-thin", "font-weight: 100;".to_string());
    selectors.register("font-extralight", "font-weight: 200;".to_string());
    selectors.register("font-light", "font-weight: 300;".to_string());
    selectors.register("font-normal", "font-weight: 400;".to_string());
    selectors.register("font-medium", "font-weight: 500;".to_string());
    selectors.register("font-semibold", "font-weight: 600;".to_string());
    selectors.register("font-bold", "font-weight: 700;".to_string());
    selectors.register("font-extrabold", "font-weight: 800;".to_string());
    selectors.register("font-black", "font-weight: 900;".to_string());
    selectors.register("normal-nums", "font-variant-numeric: normal;".to_string());
    selectors.register("ordinal", "font-variant-numeric: ordinal;".to_string());
    selectors.register(
        "slashed-zero",
        "font-variant-numeric: slashed-zero;".to_string(),
    );
    selectors.register(
        "lining-nums",
        "font-variant-numeric: lining-nums;".to_string(),
    );
    selectors.register(
        "oldstyle-nums",
        "font-variant-numeric: oldstyle-nums;".to_string(),
    );
    selectors.register(
        "proportional-nums",
        "font-variant-numeric: proportional-nums;".to_string(),
    );
    selectors.register(
        "tabular-nums",
        "font-variant-numeric: tabular-nums;".to_string(),
    );
    selectors.register(
        "diagonal-fractions",
        "font-variant-numeric: diagonal-fractions;".to_string(),
    );
    selectors.register(
        "stacked-fractions",
        "font-variant-numeric: stacked-fractions;".to_string(),
    );
    selectors.register("tracking-tighter", "letter-spacing: -0.05em;".to_string());
    selectors.register("tracking-tight", "letter-spacing: -0.025em;".to_string());
    selectors.register("tracking-normal", "letter-spacing: 0;".to_string());
    selectors.register("tracking-wide", "letter-spacing: 0.025em;".to_string());
    selectors.register("tracking-wider", "letter-spacing: 0.05em;".to_string());
    selectors.register("tracking-widest", "letter-spacing: 0.1em;".to_string());
    selectors.register("leading-none", "line-height: 1;".to_string());
    selectors.register("leading-tight", "line-height: 1.25;".to_string());
    selectors.register("leading-snug", "line-height: 1.375;".to_string());
    selectors.register("leading-normal", "line-height: 1.5;".to_string());
    selectors.register("leading-relaxed", "line-height: 1.625;".to_string());
    selectors.register("leading-loose", "line-height: 2;".to_string());
    selectors.register("leading-3", "line-height: .75rem;".to_string());
    selectors.register("leading-4", "line-height: 1rem;".to_string());
    selectors.register("leading-5", "line-height: 1.25rem;".to_string());
    selectors.register("leading-6", "line-height: 1.5rem;".to_string());
    selectors.register("leading-7", "line-height: 1.75rem;".to_string());
    selectors.register("leading-8", "line-height: 2rem;".to_string());
    selectors.register("leading-9", "line-height: 2.25rem;".to_string());
    selectors.register("leading-10", "line-height: 2.5rem;".to_string());
    selectors.register("list-none", "list-style-type: none;".to_string());
    selectors.register("list-disc", "list-style-type: disc;".to_string());
    selectors.register("list-decimal", "list-style-type: decimal;".to_string());
    selectors.register("list-inside", "list-style-position: inside;".to_string());
    selectors.register("list-outside", "list-style-position: outside;".to_string());
    selectors.register("text-left", "text-align: left;".to_string());
    selectors.register("text-center", "text-align: center;".to_string());
    selectors.register("text-right", "text-align: right;".to_string());
    selectors.register("text-justify", "text-align: justify;".to_string());
    selectors.register("underline", "text-decoration: underline;".to_string());
    selectors.register("line-through", "text-decoration: line-through;".to_string());
    selectors.register("no-underline", "text-decoration: none;".to_string());
    selectors.register("uppercase", "text-transform: uppercase;".to_string());
    selectors.register("lowercase", "text-transform: lowercase;".to_string());
    selectors.register("capitalize", "text-transform: capitalize;".to_string());
    selectors.register("normal-case", "text-transform: none;".to_string());
    selectors.register(
        "truncate",
        "overflow: hidden;
text-overflow: ellipsis;
white-space: nowrap;"
            .to_string(),
    );
    selectors.register("overflow-ellipsis", "text-overflow: ellipsis;".to_string());
    selectors.register("overflow-clip", "text-overflow: clip;".to_string());
    selectors.register("align-baseline", "vertical-align: baseline;".to_string());
    selectors.register("align-top", "vertical-align: top;".to_string());
    selectors.register("align-middle", "vertical-align: middle;".to_string());
    selectors.register("align-bottom", "vertical-align: bottom;".to_string());
    selectors.register("align-text-top", "vertical-align: text-top;".to_string());
    selectors.register(
        "align-text-bottom",
        "vertical-align: text-bottom;".to_string(),
    );
    selectors.register("whitespace-normal", "white-space: normal;".to_string());
    selectors.register("whitespace-nowrap", "white-space: nowrap;".to_string());
    selectors.register("whitespace-pre", "white-space: pre;".to_string());
    selectors.register("whitespace-pre-line", "white-space: pre-line;".to_string());
    selectors.register("whitespace-pre-wrap", "white-space: pre-wrap;".to_string());
    selectors.register(
        "break-normal",
        "overflow-wrap: normal;
word-break: normal;"
            .to_string(),
    );
    selectors.register("break-words", "overflow-wrap: break-word;".to_string());
    selectors.register("break-all", "word-break: break-all;".to_string());
}*/
