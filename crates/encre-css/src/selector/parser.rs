use super::{Modifier, Selector, Variant, VariantType};
use crate::{
    config::{Config, BUILTIN_PLUGINS, BUILTIN_VARIANTS},
    generator::ContextCanHandle,
    plugins::{css_property::CssPropertyPlugin, Plugin},
    utils::split_ignore_arbitrary,
};

use std::borrow::Cow;

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
/// - `\[` are replaced with `[`
/// - `\]` are replaced with `]`
/// - `\(` are replaced with `(`
/// - `\)` are replaced with `)`
pub fn unescape(mut val: Cow<str>) -> Cow<str> {
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
pub fn underscores_to_spaces(mut val: Cow<str>) -> Cow<str> {
    // Don't replace `_` if it is a URL
    if val.contains("url") {
        // For the `CursorPlugin`, `ContentPlugin` and `ImagePlugin` plugins, we need to keep underscores in URLs
        val = Cow::from(val.replace('_', WILL_BE_REPLACED_BY_UNDERSCORE));
    }

    // Replace `_` with ` ` (spaces) (if not prefixed by a backslash)
    if val.contains("\\_") {
        val = Cow::from(val.replace("\\_", WILL_BE_REPLACED_BY_UNDERSCORE));
    }

    if val.contains('_') {
        val = Cow::from(val.replace('_', " "));
    }

    if val.contains(WILL_BE_REPLACED_BY_UNDERSCORE) {
        val = Cow::from(val.replace(WILL_BE_REPLACED_BY_UNDERSCORE, "_"));
    }

    val
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

pub(crate) fn parse<'a>(val: &'a str, config: &Config) -> Option<Vec<Selector<'a>>> {
    // The smallest selector is `m-1`
    if val.len() < 3 {
        return None;
    }

    parse_recursive(val, None, config)
}

#[allow(clippy::too_many_lines)]
fn parse_recursive<'a>(
    val: &'a str,
    full_class: Option<&'a str>,
    config: &Config,
) -> Option<Vec<Selector<'a>>> {
    // Parse variants
    let mut variants = vec![];
    let mut remaining = "";

    {
        let custom_variants = config.get_custom_variants();
        let mut iter = split_ignore_arbitrary(val, VARIANT_SEPARATOR, true).peekable();

        while let Some(mut val) = iter.next() {
            if iter.peek().is_none() {
                remaining = val;
                continue;
            }

            let (is_arbitrary, variant) = {
                if val.starts_with(ARBITRARY_START) && val.ends_with(ARBITRARY_END) {
                    unwrap_string(&mut val);
                    (true, val)
                } else {
                    (false, val)
                }
            };

            if is_arbitrary {
                variants.push(Variant::Arbitrary(underscores_to_spaces(unescape(
                    Cow::from(variant),
                ))));
            } else if let Some((order, variant)) = BUILTIN_VARIANTS
                .iter()
                .enumerate()
                .find(|(_, v)| v.0 == variant)
                .or_else(|| {
                    custom_variants.iter().enumerate().find_map(|(order, v)| {
                        if v.0 == variant {
                            Some((order + BUILTIN_VARIANTS.len(), v))
                        } else {
                            None
                        }
                    })
                })
            {
                variants.push(Variant::Builtin(order, variant.1.clone()));
            } else {
                // Maybe a parent or peer variant
                if let Some(group_variant) = variant.strip_prefix("group-") {
                    if let Some((order, (_, VariantType::PseudoClass(class)))) = BUILTIN_VARIANTS
                        .iter()
                        .enumerate()
                        .find(|(_, v)| v.0 == group_variant)
                    {
                        variants.push(Variant::Builtin(order + 1000, VariantType::Group(class)));
                    }
                } else if let Some(peer_not_variant) = variant.strip_prefix("peer-not-") {
                    if let Some((order, (_, VariantType::PseudoClass(class)))) = BUILTIN_VARIANTS
                        .iter()
                        .enumerate()
                        .find(|(_, v)| v.0 == peer_not_variant)
                    {
                        variants.push(Variant::Builtin(order + 2000, VariantType::PeerNot(class)));
                    }
                } else if let Some(peer_variant) = variant.strip_prefix("peer-") {
                    if let Some((order, (_, VariantType::PseudoClass(class)))) = BUILTIN_VARIANTS
                        .iter()
                        .enumerate()
                        .find(|(_, v)| v.0 == peer_variant)
                    {
                        variants.push(Variant::Builtin(order + 3000, VariantType::Peer(class)));
                    }
                }
            }
        }
    }

    if remaining.is_empty() {
        // Parsing error, abort.
        return None;
    }

    // Parse the namespace and modifier
    if remaining.starts_with(GROUP_START) && remaining.ends_with(GROUP_END) {
        // Variant group (selectors are separated by `,`), we need to parse child selectors
        unwrap_string(&mut remaining);

        if remaining.is_empty() {
            return None;
        }

        let mut selectors = vec![];

        split_ignore_arbitrary(remaining, GROUP_SELECTOR_SEPARATOR, true).for_each(|remaining| {
            if let Some(mut new_selectors) = parse_recursive(
                remaining,
                Some(if let Some(full_class) = full_class {
                    full_class
                } else {
                    val
                }),
                config,
            ) {
                // Merge the common variants with each child selector variant list
                new_selectors
                    .iter_mut()
                    .for_each(|selector| selector.variants.extend(variants.iter().cloned()));

                selectors.extend(new_selectors);
            }
        });

        Some(selectors)
    } else {
        // Child selector
        let is_important = if let Some(new_remaining) = remaining.strip_prefix(IMPORTANT_FLAG) {
            remaining = new_remaining;
            true
        } else {
            false
        };

        let is_negative = if let Some(new_remaining) = remaining.strip_prefix(NEGATIVE_FLAG) {
            remaining = new_remaining;
            true
        } else {
            false
        };

        if remaining.starts_with(ARBITRARY_START) && remaining.ends_with(ARBITRARY_END) {
            // Arbitrary CSS property (without namespace)
            let plugin = &CssPropertyPlugin;

            Some(vec![Selector {
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
                    value: Cow::from(&remaining[1..remaining.len() - 1]),
                },
                variants,
                is_important,
                plugin,
            }])
        } else {
            // Find the right plugin for handling this selector
            let find = move |(order, plugin): (usize, &&'static (dyn Plugin + Send + Sync))| {
                // Find the modifier
                if let Some(modifier_part) = remaining.strip_prefix(&plugin.namespace()) {
                    let modifier_part = modifier_part
                        .strip_prefix(MODIFIER_SEPARATOR)
                        .unwrap_or(modifier_part);

                    let modifier = if let Some((mut prefix, mut value)) =
                        modifier_part.split_once(ARBITRARY_START)
                    {
                        prefix = prefix.strip_prefix(MODIFIER_SEPARATOR).unwrap_or(prefix);
                        value = value.strip_suffix(ARBITRARY_END)?;

                        let (hint, value) =
                            if let Some((maybe_hint, rest)) = value.split_once(HINT_SEPARATOR) {
                                if VALID_PLUGIN_HINT.contains(&maybe_hint) {
                                    (maybe_hint, to_css_value(rest))
                                } else {
                                    ("", to_css_value(value))
                                }
                            } else {
                                ("", to_css_value(value))
                            };

                        Modifier::Arbitrary {
                            prefix,
                            hint,
                            value,
                        }
                    } else {
                        Modifier::Builtin {
                            is_negative,
                            value: modifier_part,
                        }
                    };

                    let context = ContextCanHandle {
                        config,
                        modifier: &modifier,
                    };

                    if plugin.can_handle(context) {
                        Some(vec![Selector {
                            order,
                            full: if let Some(full_class) = full_class {
                                full_class
                            } else {
                                val
                            },
                            modifier,
                            variants: variants.clone(),
                            is_important,
                            plugin: *plugin,
                        }])
                    } else {
                        None
                    }
                } else {
                    None
                }
            };
            BUILTIN_PLUGINS
                .iter()
                .enumerate()
                .find_map(&find)
                .map(|mut selectors| {
                    // Selectors generated using custom plugins are placed first to be easily
                    // overridden, so we need to shift the order of builtin plugins to take that
                    // into account
                    selectors
                        .iter_mut()
                        .for_each(|p| p.order += config.custom_plugins.len());
                    selectors
                })
                .or_else(|| config.custom_plugins.iter().enumerate().find_map(find))
        }
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
            parse("absolute", &Config::default()).unwrap()[0],
            Selector {
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
            parse("text-center", &Config::default()).unwrap()[0],
            Selector {
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
            parse("bg-red-500/25", &Config::default()).unwrap()[0],
            Selector {
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
            parse("!px-4", &Config::default()).unwrap()[0],
            Selector {
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
            parse("-px-4", &Config::default()).unwrap()[0],
            Selector {
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
            parse("!-px-4", &Config::default()).unwrap()[0],
            Selector {
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
            parse("px-4", &Config::default()).unwrap()[0],
            Selector {
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
            parse("px-1.5", &Config::default()).unwrap()[0],
            Selector {
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
            parse("hover:text-center", &Config::default()).unwrap()[0],
            Selector {
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
            parse("marker:xl:hover:text-center", &Config::default()).unwrap()[0],
            Selector {
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
            parse("marker:xl:hover:-mx-4", &Config::default()).unwrap()[0],
            Selector {
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
            parse("[&>*]:text-center", &Config::default()).unwrap()[0],
            Selector {
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
            parse("[@supports_not_(display:grid)]:grid", &Config::default()).unwrap()[0],
            Selector {
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
            parse("xl:[&>*]:focus:text-center", &Config::default()).unwrap()[0],
            Selector {
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
            parse("xl:[&>*]:focus:-m-4", &Config::default()).unwrap()[0],
            Selector {
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
            parse("mx-[12px]", &Config::default()).unwrap()[0],
            Selector {
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
            parse("bg-[url('/hello_world.png')]", &Config::default()).unwrap()[0],
            Selector {
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
            parse("bg-[color:#fff]", &Config::default()).unwrap()[0],
            Selector {
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
            parse("xl:marker:bg-[#fff]", &Config::default()).unwrap()[0],
            Selector {
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
            parse("xl:marker:bg-[color:#fff]", &Config::default()).unwrap()[0],
            Selector {
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
            parse("[&>*]:bg-[#fff]", &Config::default()).unwrap()[0],
            Selector {
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
            parse(r"[\[type='input'\]_&>:*]:bg-red-300", &Config::default()).unwrap()[0],
            Selector {
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
            parse("xl:[&>*]:hover:bg-[#fff]", &Config::default()).unwrap()[0],
            Selector {
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
            parse("xl:[&>*]:hover:bg-[color:#fff]", &Config::default()).unwrap()[0],
            Selector {
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
            parse(r"bg-[url('/url_with_\]\)\'.png')]", &Config::default()).unwrap()[0],
            Selector {
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
            parse("hover:[mask-type:luminance]", &Config::default()).unwrap()[0],
            Selector {
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
                &Config::default(),
            )
            .unwrap(),
            vec![
                Selector {
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
                },
                Selector {
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
                }
            ],
        );
    }

    #[test]
    fn variant_grouping_single() {
        assert_eq!(
            parse("hover:(bg-gray-500)", &Config::default()).unwrap(),
            vec![Selector {
                full: "hover:(bg-gray-500)",
                order: 140,
                plugin: &background::background_color::PluginDefinition,
                variants: vec![Variant::Builtin(46, VariantType::PseudoClass("hover"))],
                modifier: Modifier::Builtin {
                    is_negative: false,
                    value: "gray-500",
                },
                is_important: false,
            }],
        );
    }

    #[test]
    fn variant_grouping_nested() {
        assert_eq!(
            parse(
                "focus:([&>*]:-m-4,xl:dark:(bg-red-100,rtl:text-[color:black]))",
                &Config::default(),
            )
            .unwrap(),
            vec![
                Selector {
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
                },
                Selector {
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
                },
                Selector {
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
                },
            ],
        );
    }

    #[test]
    fn variant_grouping_nested_escaped() {
        assert_eq!(
            parse(
                r"focus:([&>*]:-m-4,xl:dark:([\[type='text'\].light_&,.foo]:bg-red-100,text-[color:black,]))",
                &Config::default(),
            ).unwrap(),
            vec![
                Selector {
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
                },
                Selector {
                    full: r"focus:([&>*]:-m-4,xl:dark:([\[type='text'\].light_&,.foo]:bg-red-100,text-[color:black,]))",
                    order: 140,
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![
                        Variant::Arbitrary(Cow::from(r"[type='text'].light &,.foo")),
                        Variant::Builtin(64, VariantType::AtRule(Cow::from("@media (min-width: 1280px)"))),
                        Variant::Builtin(66, VariantType::AtRule(Cow::from("@media (prefers-color-scheme: dark)"))),
                        Variant::Builtin(47, VariantType::PseudoClass("focus")),
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "red-100",
                    },
                    is_important: false,
                },
                Selector {
                    full: r"focus:([&>*]:-m-4,xl:dark:([\[type='text'\].light_&,.foo]:bg-red-100,text-[color:black,]))",
                    order: 176,
                    plugin: &typography::text_color::PluginDefinition,
                    variants: vec![
                        Variant::Builtin(64, VariantType::AtRule(Cow::from("@media (min-width: 1280px)"))),
                        Variant::Builtin(66, VariantType::AtRule(Cow::from("@media (prefers-color-scheme: dark)"))),
                        Variant::Builtin(47, VariantType::PseudoClass("focus")),
                    ],
                    modifier: Modifier::Arbitrary {
                        prefix: "",
                        hint: "color",
                        value: Cow::from("black,"),
                    },
                    is_important: false,
                },
            ],
        );
    }

    #[test]
    fn variant_grouping_complex_nested() {
        assert_eq!(
            parse(
                r"xl:(focus:(outline,outline-red-200),dark:(bg-black,text-white))",
                &Config::default(),
            )
            .unwrap(),
            vec![
                Selector {
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
                },
                Selector {
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
                },
                Selector {
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
                },
                Selector {
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
                },
            ],
        );
    }
}
