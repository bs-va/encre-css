use super::Plugin;
use crate::utils::{default_colors, value_matchers::*};

use lazy_static::lazy_static;
use regex::Regex;
use std::fmt::{Result, Write};

lazy_static! {
    static ref START_WITH_INT_REGEX: Regex = Regex::new(r"(?-u)^\d").unwrap();
}

#[derive(Debug)]
pub struct TypographyColorPlugin;

impl Plugin for TypographyColorPlugin {
    fn namespace(&self) -> &str {
        "text"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "color" || is_matching_color(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        if val.contains("--tw-opacity") {
            write!(
                css_content,
                "--tw-text-opacity: 1;
  color: {};",
                val.replace("--tw-opacity", "--tw-text-opacity")
            )
        } else {
            write!(css_content, "color: {val};")
        }
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if let Some(color) = default_colors::get(modifier) {
            self.css_template_value(&color, css_content)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct TypographyOpacityPlugin;

impl Plugin for TypographyOpacityPlugin {
    fn namespace(&self) -> &str {
        "text-opacity"
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        // NOTE: Not-compatible with TailwindCSS, support all values
        if let Ok(opacity_value) = modifier.parse::<f32>() {
            write!(css_content, "--tw-text-opacity: {};", opacity_value / 100.)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct TypographyFontFamilyPlugin;

impl Plugin for TypographyFontFamilyPlugin {
    fn namespace(&self) -> &str {
        "font"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        val.split(',').all(|v| {
            if is_matching_generic_name(v) || is_matching_var(v) {
                true
            } else {
                !START_WITH_INT_REGEX.is_match(v)
            }
        })
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        // NOTE: Not-compatible with TailwindCSS, it is not needed to add quotes to fonts
        // containing spaces, they are added later
        write!(
            css_content,
            "font-family: {maybe_quote}{val}{maybe_quote};",
            maybe_quote = if val.contains(' ') { "\"" } else { "" }
        )
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        match modifier {
            "sans" => write!(
                css_content,
                r#"font-family: ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, "Noto Sans", sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol", "Noto Color Emoji";"#
            ),
            "serif" => write!(
                css_content,
                r#"font-family: Georgia, Cambria, "Times New Roman", Times, serif;"#
            ),
            "mono" => write!(
                css_content,
                r#"font-family: Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace;"#
            ),
            _ => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct TypographyFontSizePlugin;

impl Plugin for TypographyFontSizePlugin {
    fn namespace(&self) -> &str {
        "text"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "font-size: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        match modifier {
            "xs" => write!(
                css_content,
                "font-size: 0.75rem;
  line-height: 1rem;"
            ),
            "sm" => write!(
                css_content,
                "font-size: 0.875rem;
  line-height: 1.25rem;"
            ),
            "base" => write!(
                css_content,
                "font-size: 1rem;
  line-height: 1.5rem;"
            ),
            "lg" => write!(
                css_content,
                "font-size: 1.125rem;
  line-height: 1.75rem;"
            ),
            "xl" => write!(
                css_content,
                "font-size: 1.25rem;
  line-height: 1.75rem;"
            ),
            "2xl" => write!(
                css_content,
                "font-size: 1.5rem;
  line-height: 2rem;"
            ),
            "3xl" => write!(
                css_content,
                "font-size: 1.875rem;
  line-height: 2.25rem;"
            ),
            "4xl" => write!(
                css_content,
                "font-size: 2.25rem;
  line-height: 2.5rem;"
            ),
            "5xl" => write!(
                css_content,
                "font-size: 3rem;
  line-height: 1;"
            ),
            "6xl" => write!(
                css_content,
                "font-size: 3.75rem;
  line-height: 1;"
            ),
            "7xl" => write!(
                css_content,
                "font-size: 4.5rem;
  line-height: 1;"
            ),
            "8xl" => write!(
                css_content,
                "font-size: 6rem;
  line-height: 1;"
            ),
            "9xl" => write!(
                css_content,
                "font-size: 8rem;
  line-height: 1;"
            ),
            _ => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct TypographyFontWeightPlugin;

impl Plugin for TypographyFontWeightPlugin {
    fn namespace(&self) -> &str {
        "font"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        ["normal", "bold", "lighter", "bolder"].contains(&val)
            || is_matching_number(val)
            || is_matching_var(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "font-weight: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        match modifier {
            "thin" => self.css_template_value("100", css_content),
            "extralight" => self.css_template_value("200", css_content),
            "light" => self.css_template_value("300", css_content),
            "normal" => self.css_template_value("400", css_content),
            "medium" => self.css_template_value("500", css_content),
            "semibold" => self.css_template_value("600", css_content),
            "bold" => self.css_template_value("700", css_content),
            "extrabold" => self.css_template_value("800", css_content),
            "black" => self.css_template_value("900", css_content),
            _ => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct TypographyTextAlignmentPlugin;

impl Plugin for TypographyTextAlignmentPlugin {
    fn namespace(&self) -> &str {
        "text"
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        match modifier {
            "left" => write!(css_content, "text-align: left;"),
            "center" => write!(css_content, "text-align: center;"),
            "right" => write!(css_content, "text-align: right;"),
            "justify" => write!(css_content, "text-align: justify;"),
            _ => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct TypographyTextTransformPlugin;

impl Plugin for TypographyTextTransformPlugin {
    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        match modifier {
            "uppercase" => write!(css_content, "text-transform: uppercase;"),
            "lowercase" => write!(css_content, "text-transform: lowercase;"),
            "capitalize" => write!(css_content, "text-transform: capitalize;"),
            "normal-case" => write!(css_content, "text-transform: none;"),
            _ => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct TypographyTrackingPlugin;

impl Plugin for TypographyTrackingPlugin {
    fn namespace(&self) -> &str {
        "tracking"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        val == "normal" || is_matching_length(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "letter-spacing: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        match modifier {
            "tighter" => self.css_template_value("-0.05em", css_content),
            "tight" => self.css_template_value("-0.025em", css_content),
            "normal" => self.css_template_value("0", css_content),
            "wide" => self.css_template_value("0.025em", css_content),
            "wider" => self.css_template_value("0.05em", css_content),
            "widest" => self.css_template_value("0.1em", css_content),
            _ => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct TypographyLeadingPlugin;

impl Plugin for TypographyLeadingPlugin {
    fn namespace(&self) -> &str {
        "leading"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        // https://developer.mozilla.org/en-US/docs/Web/CSS/line-height#values
        val == "normal"
            || is_matching_float(val)
            || is_matching_length(val)
            || is_matching_percentage(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "line-height: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        match modifier {
            "none" => self.css_template_value("1", css_content),
            "tight" => self.css_template_value("1.25", css_content),
            "snug" => self.css_template_value("1.375", css_content),
            "normal" => self.css_template_value("1.5", css_content),
            "relaxed" => self.css_template_value("1.625", css_content),
            "loose" => self.css_template_value("2", css_content),
            "3" => self.css_template_value(".75rem", css_content),
            "4" => self.css_template_value("1rem", css_content),
            "5" => self.css_template_value("1.25rem", css_content),
            "6" => self.css_template_value("1.5rem", css_content),
            "7" => self.css_template_value("1.75rem", css_content),
            "8" => self.css_template_value("2rem", css_content),
            "9" => self.css_template_value("2.25rem", css_content),
            "10" => self.css_template_value("2.5rem", css_content),
            _ => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct TypographyItalicPlugin;

impl Plugin for TypographyItalicPlugin {
    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        match modifier {
            "italic" => write!(css_content, "font-style: italic;"),
            "no-italic" => write!(css_content, "font-style: normal;"),
            _ => Ok(()),
        }
    }
}

#[derive(Debug)]
pub struct TypographyTextDecorationPlugin;

impl Plugin for TypographyTextDecorationPlugin {
    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if ["underline", "overline", "line-through", "no-underline"].contains(&modifier) {
            write!(css_content, "text-decoration-line: {modifier};")
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct TypographyTextDecorationColorPlugin;

impl Plugin for TypographyTextDecorationColorPlugin {
    fn namespace(&self) -> &str {
        "decoration"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "color" || is_matching_color(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        if val.contains("--tw-opacity") {
            write!(
                css_content,
                "-webkit-text-decoration-color: {color};
  text-decoration-color: {color};",
                color = val.replace(" / var(--tw-opacity)", "")
            )
        } else {
            write!(
                css_content,
                "-webkit-text-decoration-color: {val};
  text-decoration-color: {val};"
            )
        }
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if let Some(color) = default_colors::get(modifier) {
            self.css_template_value(&color, css_content)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct TypographyTextDecorationStylePlugin;

impl Plugin for TypographyTextDecorationStylePlugin {
    fn namespace(&self) -> &str {
        "decoration"
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if ["solid", "double", "dotted", "dashed", "wavy"].contains(&modifier) {
            write!(css_content, "text-decoration-style: {modifier};")
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct TypographyTextDecorationThicknessPlugin;

impl Plugin for TypographyTextDecorationThicknessPlugin {
    fn namespace(&self) -> &str {
        "decoration"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        hint == "length"
            || hint == "percentage"
            || ["auto", "from-font"].contains(&val)
            || is_matching_length(val)
            || is_matching_percentage(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "text-decoration-thickness: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if ["auto", "from-font"].contains(&modifier) {
            return self.css_template_value(modifier, css_content);
        }

        // NOTE: Not-compatible with TailwindCSS, support all values
        if let Ok(thickness) = modifier.parse::<usize>() {
            self.css_template_value(&format!("{thickness}px"), css_content)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct TypographyTextDecorationOffsetPlugin;

impl Plugin for TypographyTextDecorationOffsetPlugin {
    fn namespace(&self) -> &str {
        "underline"
    }

    fn is_matching_value(&self, hint: &str, val: &str) -> bool {
        // TODO: Is it useful to match the hint against `length` AND `percentage`?
        hint == "length"
            || hint == "percentage"
            || is_matching_auto(val)
            || is_matching_length(val)
            || is_matching_percentage(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        write!(css_content, "text-underline-offset: {val};")
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if is_matching_auto(modifier) {
            return self.css_template_value("auto", css_content);
        }

        // NOTE: Not-compatible with TailwindCSS, support all values
        if let Ok(offset) = modifier.parse::<usize>() {
            self.css_template_value(&format!("{offset}px"), css_content)
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
pub struct TypographyContentPlugin;

impl Plugin for TypographyContentPlugin {
    fn namespace(&self) -> &str {
        "content"
    }

    fn is_matching_value(&self, _hint: &str, _val: &str) -> bool {
        true
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> Result {
        // NOTE: Not-compatible with TailwindCSS, it is not needed to add quotes to `content`
        // containing spaces, they are added later
        write!(css_content, "content: \"{val}\";")
    }

    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> Result {
        if modifier == "none" {
            self.css_template_value("none", css_content)
        } else {
            Ok(())
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
    selectors.register("list-none", "list-style-type: none;".to_string());
    selectors.register("list-disc", "list-style-type: disc;".to_string());
    selectors.register("list-decimal", "list-style-type: decimal;".to_string());
    selectors.register("list-inside", "list-style-position: inside;".to_string());
    selectors.register("list-outside", "list-style-position: outside;".to_string());
    selectors.register("underline", "text-decoration: underline;".to_string());
    selectors.register("line-through", "text-decoration: line-through;".to_string());
    selectors.register("no-underline", "text-decoration: none;".to_string());
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
