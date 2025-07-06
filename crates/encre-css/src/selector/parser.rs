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
                if p.is_empty() {
                    return String::new();
                }

                let result = if let Some(sub) = p.strip_prefix('\'') {
                    sub.find("')").map(|index| p.split_at(index + 2))
                } else if let Some(sub) = p.strip_prefix('"') {
                    sub.find("\")").map(|index| p.split_at(index + 2))
                } else {
                    p.find(')').map(|index| p.split_at(index + 1))
                };

                if let Some((before, after)) = result {
                    format!("url({}{}", before, after.replace('_', " "))
                } else {
                    format!("url({}", p.replace('_', " "))
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
    config_derived_variants: &[(Cow<'static, str>, VariantType)],
) -> Vec<Result<Selector<'a>, ParseError<'a>>> {
    // The shortest selector is `m1`
    if val.len() < 2 {
        return vec![Err(ParseError::new(
            span.unwrap_or(0..val.len()),
            ParseErrorKind::TooShort(val),
        ))];
    }

    parse_recursive(val, span, full_class, config, config_derived_variants)
}

fn push_variant<'a>(
    is_arbitrary: bool,
    variant: &'a str,
    variant_list: &mut Vec<Variant<'a>>,
    val: &'a str,
    span: &Range<usize>,
    config: &Config,
    config_derived_variants: &[(Cow<'static, str>, VariantType)],
) -> Result<(), ParseError<'a>> {
    if is_arbitrary {
        variant_list.push(Variant::Arbitrary(underscores_to_spaces(unescape(
            Cow::from(variant),
        ))));
    } else if let Some((order, variant)) = BUILTIN_VARIANTS.get(variant) {
        variant_list.push(Variant::Builtin(*order, variant.clone()));
    } else if let Some((order, variant)) = config
        .custom_variants
        .iter()
        .chain(config_derived_variants)
        .enumerate()
        .find(|(_, v)| v.0 == variant)
    {
        variant_list.push(Variant::Builtin(
            BUILTIN_VARIANTS.len() + order,
            variant.1.clone(),
        ));
    } else {
        // Maybe a parent or peer variant
        if let Some(group_variant) = variant.strip_prefix("group-") {
            if let Some((order, VariantType::PseudoClass(class))) =
                BUILTIN_VARIANTS.get(group_variant)
            {
                variant_list.push(Variant::Builtin(order + 1000, VariantType::Group(class)));
            }
        } else if let Some(peer_not_variant) = variant.strip_prefix("peer-not-") {
            if let Some((order, VariantType::PseudoClass(class))) =
                BUILTIN_VARIANTS.get(peer_not_variant)
            {
                variant_list.push(Variant::Builtin(order + 2000, VariantType::PeerNot(class)));
            }
        } else if let Some(peer_variant) = variant.strip_prefix("peer-") {
            if let Some((order, VariantType::PseudoClass(class))) =
                BUILTIN_VARIANTS.get(peer_variant)
            {
                variant_list.push(Variant::Builtin(order + 3000, VariantType::Peer(class)));
            }
        } else {
            return Err(ParseError::new(
                span.clone(),
                ParseErrorKind::UnknownVariant(variant, val),
            ));
        }
    }

    Ok(())
}

#[allow(clippy::too_many_lines)]
fn parse_recursive<'a>(
    val: &'a str,
    span: Option<Range<usize>>,
    full_class: Option<&'a str>,
    config: &Config,
    config_derived_variants: &[(Cow<'static, str>, VariantType)],
) -> Vec<Result<Selector<'a>, ParseError<'a>>> {
    let span = span.unwrap_or(0..val.len());

    // Parse variants
    let (variants, mut remaining) = {
        let mut arbitraries = 0usize;
        let mut groups = 0usize;
        let mut last_index = 0;
        let mut variants = vec![vec![]];

        for ch in val.char_indices() {
            if last_index > val.len() {
                continue;
            }

            match ch.1 {
                '[' => {
                    arbitraries = arbitraries.saturating_add(1);
                    continue;
                }
                ']' => {
                    arbitraries = arbitraries.saturating_sub(1);
                    continue;
                }
                '(' if arbitraries == 0 => {
                    groups = groups.saturating_add(1);
                    continue;
                }
                ')' if arbitraries == 0 => {
                    groups = groups.saturating_sub(1);
                    continue;
                }
                ':' if arbitraries == 0 && groups == 0 => (),
                _ => {
                    continue;
                }
            }

            let mut part = &val[last_index..ch.0];
            last_index = ch.0 + 1;

            if part.starts_with(GROUP_START) && part.ends_with(GROUP_END) {
                unwrap_string(&mut part);

                let mut errors: Vec<Result<Selector<'a>, ParseError<'a>>> = vec![];
                let mut new_variants = vec![];

                split_ignore_arbitrary(part, ',', true).for_each(|(_, mut sub_variant)| {
                    let (is_arbitrary, variant) = {
                        if sub_variant.starts_with(ARBITRARY_START)
                            && sub_variant.ends_with(ARBITRARY_END)
                        {
                            unwrap_string(&mut sub_variant);
                            (true, sub_variant)
                        } else {
                            (false, sub_variant)
                        }
                    };

                    for variant_list in &variants {
                        // TODO: Maybe use `Cow`s to just reference all the other variants
                        let mut new_variant_list = variant_list.clone();

                        if let Err(e) = push_variant(
                            is_arbitrary,
                            variant,
                            &mut new_variant_list,
                            val,
                            &span,
                            config,
                            config_derived_variants,
                        ) {
                            errors.push(Err(e));
                        }

                        new_variants.push(new_variant_list);
                    }
                });

                variants = new_variants;

                if !errors.is_empty() {
                    return errors;
                }
            } else {
                let (is_arbitrary, variant) = {
                    if part.starts_with(ARBITRARY_START) && part.ends_with(ARBITRARY_END) {
                        unwrap_string(&mut part);
                        (true, part)
                    } else {
                        (false, part)
                    }
                };

                let mut errors = vec![];
                for variant_list in &mut variants {
                    if let Err(e) = push_variant(
                        is_arbitrary,
                        variant,
                        variant_list,
                        val,
                        &span,
                        config,
                        config_derived_variants,
                    ) {
                        errors.push(Err(e));
                    }
                }

                if !errors.is_empty() {
                    return errors;
                }
            }
        }

        (variants, (last_index, &val[last_index..]))
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

        split_ignore_arbitrary(remaining.1, ',', true)
            .flat_map(|(i, sub_selector)| {
                #[allow(clippy::range_plus_one)]
                let selectors = parse_recursive(
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
                    config_derived_variants,
                );

                // Merge the common variants with each child selector variant list
                selectors
                    .into_iter()
                    .fold(vec![], |mut new_selectors, selector| {
                        match selector {
                            Ok(mut selector) => {
                                let variant_len = selector.variants.len();

                                for variant_list in &variants {
                                    selector.variants.extend(variant_list.iter().cloned());
                                    new_selectors.push(Ok(selector.clone()));
                                    selector.variants.truncate(variant_len);
                                }
                            }
                            Err(e) => new_selectors.push(Err(e)),
                        }

                        new_selectors
                    })
            })
            .collect()
    } else {
        // Child selector
        let is_important = if let Some(new_remaining) = remaining.1.strip_prefix('!') {
            remaining = (remaining.0, new_remaining);
            true
        } else {
            false
        };

        let is_negative = if let Some(new_remaining) = remaining.1.strip_prefix('-') {
            remaining = (remaining.0, new_remaining);
            true
        } else {
            false
        };

        if remaining.1.starts_with(ARBITRARY_START) && remaining.1.ends_with(ARBITRARY_END) {
            // Arbitrary CSS property (without namespace)
            let plugin = &CssPropertyPlugin;

            variants
                .into_iter()
                .map(|variants| {
                    Ok(Selector {
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
                            value: to_css_value(&remaining.1[1..remaining.1.len() - 1]),
                        },
                        variants,
                        is_important,
                        plugin,
                    })
                })
                .collect()
        } else {
            // Find the right plugin for handling this selector
            for (order, (namespace, plugin)) in BUILTIN_PLUGINS
                .iter()
                .enumerate()
                // Selectors generated using custom plugins are placed first to be easily
                // overridden, so we need to shift the order of builtin plugins to take that
                // into account
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

                    if let Modifier::Arbitrary { prefix, .. } = modifier {
                        if !prefix.is_empty() {
                            // If the modifier is arbitrary, the namespace must be strictly parsed
                            // to avoid accepting too much selectors, e.g `flex-test-[]` being
                            // parsed as a `flex` selector
                            continue;
                        }
                    }

                    if plugin.can_handle(context) {
                        return variants
                            .into_iter()
                            .map(|variants| {
                                Ok(Selector {
                                    order,
                                    full: if let Some(full_class) = full_class {
                                        full_class
                                    } else {
                                        val
                                    },
                                    modifier: modifier.clone(),
                                    variants,
                                    is_important,
                                    plugin: *plugin,
                                })
                            })
                            .collect();
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

    if modifier.starts_with('-') {
        modifier = &modifier[1..];
    }

    if let Some((prefix, mut value)) = modifier.split_once(ARBITRARY_START) {
        if value.chars().last().map_or(false, |v| v == ARBITRARY_END) {
            value = &value[..value.len() - 1];
        } else {
            return None;
        }

        let (hint, value) = if let Some((maybe_hint, rest)) = value.split_once(':') {
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
    // NOTE: In these tests, the order value of the selectors and variants are arbitrary because
    // the implementation of PartialEq does not take them into account, so instead they are written
    // as Default::default()
    use super::*;

    #[allow(clippy::wildcard_imports)]
    use crate::{config::Config, plugins::*, selector::Selector};

    use pretty_assertions::assert_eq;

    #[test]
    fn underscores_to_spaces_test() {
        assert_eq!(
            underscores_to_spaces(Cow::Borrowed(
                "url('/hello_world.png'),url(\"some__text.txt\")"
            )),
            "url('/hello_world.png'),url(\"some__text.txt\")",
        );
        assert_eq!(
            underscores_to_spaces(Cow::Borrowed(r#"url("[l8""#)),
            r#"url("[l8""#
        );
    }

    #[test]
    fn basic_single() {
        let config = Config::default();
        assert_eq!(
            parse(
                "absolute",
                None,
                None,
                &config,
                &config.get_derived_variants()
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                full: "absolute",
                order: Default::default(), // order is not checked in tests
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
        let config = Config::default();
        assert_eq!(
            parse(
                "text-center",
                None,
                None,
                &config,
                &config.get_derived_variants()
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                full: "text-center",
                order: Default::default(),
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
        let config = Config::default();
        assert_eq!(
            parse(
                "bg-red-500/25",
                None,
                None,
                &config,
                &config.get_derived_variants()
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                full: "bg-red-500/25",
                order: Default::default(),
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
        let config = Config::default();
        assert_eq!(
            parse("!px-4", None, None, &config, &config.get_derived_variants())[0]
                .as_ref()
                .unwrap(),
            &Selector {
                full: "!px-4",
                order: Default::default(),
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
        let config = Config::default();
        assert_eq!(
            parse("-px-4", None, None, &config, &config.get_derived_variants())[0]
                .as_ref()
                .unwrap(),
            &Selector {
                full: "-px-4",
                order: Default::default(),
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
        let config = Config::default();
        assert_eq!(
            parse(
                "!-px-4",
                None,
                None,
                &config,
                &config.get_derived_variants()
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                full: "!-px-4",
                order: Default::default(),
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
        let config = Config::default();
        assert_eq!(
            parse("px-4", None, None, &config, &config.get_derived_variants())[0]
                .as_ref()
                .unwrap(),
            &Selector {
                full: "px-4",
                order: Default::default(),
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
        let config = Config::default();
        assert_eq!(
            parse(
                "px-1.5",
                None,
                None,
                &config,
                &config.get_derived_variants()
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                full: "px-1.5",
                order: Default::default(),
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
        let config = Config::default();
        assert_eq!(
            parse(
                "hover:text-center",
                None,
                None,
                &config,
                &config.get_derived_variants()
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                full: "hover:text-center",
                order: Default::default(),
                plugin: &typography::text_align::PluginDefinition,
                variants: vec![Variant::Builtin(Default::default(), VariantType::PseudoClass("hover"))],
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
        let config = Config::default();
        assert_eq!(
            parse(
                "marker:xl:hover:text-center",
                None,
                None,
                &config,
                &config.get_derived_variants(),
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                full: "marker:xl:hover:text-center",
                order: Default::default(),
                plugin: &typography::text_align::PluginDefinition,
                variants: vec![
                    Variant::Builtin(
                        Default::default(),
                        VariantType::WrapClass(Cow::from("& *::marker, &::marker"))
                    ),
                    Variant::Builtin(
                        Default::default(),
                        VariantType::AtRule(Cow::from("@media (width >= 80rem)"))
                    ),
                    Variant::Builtin(Default::default(), VariantType::PseudoClass("hover"))
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
        let config = Config::default();
        assert_eq!(
            parse(
                "marker:xl:hover:-mx-4",
                None,
                None,
                &config,
                &config.get_derived_variants()
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                full: "marker:xl:hover:-mx-4",
                order: Default::default(),
                plugin: &spacing::margin::PluginXDefinition,
                variants: vec![
                    Variant::Builtin(
                        Default::default(),
                        VariantType::WrapClass(Cow::from("& *::marker, &::marker"))
                    ),
                    Variant::Builtin(
                        Default::default(),
                        VariantType::AtRule(Cow::from("@media (width >= 80rem)"))
                    ),
                    Variant::Builtin(Default::default(), VariantType::PseudoClass("hover"))
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
        let config = Config::default();
        assert_eq!(
            parse(
                "[&>*]:text-center",
                None,
                None,
                &config,
                &config.get_derived_variants()
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                full: "[&>*]:text-center",
                order: Default::default(),
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
        let config = Config::default();
        assert_eq!(
            parse(
                "[@supports_not_(display:grid)]:grid",
                None,
                None,
                &config,
                &config.get_derived_variants(),
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                full: "[@supports_not_(display:grid)]:grid",
                order: Default::default(),
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
        let config = Config::default();
        assert_eq!(
            parse(
                "xl:[&>*]:focus:text-center",
                None,
                None,
                &config,
                &config.get_derived_variants()
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                full: "xl:[&>*]:focus:text-center",
                order: Default::default(),
                plugin: &typography::text_align::PluginDefinition,
                variants: vec![
                    Variant::Builtin(
                        Default::default(),
                        VariantType::AtRule(Cow::from("@media (width >= 80rem)"))
                    ),
                    Variant::Arbitrary(Cow::from("&>*")),
                    Variant::Builtin(Default::default(), VariantType::PseudoClass("focus"))
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
        let config = Config::default();
        assert_eq!(
            parse(
                "xl:[&>*]:focus:-m-4",
                None,
                None,
                &config,
                &config.get_derived_variants()
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                full: "xl:[&>*]:focus:-m-4",
                order: Default::default(),
                plugin: &spacing::margin::PluginDefinition,
                variants: vec![
                    Variant::Builtin(
                        Default::default(),
                        VariantType::AtRule(Cow::from("@media (width >= 80rem)"))
                    ),
                    Variant::Arbitrary(Cow::from("&>*")),
                    Variant::Builtin(Default::default(), VariantType::PseudoClass("focus"))
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
        let config = Config::default();
        assert_eq!(
            parse(
                "mx-[12px]",
                None,
                None,
                &config,
                &config.get_derived_variants()
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                full: "mx-[12px]",
                order: Default::default(),
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
        let config = Config::default();
        assert_eq!(
            parse(
                "bg-[url('/hello_world.png')]",
                None,
                None,
                &config,
                &config.get_derived_variants(),
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                full: "bg-[url('/hello_world.png')]",
                order: Default::default(),
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
        let config = Config::default();
        assert_eq!(
            parse(
                "bg-[color:#fff]",
                None,
                None,
                &config,
                &config.get_derived_variants()
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                full: "bg-[color:#fff]",
                order: Default::default(),
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
        let config = Config::default();
        assert_eq!(
            parse(
                "xl:marker:bg-[#fff]",
                None,
                None,
                &config,
                &config.get_derived_variants()
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                full: "xl:marker:bg-[#fff]",
                order: Default::default(),
                plugin: &background::background_color::PluginDefinition,
                variants: vec![
                    Variant::Builtin(
                        Default::default(),
                        VariantType::AtRule(Cow::from("@media (width >= 80rem)"))
                    ),
                    Variant::Builtin(
                        Default::default(),
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
        let config = Config::default();
        assert_eq!(
            parse(
                "xl:marker:bg-[color:#fff]",
                None,
                None,
                &config,
                &config.get_derived_variants()
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                full: "xl:marker:bg-[color:#fff]",
                order: Default::default(),
                plugin: &background::background_color::PluginDefinition,
                variants: vec![
                    Variant::Builtin(
                        Default::default(),
                        VariantType::AtRule(Cow::from("@media (width >= 80rem)"))
                    ),
                    Variant::Builtin(
                        Default::default(),
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
        let config = Config::default();
        assert_eq!(
            parse(
                "[&>*]:bg-[#fff]",
                None,
                None,
                &config,
                &config.get_derived_variants()
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                full: "[&>*]:bg-[#fff]",
                order: Default::default(),
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
        let config = Config::default();
        assert_eq!(
            parse(
                r"[\[type='input'\]_&>:*]:bg-red-300",
                None,
                None,
                &config,
                &config.get_derived_variants(),
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                full: r"[\[type='input'\]_&>:*]:bg-red-300",
                order: Default::default(),
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
        let config = Config::default();
        assert_eq!(
            parse(
                "xl:[&>*]:hover:bg-[#fff]",
                None,
                None,
                &config,
                &config.get_derived_variants()
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                full: "xl:[&>*]:hover:bg-[#fff]",
                order: Default::default(),
                plugin: &background::background_color::PluginDefinition,
                variants: vec![
                    Variant::Builtin(
                        Default::default(),
                        VariantType::AtRule(Cow::from("@media (width >= 80rem)"))
                    ),
                    Variant::Arbitrary(Cow::from("&>*")),
                    Variant::Builtin(Default::default(), VariantType::PseudoClass("hover"))
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
        let config = Config::default();
        assert_eq!(
            parse(
                "xl:[&>*]:hover:bg-[color:#fff]",
                None,
                None,
                &config,
                &config.get_derived_variants(),
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                full: "xl:[&>*]:hover:bg-[color:#fff]",
                order: Default::default(),
                plugin: &background::background_color::PluginDefinition,
                variants: vec![
                    Variant::Builtin(
                        Default::default(),
                        VariantType::AtRule(Cow::from("@media (width >= 80rem)"))
                    ),
                    Variant::Arbitrary(Cow::from("&>*")),
                    Variant::Builtin(Default::default(), VariantType::PseudoClass("hover"))
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
        let config = Config::default();
        assert_eq!(
            parse(
                r"bg-[url('/url_with_\]\)\'.png')]",
                None,
                None,
                &config,
                &config.get_derived_variants(),
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                full: r"bg-[url('/url_with_\]\)\'.png')]",
                order: Default::default(),
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
        let config = Config::default();
        assert_eq!(
            parse(
                "hover:[mask-type:luminance]",
                None,
                None,
                &config,
                &config.get_derived_variants(),
            )[0]
            .as_ref()
            .unwrap(),
            &Selector {
                full: "hover:[mask-type:luminance]",
                order: BUILTIN_PLUGINS.len(),
                plugin: &CssPropertyPlugin,
                variants: vec![Variant::Builtin(Default::default(), VariantType::PseudoClass("hover"))],
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
        let config = Config::default();
        assert_eq!(
            parse(
                "hover:(focus:bg-gray-500,text-[color:black,])",
                None,
                None,
                &config,
                &config.get_derived_variants(),
            ),
            vec![
                Ok(Selector {
                    full: "hover:(focus:bg-gray-500,text-[color:black,])",
                    order: Default::default(),
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![
                        Variant::Builtin(Default::default(), VariantType::PseudoClass("focus")),
                        Variant::Builtin(Default::default(), VariantType::PseudoClass("hover"))
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "gray-500",
                    },
                    is_important: false,
                }),
                Ok(Selector {
                    full: "hover:(focus:bg-gray-500,text-[color:black,])",
                    order: Default::default(),
                    plugin: &typography::text_color::PluginDefinition,
                    variants: vec![Variant::Builtin(Default::default(), VariantType::PseudoClass("hover"))],
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
        let config = Config::default();
        assert_eq!(
            parse(
                "hover:(bg-gray-500)",
                None,
                None,
                &config,
                &config.get_derived_variants()
            ),
            vec![Ok(Selector {
                full: "hover:(bg-gray-500)",
                order: Default::default(),
                plugin: &background::background_color::PluginDefinition,
                variants: vec![Variant::Builtin(Default::default(), VariantType::PseudoClass("hover"))],
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
        let config = Config::default();
        assert_eq!(
            parse(
                "focus:([&>*]:-m-4,xl:dark:(bg-red-100,rtl:text-[color:black]))",
                None,
                None,
                &config,
                &config.get_derived_variants(),
            ),
            vec![
                Ok(Selector {
                    full: "focus:([&>*]:-m-4,xl:dark:(bg-red-100,rtl:text-[color:black]))",
                    order: Default::default(),
                    plugin: &spacing::margin::PluginDefinition,
                    variants: vec![
                        Variant::Arbitrary(Cow::from("&>*")),
                        Variant::Builtin(Default::default(), VariantType::PseudoClass("focus")),
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: true,
                        value: "4",
                    },
                    is_important: false,
                }),
                Ok(Selector {
                    full: "focus:([&>*]:-m-4,xl:dark:(bg-red-100,rtl:text-[color:black]))",
                    order: Default::default(),
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![
                        Variant::Builtin(
                            73,
                            VariantType::AtRule(Cow::from("@media (width >= 80rem)"))
                        ),
                        Variant::Builtin(
                            75,
                            VariantType::AtRule(Cow::from("@media (prefers-color-scheme: dark)"))
                        ),
                        Variant::Builtin(Default::default(), VariantType::PseudoClass("focus")),
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "red-100",
                    },
                    is_important: false,
                }),
                Ok(Selector {
                    full: "focus:([&>*]:-m-4,xl:dark:(bg-red-100,rtl:text-[color:black]))",
                    order: Default::default(),
                    plugin: &typography::text_color::PluginDefinition,
                    variants: vec![
                        Variant::Builtin(Default::default(), VariantType::WrapClass(Cow::from("[dir=\"rtl\"] &"))),
                        Variant::Builtin(
                            73,
                            VariantType::AtRule(Cow::from("@media (width >= 80rem)"))
                        ),
                        Variant::Builtin(
                            75,
                            VariantType::AtRule(Cow::from("@media (prefers-color-scheme: dark)"))
                        ),
                        Variant::Builtin(Default::default(), VariantType::PseudoClass("focus")),
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
        let config = Config::default();
        assert_eq!(
            parse(
                r"focus:([&>*]:-m-4,xl:dark:([\[type='text'\].light_&,.foo]:bg-red-100,text-[color:black,]))",
                None,
                None,
                &config,
                &config.get_derived_variants(),
            ),
            vec![
                Ok(Selector {
                    full: r"focus:([&>*]:-m-4,xl:dark:([\[type='text'\].light_&,.foo]:bg-red-100,text-[color:black,]))",
                    order: Default::default(),
                    plugin: &spacing::margin::PluginDefinition,
                    variants: vec![
                        Variant::Arbitrary(Cow::from("&>*")),
                        Variant::Builtin(Default::default(), VariantType::PseudoClass("focus")),
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: true,
                        value: "4",
                    },
                    is_important: false,
                }),
                Ok(Selector {
                    full: r"focus:([&>*]:-m-4,xl:dark:([\[type='text'\].light_&,.foo]:bg-red-100,text-[color:black,]))",
                    order: Default::default(),
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![
                        Variant::Arbitrary(Cow::from(r"[type='text'].light &,.foo")),
                        Variant::Builtin(
                            73,
                            VariantType::AtRule(Cow::from("@media (width >= 80rem)"))
                        ),
                        Variant::Builtin(
                            75,
                            VariantType::AtRule(Cow::from("@media (prefers-color-scheme: dark)"))
                        ),
                        Variant::Builtin(Default::default(), VariantType::PseudoClass("focus")),
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "red-100",
                    },
                    is_important: false,
                }),
                Ok(Selector {
                    full: r"focus:([&>*]:-m-4,xl:dark:([\[type='text'\].light_&,.foo]:bg-red-100,text-[color:black,]))",
                    order: Default::default(),
                    plugin: &typography::text_color::PluginDefinition,
                    variants: vec![
                        Variant::Builtin(
                            73,
                            VariantType::AtRule(Cow::from("@media (width >= 80rem)"))
                        ),
                        Variant::Builtin(
                            75,
                            VariantType::AtRule(Cow::from("@media (prefers-color-scheme: dark)"))
                        ),
                        Variant::Builtin(Default::default(), VariantType::PseudoClass("focus")),
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
        let config = Config::default();
        assert_eq!(
            parse(
                r"xl:(focus:(outline,outline-red-200),dark:(bg-black,text-white))",
                None,
                None,
                &config,
                &config.get_derived_variants(),
            ),
            vec![
                Ok(Selector {
                    full: r"xl:(focus:(outline,outline-red-200),dark:(bg-black,text-white))",
                    order: Default::default(),
                    plugin: &border::outline_style::PluginDefinition,
                    variants: vec![
                        Variant::Builtin(Default::default(), VariantType::PseudoClass("focus")),
                        Variant::Builtin(
                            73,
                            VariantType::AtRule(Cow::from("@media (width >= 80rem)"))
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
                    order: Default::default(),
                    plugin: &border::outline_color::PluginDefinition,
                    variants: vec![
                        Variant::Builtin(Default::default(), VariantType::PseudoClass("focus")),
                        Variant::Builtin(
                            73,
                            VariantType::AtRule(Cow::from("@media (width >= 80rem)"))
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
                    order: Default::default(),
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![
                        Variant::Builtin(
                            75,
                            VariantType::AtRule(Cow::from("@media (prefers-color-scheme: dark)"))
                        ),
                        Variant::Builtin(
                            73,
                            VariantType::AtRule(Cow::from("@media (width >= 80rem)"))
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
                    order: Default::default(),
                    plugin: &typography::text_color::PluginDefinition,
                    variants: vec![
                        Variant::Builtin(
                            75,
                            VariantType::AtRule(Cow::from("@media (prefers-color-scheme: dark)"))
                        ),
                        Variant::Builtin(
                            73,
                            VariantType::AtRule(Cow::from("@media (width >= 80rem)"))
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

    #[test]
    #[allow(clippy::too_many_lines)]
    fn variant_grouping_without_modifier() {
        let config = Config::default();
        assert_eq!(
            parse(
                "(hover,focus):bg-red-400",
                None,
                None,
                &config,
                &config.get_derived_variants(),
            ),
            vec![
                Ok(Selector {
                    full: "(hover,focus):bg-red-400",
                    order: Default::default(),
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![Variant::Builtin(Default::default(), VariantType::PseudoClass("hover"))],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "red-400",
                    },
                    is_important: false,
                }),
                Ok(Selector {
                    full: "(hover,focus):bg-red-400",
                    order: Default::default(),
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![Variant::Builtin(Default::default(), VariantType::PseudoClass("focus")),],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "red-400",
                    },
                    is_important: false,
                })
            ],
        );

        assert_eq!(
            parse(
                "([@supports_(display:flex)],focus-visible):flex",
                None,
                None,
                &config,
                &config.get_derived_variants(),
            ),
            vec![
                Ok(Selector {
                    full: "([@supports_(display:flex)],focus-visible):flex",
                    order: Default::default(),
                    plugin: &flexbox::flex::PluginDefinition,
                    variants: vec![Variant::Arbitrary(Cow::Borrowed(
                        "@supports (display:flex)"
                    ))],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "flex",
                    },
                    is_important: false,
                }),
                Ok(Selector {
                    full: "([@supports_(display:flex)],focus-visible):flex",
                    order: Default::default(),
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![Variant::Builtin(
                        48,
                        VariantType::PseudoClass("focus-visible")
                    ),],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "flex",
                    },
                    is_important: false,
                })
            ],
        );

        assert_eq!(
            parse(
                "([@supports_(display:flex)],focus-visible):!-m-4",
                None,
                None,
                &config,
                &config.get_derived_variants(),
            ),
            vec![
                Ok(Selector {
                    full: "([@supports_(display:flex)],focus-visible):!-m-4",
                    order: Default::default(),
                    plugin: &spacing::margin::PluginDefinition,
                    variants: vec![Variant::Arbitrary(Cow::Borrowed(
                        "@supports (display:flex)"
                    ))],
                    modifier: Modifier::Builtin {
                        is_negative: true,
                        value: "4",
                    },
                    is_important: true,
                }),
                Ok(Selector {
                    full: "([@supports_(display:flex)],focus-visible):!-m-4",
                    order: Default::default(),
                    plugin: &spacing::margin::PluginDefinition,
                    variants: vec![Variant::Builtin(
                        48,
                        VariantType::PseudoClass("focus-visible")
                    ),],
                    modifier: Modifier::Builtin {
                        is_negative: true,
                        value: "4",
                    },
                    is_important: true,
                })
            ],
        );

        assert_eq!(
            parse(
                "xl:(hover,focus):bg-red-400",
                None,
                None,
                &config,
                &config.get_derived_variants(),
            ),
            vec![
                Ok(Selector {
                    full: "xl:(hover,focus):bg-red-400",
                    order: Default::default(),
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![
                        Variant::Builtin(
                            73,
                            VariantType::AtRule(Cow::Borrowed("@media (width >= 80rem)"))
                        ),
                        Variant::Builtin(Default::default(), VariantType::PseudoClass("hover"))
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "red-400",
                    },
                    is_important: false,
                }),
                Ok(Selector {
                    full: "xl:(hover,focus):bg-red-400",
                    order: Default::default(),
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![
                        Variant::Builtin(
                            73,
                            VariantType::AtRule(Cow::Borrowed("@media (width >= 80rem)"))
                        ),
                        Variant::Builtin(Default::default(), VariantType::PseudoClass("focus")),
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "red-400",
                    },
                    is_important: false,
                })
            ],
        );

        // Not a variant without a modifier but we need to make sure it is correctly interpreted as
        // variants **with** a modifier
        assert_eq!(
            parse(
                "(hover:bg-red-400,focus:bg-green-400)",
                None,
                None,
                &config,
                &config.get_derived_variants(),
            ),
            vec![
                Ok(Selector {
                    full: "(hover:bg-red-400,focus:bg-green-400)",
                    order: Default::default(),
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![Variant::Builtin(Default::default(), VariantType::PseudoClass("hover"))],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "red-400",
                    },
                    is_important: false,
                }),
                Ok(Selector {
                    full: "(hover:bg-red-400,focus:bg-green-400)",
                    order: Default::default(),
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![Variant::Builtin(Default::default(), VariantType::PseudoClass("focus")),],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "green-400",
                    },
                    is_important: false,
                })
            ],
        );

        assert_eq!(
            parse(
                "xl:(hover,focus):target:(dark:bg-red-400,bg-green-400)",
                None,
                None,
                &config,
                &config.get_derived_variants(),
            ),
            vec![
                Ok(Selector {
                    full: "xl:(hover,focus):target:(dark:bg-red-400,bg-green-400)",
                    order: Default::default(),
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![
                        Variant::Builtin(
                            75,
                            VariantType::AtRule(Cow::Borrowed(
                                "@media (prefers-color-scheme: dark)"
                            ))
                        ),
                        Variant::Builtin(
                            73,
                            VariantType::AtRule(Cow::Borrowed("@media (width >= 80rem)"))
                        ),
                        Variant::Builtin(Default::default(), VariantType::PseudoClass("hover")),
                        Variant::Builtin(Default::default(), VariantType::PseudoClass("target")),
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "red-400",
                    },
                    is_important: false,
                }),
                Ok(Selector {
                    full: "xl:(hover,focus):target:(dark:bg-red-400,bg-green-400)",
                    order: Default::default(),
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![
                        Variant::Builtin(
                            75,
                            VariantType::AtRule(Cow::Borrowed(
                                "@media (prefers-color-scheme: dark)"
                            ))
                        ),
                        Variant::Builtin(
                            73,
                            VariantType::AtRule(Cow::Borrowed("@media (width >= 80rem)"))
                        ),
                        Variant::Builtin(Default::default(), VariantType::PseudoClass("focus")),
                        Variant::Builtin(Default::default(), VariantType::PseudoClass("target")),
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "red-400",
                    },
                    is_important: false,
                }),
                Ok(Selector {
                    full: "xl:(hover,focus):target:(dark:bg-red-400,bg-green-400)",
                    order: Default::default(),
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![
                        Variant::Builtin(
                            73,
                            VariantType::AtRule(Cow::Borrowed("@media (width >= 80rem)"))
                        ),
                        Variant::Builtin(Default::default(), VariantType::PseudoClass("hover")),
                        Variant::Builtin(Default::default(), VariantType::PseudoClass("target"))
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "green-400",
                    },
                    is_important: false,
                }),
                Ok(Selector {
                    full: "xl:(hover,focus):target:(dark:bg-red-400,bg-green-400)",
                    order: Default::default(),
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![
                        Variant::Builtin(
                            73,
                            VariantType::AtRule(Cow::Borrowed("@media (width >= 80rem)"))
                        ),
                        Variant::Builtin(Default::default(), VariantType::PseudoClass("focus")),
                        Variant::Builtin(Default::default(), VariantType::PseudoClass("target"))
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "green-400",
                    },
                    is_important: false,
                }),
            ],
        );

        assert_eq!(
            parse(
                "(hover,focus):(focus-within,target):bg-red-400",
                None,
                None,
                &config,
                &config.get_derived_variants(),
            ),
            vec![
                Ok(Selector {
                    full: "(hover,focus):(focus-within,target):bg-red-400",
                    order: Default::default(),
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![
                        Variant::Builtin(Default::default(), VariantType::PseudoClass("hover")),
                        Variant::Builtin(Default::default(), VariantType::PseudoClass("focus-within")),
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "red-400",
                    },
                    is_important: false,
                }),
                Ok(Selector {
                    full: "(hover,focus):(focus-within,target):bg-red-400",
                    order: Default::default(),
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![
                        Variant::Builtin(Default::default(), VariantType::PseudoClass("focus")),
                        Variant::Builtin(Default::default(), VariantType::PseudoClass("focus-within"))
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "red-400",
                    },
                    is_important: false,
                }),
                Ok(Selector {
                    full: "(hover,focus):(focus-within,target):bg-red-400",
                    order: Default::default(),
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![
                        Variant::Builtin(Default::default(), VariantType::PseudoClass("hover")),
                        Variant::Builtin(Default::default(), VariantType::PseudoClass("target")),
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "red-400",
                    },
                    is_important: false,
                }),
                Ok(Selector {
                    full: "(hover,focus):(focus-within,target):bg-red-400",
                    order: Default::default(),
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![
                        Variant::Builtin(Default::default(), VariantType::PseudoClass("focus")),
                        Variant::Builtin(Default::default(), VariantType::PseudoClass("target"))
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "red-400",
                    },
                    is_important: false,
                })
            ],
        );

        assert_eq!(
            parse(
                "(hover,focus):(bg-red-400,(target,focus-within):bg-green-400)",
                None,
                None,
                &config,
                &config.get_derived_variants(),
            ),
            vec![
                Ok(Selector {
                    full: "(hover,focus):(bg-red-400,(target,focus-within):bg-green-400)",
                    order: Default::default(),
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![Variant::Builtin(Default::default(), VariantType::PseudoClass("hover")),],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "red-400",
                    },
                    is_important: false,
                }),
                Ok(Selector {
                    full: "(hover,focus):(bg-red-400,(target,focus-within):bg-green-400)",
                    order: Default::default(),
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![Variant::Builtin(Default::default(), VariantType::PseudoClass("focus")),],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "red-400",
                    },
                    is_important: false,
                }),
                Ok(Selector {
                    full: "(hover,focus):(bg-red-400,(target,focus-within):bg-green-400)",
                    order: Default::default(),
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![
                        Variant::Builtin(Default::default(), VariantType::PseudoClass("target")),
                        Variant::Builtin(Default::default(), VariantType::PseudoClass("hover")),
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "green-400",
                    },
                    is_important: false,
                }),
                Ok(Selector {
                    full: "(hover,focus):(bg-red-400,(target,focus-within):bg-green-400)",
                    order: Default::default(),
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![
                        Variant::Builtin(Default::default(), VariantType::PseudoClass("target")),
                        Variant::Builtin(Default::default(), VariantType::PseudoClass("focus")),
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "green-400",
                    },
                    is_important: false,
                }),
                Ok(Selector {
                    full: "(hover,focus):(bg-red-400,(target,focus-within):bg-green-400)",
                    order: Default::default(),
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![
                        Variant::Builtin(Default::default(), VariantType::PseudoClass("focus-within")),
                        Variant::Builtin(Default::default(), VariantType::PseudoClass("hover"))
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "green-400",
                    },
                    is_important: false,
                }),
                Ok(Selector {
                    full: "(hover,focus):(bg-red-400,(target,focus-within):bg-green-400)",
                    order: Default::default(),
                    plugin: &background::background_color::PluginDefinition,
                    variants: vec![
                        Variant::Builtin(Default::default(), VariantType::PseudoClass("focus-within")),
                        Variant::Builtin(Default::default(), VariantType::PseudoClass("focus"))
                    ],
                    modifier: Modifier::Builtin {
                        is_negative: false,
                        value: "green-400",
                    },
                    is_important: false,
                }),
            ],
        );
    }
}
