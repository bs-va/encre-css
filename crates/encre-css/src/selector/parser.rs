use super::{Modifier, Selector, Variant, VariantType};
use crate::{
    config::{Config, BUILTIN_PLUGINS, BUILTIN_VARIANTS},
    error::{ParseError, ParseErrorKind},
    generator::ContextCanHandle,
    plugins::css_property::CssPropertyPlugin,
    utils::split_ignore_arbitrary,
};

use std::{borrow::Cow, ops::Range};

pub(crate) const ARBITRARY_START: char = '[';
pub(crate) const ARBITRARY_END: char = ']';
pub(crate) const GROUP_START: char = '(';
pub(crate) const GROUP_END: char = ')';
pub(crate) const ESCAPE: char = '\\';
const GROUP_SELECTOR_SEPARATOR: char = ',';
const VARIANT_SEPARATOR: char = ':';
const MODIFIER_SEPARATOR: char = '-';
const HINT_SEPARATOR: char = ':';
const NEGATIVE_FLAG: char = '-';
const IMPORTANT_FLAG: char = '!';

const WILL_BE_REPLACED_BY_UNDERSCORE: &str = "WILL-BE-REPLACED-BY-UNDERSCORE";

const VALID_PLUGIN_HINT: [&str; 13] = [
    "color",
    "length",
    "line-width",
    "image",
    "url",
    "position",
    "percentage",
    "number",
    "generic-name",
    "family-name",
    "absolute-size",
    "relative-size",
    "shadow",
];

/// Remove the first and last character of a string.
pub(crate) fn unwrap_string(val: &mut &str) {
    let len = val.len();
    *val = &val[1..len - 1];
}

/// Remove escape characters from an arbitrary values.
///
/// - `\[` is replaced with `[`
/// - `\]` is replaced with `]`
/// - `\(` is replaced with `(`
/// - `\)` is replaced with `)`
pub(crate) fn unescape(mut val: Cow<str>) -> Cow<str> {
    if val.contains("\\[") {
        val = Cow::from(val.replace("\\[", "["));
    }

    if val.contains("\\]") {
        val = Cow::from(val.replace("\\]", "]"));
    }

    if val.contains("\\(") {
        val = Cow::from(val.replace("\\(", "("));
    }

    if val.contains("\\)") {
        val = Cow::from(val.replace("\\)", ")"));
    }

    val
}

/// Replace all underscores with spaces (not in `url()` or if the underscore is prefixed by a backslash).
pub(crate) fn underscores_to_spaces(mut val: Cow<str>) -> Cow<str> {
    // Don't replace `_` if it is a URL
    if val.contains("url(") {
        // For the `CursorPlugin`, `ContentPlugin` and `ImagePlugin` plugins, we need to keep underscores in URLs
        val.split("url(")
            .map(|p| {
                let result = if let Some(sub) = p.strip_prefix('\'') {
                    sub.find('\'').map(|index| p.split_at(index + 3))
                } else if let Some(sub) = p.strip_prefix('"') {
                    sub.find('"').map(|index| p.split_at(index + 3))
                } else {
                    p.find(')').map(|index| p.split_at(index + 1))
                };

                if let Some((before, after)) = result {
                    format!("url({}{}", before, after.replace('_', " "))
                } else {
                    p.replace('_', " ")
                }
            })
            .collect()
    } else {
        // Replace `_` with ` ` (spaces) if not prefixed by a backslash
        if val.contains('_') {
            if val.contains("\\_") {
                val = Cow::from(val.replace("\\_", WILL_BE_REPLACED_BY_UNDERSCORE));
            }

            val = Cow::from(val.replace('_', " "));

            if val.contains(WILL_BE_REPLACED_BY_UNDERSCORE) {
                val = Cow::from(val.replace(WILL_BE_REPLACED_BY_UNDERSCORE, "_"));
            }
        }

        val
    }
}

/// Convert an arbitrary value into a CSS value.
///
///  -  `_` (underscores) are converted to ` ` (spaces) (not in `url`s or if prefixed by a backslash);
///  - Spaces are added around operators in the `calc` CSS function.
///  - The value is unescaped (see [`unescape`])
pub(crate) fn to_css_value(val: &str) -> Cow<str> {
    let mut val = underscores_to_spaces(Cow::from(val));

    // Add spaces around operators in the `calc` CSS function
    if val.contains("calc") {
        val = Cow::from(
            val.split(' ')
                .map(|v| {
                    if v.starts_with("calc(") {
                        Cow::from(
                            v.replace('-', " - ")
                                .replace('+', " + ")
                                .replace('/', " / ")
                                .replace('*', " * "),
                        )
                    } else {
                        Cow::from(v)
                    }
                })
                .collect::<Vec<Cow<str>>>()
                .join(" "),
        );
    }

    unescape(val)
}

pub(crate) fn parse<'a>(
    val: &'a str,
    span: Option<Range<usize>>,
    full_class: Option<&'a str>,
    config: &Config,
) -> Vec<Result<Selector<'a>, ParseError<'a>>> {
    // The shortest selector is `m1`
    if val.len() < 2 {
        return vec![Err(ParseError::new(
            span.unwrap_or(0..val.len()),
            ParseErrorKind::TooShort(val),
        ))];
    }

    parse_recursive(val, span, full_class, config)
}

#[allow(clippy::too_many_lines)]
fn parse_recursive<'a>(
    val: &'a str,
    span: Option<Range<usize>>,
    full_class: Option<&'a str>,
    config: &Config,
) -> Vec<Result<Selector<'a>, ParseError<'a>>> {
    let span = span.unwrap_or(0..val.len());

    // Parse variants
    let mut remaining = (0, "");

    let variants = {
        let custom_variants = config.get_custom_variants();
        let mut iter = split_ignore_arbitrary(val, VARIANT_SEPARATOR, true).peekable();
        let mut variants = Vec::with_capacity(iter.size_hint().0);

        while let Some(mut part) = iter.next() {
            if iter.peek().is_none() {
                remaining = part;
                continue;
            }

            let (is_arbitrary, variant) = {
                if part.1.starts_with(ARBITRARY_START) && part.1.ends_with(ARBITRARY_END) {
                    unwrap_string(&mut part.1);
                    (true, part.1)
                } else {
                    (false, part.1)
                }
            };

            if is_arbitrary {
                variants.push(Variant::Arbitrary(underscores_to_spaces(unescape(
                    Cow::from(variant),
                ))));
            } else if let Some((order, variant)) = BUILTIN_VARIANTS.get(variant) {
                variants.push(Variant::Builtin(*order, variant.clone()));
            } else if let Some((order, variant)) = custom_variants
                .iter()
                .enumerate()
                .find(|(_, v)| v.0 == variant)
            {
                variants.push(Variant::Builtin(
                    BUILTIN_VARIANTS.len() + order,
                    variant.1.clone(),
                ));
            } else {
                // Maybe a parent or peer variant
                if let Some(group_variant) = variant.strip_prefix("group-") {
                    if let Some((order, VariantType::PseudoClass(class))) =
                        BUILTIN_VARIANTS.get(group_variant)
                    {
                        variants.push(Variant::Builtin(order + 1000, VariantType::Group(class)));
                    }
                } else if let Some(peer_not_variant) = variant.strip_prefix("peer-not-") {
                    if let Some((order, VariantType::PseudoClass(class))) =
                        BUILTIN_VARIANTS.get(peer_not_variant)
                    {
                        variants.push(Variant::Builtin(order + 2000, VariantType::PeerNot(class)));
                    }
                } else if let Some(peer_variant) = variant.strip_prefix("peer-") {
                    if let Some((order, VariantType::PseudoClass(class))) =
                        BUILTIN_VARIANTS.get(peer_variant)
                    {
                        variants.push(Variant::Builtin(order + 3000, VariantType::Peer(class)));
                    }
                } else {
                    return vec![Err(ParseError::new(
                        span,
                        ParseErrorKind::UnknownVariant(variant, val),
                    ))];
                }
            }
        }
        variants
    };

    if remaining.1.is_empty() {
        return vec![Err(ParseError::new(
            span,
            ParseErrorKind::VariantsWithoutModifier(val),
        ))];
    }

    // Parse the namespace and modifier
    if remaining.1.starts_with(GROUP_START) && remaining.1.ends_with(GROUP_END) {
        // Variant group (selectors are separated by `,`), we need to parse child selectors
        unwrap_string(&mut remaining.1);

        if remaining.1.is_empty() {
            return vec![Err(ParseError::new(
                span,
                ParseErrorKind::VariantsWithoutModifier(val),
            ))];
        }

        split_ignore_arbitrary(remaining.1, GROUP_SELECTOR_SEPARATOR, true)
            .flat_map(|(i, sub_selector)| {
                #[allow(clippy::range_plus_one)]
                let mut new_selectors = parse_recursive(
                    sub_selector,
                    Some(
                        span.start + remaining.0 + i + 1
                            ..span.start + remaining.0 + i + sub_selector.len() + 1,
                    ),
                    Some(if let Some(full_class) = full_class {
                        full_class
                    } else {
                        val
                    }),
                    config,
                );

                // Merge the common variants with each child selector variant list
                for selector in new_selectors.iter_mut().flatten() {
                    selector.variants.extend(variants.iter().cloned());
                }

                new_selectors
            })
            .collect()
    } else {
        // Child selector
        let is_important = if let Some(new_remaining) = remaining.1.strip_prefix(IMPORTANT_FLAG) {
            remaining = (remaining.0, new_remaining);
            true
        } else {
            false
        };

        let is_negative = if let Some(new_remaining) = remaining.1.strip_prefix(NEGATIVE_FLAG) {
            remaining = (remaining.0, new_remaining);
            true
        } else {
            false
        };

        if remaining.1.starts_with(ARBITRARY_START) && remaining.1.ends_with(ARBITRARY_END) {
            // Arbitrary CSS property (without namespace)
            let plugin = &CssPropertyPlugin;

            vec![Ok(Selector {
                // Arbitrary properties will be placed at the end of the CSS
                order: BUILTIN_PLUGINS.len() + config.custom_plugins.len(),
                full: if let Some(full_class) = full_class {
                    full_class
                } else {
                    val
                },
                modifier: Modifier::Arbitrary {
                    prefix: "",
                    hint: "",
                    value: Cow::from(&remaining.1[1..remaining.1.len() - 1]),
                },
                variants,
                is_important,
                plugin,
            })]
        } else {
            // Find the right plugin for handling this selector
            for (order, (namespace, plugin)) in BUILTIN_PLUGINS
                .iter()
                .enumerate()
                .map(|p| (p.0 + config.custom_plugins.len(), p.1))
                .chain(config.custom_plugins.iter().enumerate())
            {
                // Find the modifier
                if let Some(modifier) = remaining
                    .1
                    .strip_prefix(&**namespace)
                    .and_then(|modifier| parse_modifier(modifier, is_negative))
                {
                    let context = ContextCanHandle {
                        config,
                        modifier: &modifier,
                    };

                    if plugin.can_handle(context) {
                        return vec![Ok(Selector {
                            // Selectors generated using custom plugins are placed first to be easily
                            // overridden, so we need to shift the order of builtin plugins to take that
                            // into account
                            order,
                            full: if let Some(full_class) = full_class {
                                full_class
                            } else {
                                val
                            },
                            modifier,
                            variants,
                            is_important,
                            plugin: *plugin,
                        })];
                    }
                }
            }

            vec![Err(ParseError::new(
                span,
                ParseErrorKind::UnknownPlugin(val),
            ))]
        }
    }
}

fn parse_modifier(mut modifier: &str, is_negative: bool) -> Option<Modifier> {
    if modifier.is_empty() {
        return Some(Modifier::Builtin {
            is_negative: false,
            value: "",
        });
    }

    if modifier.chars().next().unwrap() == MODIFIER_SEPARATOR {
        modifier = &modifier[1..];
    }

    if let Some((prefix, mut value)) = modifier.split_once(ARBITRARY_START) {
        if value.chars().last().unwrap() == ARBITRARY_END {
            value = &value[..value.len() - 1];
        } else {
            return None;
        }

        let (hint, value) = if let Some((maybe_hint, rest)) = value.split_once(HINT_SEPARATOR) {
            if VALID_PLUGIN_HINT.contains(&maybe_hint) {
                (maybe_hint, to_css_value(rest))
            } else {
                ("", to_css_value(value))
            }
        } else {
            ("", to_css_value(value))
        };

        Some(Modifier::Arbitrary {
            prefix,
            hint,
            value,
        })
    } else {
        Some(Modifier::Builtin {
            is_negative,
            value: modifier,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::Config, plugins::*, selector::Selector};

    use pretty_assertions::assert_eq;

    #[test]
    fn basic_single() {
        assert_eq!(
            parse("absolute", None, None, &Config::default())[0]
                .as_ref()
                .unwrap(),
            &Selector {
                full: "absolute",
                order: 4,
                plugin: &layout::position::PluginDefinition,
                variants: vec![],
                modifier: Modifier::Builtin {
                    is_negative: false,
                    value: "absolute",
                },
                is_important: false,
            }
        );
    }

    #[test]
    fn basic_multiple() {
        assert_eq!(
            parse("text-center", None, None, &Config::default())[0]
                .as_ref()
                .unwrap(),
            &Selector {
                full: "text-center",
                order: 165,
                plugin: &typography::text_align::PluginDefinition,
                variants: vec![],
                modifier: Modifier::Builtin {
                    is_negative: false,
                    value: "center",
                },
                is_important: false,
            }
        );
    }

    #[test]
    fn basic_opacity() {
        assert_eq!(
            parse("bg-red-500/25", None, None, &Config::default())[0]
                .as_ref()
                .unwrap(),
            &Selector {
                full: "bg-red-500/25",
                order: 140,
                plugin: &background::background_color::PluginDefinition,
                variants: vec![],
                modifier: Modifier::Builtin {
                    is_negative: false,
                    value: "red-500/25",
                },
                is_important: false,
            }
        );
    }

    #[test]
    fn basic_important() {
        assert_eq!(
            parse("!px-4", None, None, &Config::default())[0]
                .as_ref()
                .unwrap(),
            &Selector {
                full: "!px-4",
                order: 159,
                plugin: &spacing::padding::PluginDefinition,
                variants: vec![],
                modifier: Modifier::Builtin {
                    is_negative: false,
                    value: "4",
                },
                is_important: true,
            }
        );
    }

    #[test]
    fn basic_negative() {
        assert_eq!(
            parse("-px-4", None, None, &Config::default())[0]
                .as_ref()
                .unwrap(),
            &Selector {
                full: "-px-4",
                order: 159,
                plugin: &spacing::padding::PluginDefinition,
                variants: vec![],
                modifier: Modifier::Builtin {
                    is_negative: true,
                    value: "4",
                },
                is_important: false,
            }
        );
    }

    #[test]
    fn basic_important_and_negative() {
        assert_eq!(
            parse("!-px-4", None, None, &Config::default())[0]
                .as_ref()
                .unwrap(),
            &Selector {
                full: "!-px-4",
                order: 159,
                plugin: &spacing::padding::PluginDefinition,
                variants: vec![],
                modifier: Modifier::Builtin {
                    is_negative: true,
                    value: "4",
                },
                is_important: true,
            }
        );
    }

    #[test]
    fn basic_integer() {
        assert_eq!(
            parse("px-4", None, None, &Config::default())[0]
                .as_ref()
                .unwrap(),
            &Selector {
                full: "px-4",
                order: 159,
                plugin: &spacing::padding::PluginDefinition,
                variants: vec![],
                modifier: Modifier::Builtin {
                    is_negative: false,
                    value: "4",
                },
                is_important: false,
            }
        );
    }

    #[test]
    fn basic_float() {
        assert_eq!(
            parse("px-1.5", None, None, &Config::default())[0]
                .as_ref()
                .unwrap(),
            &Selector {
                full: "px-1.5",
                order: 159,
                plugin: &spacing::padding::PluginDefinition,
                variants: vec![],
                modifier: Modifier::Builtin {
                    is_negative: false,
                    value: "1.5",
                },
                is_important: false,
            }
        );
    }

    #[test]
    fn variants_single() {
        assert_eq!(
            parse("hover:text-center", None, None, &Config::default())[0]
                .as_ref()
                .unwrap(),
            &Selector {
                full: "hover:text-center",
                order: 165,
                plugin: &typography::text_align::PluginDefinition,
                variants: vec![Variant::Builtin(46, VariantType::PseudoClass("hover"))],
                modifier: Modifier::Builtin {
                    is_negative: false,
                    value: "center",
                },
                is_important: false,
            }
        );
    }

    #[test]
    fn variants_multiple() {
        assert_eq!(
            parse(
                "marker:xl:hover:text-center",
                None,
                None,
                &Config::default()
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                full: "marker:xl:hover:text-center",
                order: 165,
                plugin: &typography::text_align::PluginDefinition,
                variants: vec![
                    Variant::Builtin(
                        2,
                        VariantType::WrapClass(Cow::from("& *::marker, &::marker"))
                    ),
                    Variant::Builtin(
                        64,
                        VariantType::AtRule(Cow::from("@media (min-width: 1280px)"))
                    ),
                    Variant::Builtin(46, VariantType::PseudoClass("hover"))
                ],
                modifier: Modifier::Builtin {
                    is_negative: false,
                    value: "center",
                },
                is_important: false,
            }
        );
    }

    #[test]
    fn variants_negative() {
        assert_eq!(
            parse("marker:xl:hover:-mx-4", None, None, &Config::default())[0]
                .as_ref()
                .unwrap(),
            &Selector {
                full: "marker:xl:hover:-mx-4",
                order: 20,
                plugin: &spacing::margin::PluginXDefinition,
                variants: vec![
                    Variant::Builtin(
                        2,
                        VariantType::WrapClass(Cow::from("& *::marker, &::marker"))
                    ),
                    Variant::Builtin(
                        64,
                        VariantType::AtRule(Cow::from("@media (min-width: 1280px)"))
                    ),
                    Variant::Builtin(46, VariantType::PseudoClass("hover"))
                ],
                modifier: Modifier::Builtin {
                    is_negative: true,
                    value: "4",
                },
                is_important: false,
            }
        );
    }

    #[test]
    fn arbitrary_variant() {
        assert_eq!(
            parse("[&>*]:text-center", None, None, &Config::default())[0]
                .as_ref()
                .unwrap(),
            &Selector {
                full: "[&>*]:text-center",
                order: 165,
                plugin: &typography::text_align::PluginDefinition,
                variants: vec![Variant::Arbitrary(Cow::from("&>*"))],
                modifier: Modifier::Builtin {
                    is_negative: false,
                    value: "center",
                },
                is_important: false,
            }
        );
    }

    #[test]
    fn arbitrary_variant_at_rule() {
        assert_eq!(
            parse(
                "[@supports_not_(display:grid)]:grid",
                None,
                None,
                &Config::default()
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                full: "[@supports_not_(display:grid)]:grid",
                order: 27,
                plugin: &layout::display::PluginDefinition,
                variants: vec![Variant::Arbitrary(Cow::from(
                    "@supports not (display:grid)"
                ))],
                modifier: Modifier::Builtin {
                    is_negative: false,
                    value: "grid",
                },
                is_important: false,
            }
        );
    }

    #[test]
    fn arbitrary_variant_multiple() {
        assert_eq!(
            parse("xl:[&>*]:focus:text-center", None, None, &Config::default())[0]
                .as_ref()
                .unwrap(),
            &Selector {
                full: "xl:[&>*]:focus:text-center",
                order: 165,
                plugin: &typography::text_align::PluginDefinition,
                variants: vec![
                    Variant::Builtin(
                        64,
                        VariantType::AtRule(Cow::from("@media (min-width: 1280px)"))
                    ),
                    Variant::Arbitrary(Cow::from("&>*")),
                    Variant::Builtin(47, VariantType::PseudoClass("focus"))
                ],
                modifier: Modifier::Builtin {
                    is_negative: false,
                    value: "center",
                },
                is_important: false,
            }
        );
    }

    #[test]
    fn arbitrary_variant_negative() {
        assert_eq!(
            parse("xl:[&>*]:focus:-m-4", None, None, &Config::default())[0]
                .as_ref()
                .unwrap(),
            &Selector {
                full: "xl:[&>*]:focus:-m-4",
                order: 19,
                plugin: &spacing::margin::PluginDefinition,
                variants: vec![
                    Variant::Builtin(
                        64,
                        VariantType::AtRule(Cow::from("@media (min-width: 1280px)"))
                    ),
                    Variant::Arbitrary(Cow::from("&>*")),
                    Variant::Builtin(47, VariantType::PseudoClass("focus"))
                ],
                modifier: Modifier::Builtin {
                    is_negative: true,
                    value: "4",
                },
                is_important: false,
            }
        );
    }

    #[test]
    fn arbitrary_value() {
        assert_eq!(
            parse("mx-[12px]", None, None, &Config::default())[0]
                .as_ref()
                .unwrap(),
            &Selector {
                full: "mx-[12px]",
                order: 20,
                plugin: &spacing::margin::PluginDefinition,
                variants: vec![],
                modifier: Modifier::Arbitrary {
                    prefix: "",
                    hint: "",
                    value: Cow::from("12px"),
                },
                is_important: false,
            }
        );
    }

    #[test]
    fn complex_arbitrary_value() {
        assert_eq!(
            parse(
                "bg-[url('/hello_world.png')]",
                None,
                None,
                &Config::default()
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                full: "bg-[url('/hello_world.png')]",
                order: 142,
                plugin: &background::background_image::PluginDefinition,
                variants: vec![],
                modifier: Modifier::Arbitrary {
                    prefix: "",
                    hint: "",
                    value: Cow::from("url('/hello_world.png')"),
                },
                is_important: false,
            }
        );
    }

    #[test]
    fn arbitrary_value_hint() {
        assert_eq!(
            parse("bg-[color:#fff]", None, None, &Config::default())[0]
                .as_ref()
                .unwrap(),
            &Selector {
                full: "bg-[color:#fff]",
                order: 140,
                plugin: &background::background_color::PluginDefinition,
                variants: vec![],
                modifier: Modifier::Arbitrary {
                    prefix: "",
                    hint: "color",
                    value: Cow::from("#fff"),
                },
                is_important: false,
            }
        );
    }

    #[test]
    fn arbitrary_value_with_variants() {
        assert_eq!(
            parse("xl:marker:bg-[#fff]", None, None, &Config::default())[0]
                .as_ref()
                .unwrap(),
            &Selector {
                full: "xl:marker:bg-[#fff]",
                order: 140,
                plugin: &background::background_color::PluginDefinition,
                variants: vec![
                    Variant::Builtin(
                        64,
                        VariantType::AtRule(Cow::from("@media (min-width: 1280px)"))
                    ),
                    Variant::Builtin(
                        2,
                        VariantType::WrapClass(Cow::from("& *::marker, &::marker"))
                    ),
                ],
                modifier: Modifier::Arbitrary {
                    prefix: "",
                    hint: "",
                    value: Cow::from("#fff"),
                },
                is_important: false,
            }
        );
    }

    #[test]
    fn arbitrary_value_with_variants_and_hint() {
        assert_eq!(
            parse("xl:marker:bg-[color:#fff]", None, None, &Config::default())[0]
                .as_ref()
                .unwrap(),
            &Selector {
                full: "xl:marker:bg-[color:#fff]",
                order: 140,
                plugin: &background::background_color::PluginDefinition,
                variants: vec![
                    Variant::Builtin(
                        64,
                        VariantType::AtRule(Cow::from("@media (min-width: 1280px)"))
                    ),
                    Variant::Builtin(
                        2,
                        VariantType::WrapClass(Cow::from("& *::marker, &::marker"))
                    ),
                ],
                modifier: Modifier::Arbitrary {
                    prefix: "",
                    hint: "color",
                    value: Cow::from("#fff"),
                },
                is_important: false,
            }
        );
    }

    #[test]
    fn arbitrary_value_with_arbitrary_variant() {
        assert_eq!(
            parse("[&>*]:bg-[#fff]", None, None, &Config::default())[0]
                .as_ref()
                .unwrap(),
            &Selector {
                full: "[&>*]:bg-[#fff]",
                order: 140,
                plugin: &background::background_color::PluginDefinition,
                variants: vec![Variant::Arbitrary(Cow::from("&>*"))],
                modifier: Modifier::Arbitrary {
                    prefix: "",
                    hint: "",
                    value: Cow::from("#fff"),
                },
                is_important: false,
            }
        );
    }

    #[test]
    fn arbitrary_variant_escaped() {
        assert_eq!(
            parse(
                r"[\[type='input'\]_&>:*]:bg-red-300",
                None,
                None,
                &Config::default()
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                full: r"[\[type='input'\]_&>:*]:bg-red-300",
                order: 140,
                plugin: &background::background_color::PluginDefinition,
                variants: vec![Variant::Arbitrary(Cow::from("[type='input'] &>:*"))],
                modifier: Modifier::Builtin {
                    is_negative: false,
                    value: "red-300",
                },
                is_important: false,
            }
        );
    }

    #[test]
    fn arbitrary_value_with_arbitrary_variant_mixed() {
        assert_eq!(
            parse("xl:[&>*]:hover:bg-[#fff]", None, None, &Config::default())[0]
                .as_ref()
                .unwrap(),
            &Selector {
                full: "xl:[&>*]:hover:bg-[#fff]",
                order: 140,
                plugin: &background::background_color::PluginDefinition,
                variants: vec![
                    Variant::Builtin(
                        64,
                        VariantType::AtRule(Cow::from("@media (min-width: 1280px)"))
                    ),
                    Variant::Arbitrary(Cow::from("&>*")),
                    Variant::Builtin(46, VariantType::PseudoClass("hover"))
                ],
                modifier: Modifier::Arbitrary {
                    prefix: "",
                    hint: "",
                    value: Cow::from("#fff"),
                },
                is_important: false,
            }
        );
    }

    #[test]
    fn arbitrary_value_with_arbitrary_variant_and_hint() {
        assert_eq!(
            parse(
                "xl:[&>*]:hover:bg-[color:#fff]",
                None,
                None,
                &Config::default()
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                full: "xl:[&>*]:hover:bg-[color:#fff]",
                order: 140,
                plugin: &background::background_color::PluginDefinition,
                variants: vec![
                    Variant::Builtin(
                        64,
                        VariantType::AtRule(Cow::from("@media (min-width: 1280px)"))
                    ),
                    Variant::Arbitrary(Cow::from("&>*")),
                    Variant::Builtin(46, VariantType::PseudoClass("hover"))
                ],
                modifier: Modifier::Arbitrary {
                    prefix: "",
                    hint: "color",
                    value: Cow::from("#fff"),
                },
                is_important: false,
            }
        );
    }

    #[test]
    fn arbitrary_value_escaped() {
        assert_eq!(
            parse(
                r"bg-[url('/url_with_\]\)\'.png')]",
                None,
                None,
                &Config::default()
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                full: r"bg-[url('/url_with_\]\)\'.png')]",
                order: 142,
                plugin: &background::background_image::PluginDefinition,
                variants: vec![],
                modifier: Modifier::Arbitrary {
                    prefix: "",
                    hint: "",
                    value: Cow::from(r"url('/url_with_])\'.png')"),
                },
                is_important: false,
            }
        );
    }

    #[test]
    fn arbitrary_css_property() {
        assert_eq!(
            parse(
                "hover:[mask-type:luminance]",
                None,
                None,
                &Config::default()
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                full: "hover:[mask-type:luminance]",
                order: BUILTIN_PLUGINS.len(),
                plugin: &CssPropertyPlugin,
                variants: vec![Variant::Builtin(46, VariantType::PseudoClass("hover"))],
                modifier: Modifier::Arbitrary {
                    prefix: "",
                    hint: "",
                    value: Cow::from("mask-type:luminance"),
                },
                is_important: false,
            }
        );
    }

    #[test]
    fn variant_grouping() {
        assert_eq!(
            parse(
                "hover:(focus:bg-gray-500,text-[color:black,])",
                None,
                None,
                &Config::default(),
            ),
            vec![
                Ok(Selector {
                    full: "hover:(focus:bg-gray-500,text-[color:black,])",
                    order: 140,
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![
                        Variant::Builtin(47, VariantType::PseudoClass("focus")),
                        Variant::Builtin(46, VariantType::PseudoClass("hover"))
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "gray-500",
                    },
                    is_important: false,
                }),
                Ok(Selector {
                    full: "hover:(focus:bg-gray-500,text-[color:black,])",
                    order: 176,
                    plugin: &typography::text_color::PluginDefinition,
                    variants: vec![Variant::Builtin(46, VariantType::PseudoClass("hover"))],
                    modifier: Modifier::Arbitrary {
                        prefix: "",
                        hint: "color",
                        value: Cow::from("black,"),
                    },
                    is_important: false,
                })
            ],
        );
    }

    #[test]
    fn variant_grouping_single() {
        assert_eq!(
            parse("hover:(bg-gray-500)", None, None, &Config::default()),
            vec![Ok(Selector {
                full: "hover:(bg-gray-500)",
                order: 140,
                plugin: &background::background_color::PluginDefinition,
                variants: vec![Variant::Builtin(46, VariantType::PseudoClass("hover"))],
                modifier: Modifier::Builtin {
                    is_negative: false,
                    value: "gray-500",
                },
                is_important: false,
            })],
        );
    }

    #[test]
    fn variant_grouping_nested() {
        assert_eq!(
            parse(
                "focus:([&>*]:-m-4,xl:dark:(bg-red-100,rtl:text-[color:black]))",
                None,
                None,
                &Config::default(),
            ),
            vec![
                Ok(Selector {
                    full: "focus:([&>*]:-m-4,xl:dark:(bg-red-100,rtl:text-[color:black]))",
                    order: 19,
                    plugin: &spacing::margin::PluginDefinition,
                    variants: vec![
                        Variant::Arbitrary(Cow::from("&>*")),
                        Variant::Builtin(47, VariantType::PseudoClass("focus")),
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: true,
                        value: "4",
                    },
                    is_important: false,
                }),
                Ok(Selector {
                    full: "focus:([&>*]:-m-4,xl:dark:(bg-red-100,rtl:text-[color:black]))",
                    order: 140,
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![
                        Variant::Builtin(
                            64,
                            VariantType::AtRule(Cow::from("@media (min-width: 1280px)"))
                        ),
                        Variant::Builtin(
                            66,
                            VariantType::AtRule(Cow::from("@media (prefers-color-scheme: dark)"))
                        ),
                        Variant::Builtin(47, VariantType::PseudoClass("focus")),
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "red-100",
                    },
                    is_important: false,
                }),
                Ok(Selector {
                    full: "focus:([&>*]:-m-4,xl:dark:(bg-red-100,rtl:text-[color:black]))",
                    order: 176,
                    plugin: &typography::text_color::PluginDefinition,
                    variants: vec![
                        Variant::Builtin(53, VariantType::WrapClass(Cow::from("[dir=\"rtl\"] &"))),
                        Variant::Builtin(
                            64,
                            VariantType::AtRule(Cow::from("@media (min-width: 1280px)"))
                        ),
                        Variant::Builtin(
                            66,
                            VariantType::AtRule(Cow::from("@media (prefers-color-scheme: dark)"))
                        ),
                        Variant::Builtin(47, VariantType::PseudoClass("focus")),
                    ],
                    modifier: Modifier::Arbitrary {
                        prefix: "",
                        hint: "color",
                        value: Cow::from("black"),
                    },
                    is_important: false,
                }),
            ],
        );
    }

    #[test]
    fn variant_grouping_nested_escaped() {
        assert_eq!(
            parse(
                r"focus:([&>*]:-m-4,xl:dark:([\[type='text'\].light_&,.foo]:bg-red-100,text-[color:black,]))",
                None,
                None,
                &Config::default(),
            ),
            vec![
                Ok(Selector {
                    full: r"focus:([&>*]:-m-4,xl:dark:([\[type='text'\].light_&,.foo]:bg-red-100,text-[color:black,]))",
                    order: 19,
                    plugin: &spacing::margin::PluginDefinition,
                    variants: vec![
                        Variant::Arbitrary(Cow::from("&>*")),
                        Variant::Builtin(47, VariantType::PseudoClass("focus")),
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: true,
                        value: "4",
                    },
                    is_important: false,
                }),
                Ok(Selector {
                    full: r"focus:([&>*]:-m-4,xl:dark:([\[type='text'\].light_&,.foo]:bg-red-100,text-[color:black,]))",
                    order: 140,
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![
                        Variant::Arbitrary(Cow::from(r"[type='text'].light &,.foo")),
                        Variant::Builtin(
                            64,
                            VariantType::AtRule(Cow::from("@media (min-width: 1280px)"))
                        ),
                        Variant::Builtin(
                            66,
                            VariantType::AtRule(Cow::from("@media (prefers-color-scheme: dark)"))
                        ),
                        Variant::Builtin(47, VariantType::PseudoClass("focus")),
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "red-100",
                    },
                    is_important: false,
                }),
                Ok(Selector {
                    full: r"focus:([&>*]:-m-4,xl:dark:([\[type='text'\].light_&,.foo]:bg-red-100,text-[color:black,]))",
                    order: 176,
                    plugin: &typography::text_color::PluginDefinition,
                    variants: vec![
                        Variant::Builtin(
                            64,
                            VariantType::AtRule(Cow::from("@media (min-width: 1280px)"))
                        ),
                        Variant::Builtin(
                            66,
                            VariantType::AtRule(Cow::from("@media (prefers-color-scheme: dark)"))
                        ),
                        Variant::Builtin(47, VariantType::PseudoClass("focus")),
                    ],
                    modifier: Modifier::Arbitrary {
                        prefix: "",
                        hint: "color",
                        value: Cow::from("black,"),
                    },
                    is_important: false,
                }),
            ],
        );
    }

    #[test]
    fn variant_grouping_complex_nested() {
        assert_eq!(
            parse(
                r"xl:(focus:(outline,outline-red-200),dark:(bg-black,text-white))",
                None,
                None,
                &Config::default(),
            ),
            vec![
                Ok(Selector {
                    full: r"xl:(focus:(outline,outline-red-200),dark:(bg-black,text-white))",
                    order: 191,
                    plugin: &border::outline_style::PluginDefinition,
                    variants: vec![
                        Variant::Builtin(47, VariantType::PseudoClass("focus")),
                        Variant::Builtin(
                            64,
                            VariantType::AtRule(Cow::from("@media (min-width: 1280px)"))
                        )
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "",
                    },
                    is_important: false,
                }),
                Ok(Selector {
                    full: r"xl:(focus:(outline,outline-red-200),dark:(bg-black,text-white))",
                    order: 194,
                    plugin: &border::outline_color::PluginDefinition,
                    variants: vec![
                        Variant::Builtin(47, VariantType::PseudoClass("focus")),
                        Variant::Builtin(
                            64,
                            VariantType::AtRule(Cow::from("@media (min-width: 1280px)"))
                        )
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "red-200",
                    },
                    is_important: false,
                }),
                Ok(Selector {
                    full: r"xl:(focus:(outline,outline-red-200),dark:(bg-black,text-white))",
                    order: 140,
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![
                        Variant::Builtin(
                            66,
                            VariantType::AtRule(Cow::from("@media (prefers-color-scheme: dark)"))
                        ),
                        Variant::Builtin(
                            64,
                            VariantType::AtRule(Cow::from("@media (min-width: 1280px)"))
                        ),
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "black",
                    },
                    is_important: false,
                }),
                Ok(Selector {
                    full: r"xl:(focus:(outline,outline-red-200),dark:(bg-black,text-white))",
                    order: 176,
                    plugin: &typography::text_color::PluginDefinition,
                    variants: vec![
                        Variant::Builtin(
                            66,
                            VariantType::AtRule(Cow::from("@media (prefers-color-scheme: dark)"))
                        ),
                        Variant::Builtin(
                            64,
                            VariantType::AtRule(Cow::from("@media (min-width: 1280px)"))
                        ),
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "white",
                    },
                    is_important: false,
                }),
            ],
        );
    }
}
