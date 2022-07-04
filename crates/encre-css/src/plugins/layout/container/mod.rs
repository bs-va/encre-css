#![doc = include_str!("README.md")]
use crate::{
    config::{BUILTIN_SCREENS, BUILTIN_VARIANTS},
    context::{ContextAfterRule, ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::{Modifier, VARIANT_SEPARATOR},
    utils::indent,
    variant::Variant,
};

use std::{
    borrow::Cow,
    cmp::Ordering,
    fmt::{self, Write},
};

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "container"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => value.is_empty(),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn css_after_rule(&self, context: ContextAfterRule) -> fmt::Result {
        if let Modifier::Builtin { .. } = context.selector.modifier {
            let get_variant = |variant| {
                BUILTIN_VARIANTS
                    .iter()
                    .find_map(|v| if v.0 == variant { Some(&v.1) } else { None })
                    .or_else(|| context.custom_variants.get(&variant))
            };

            let mut indentation = 0;
            let mut first_child = false;

            // Support variants before rule
            if !context.selector.variants.is_empty() {
                context
                    .selector
                    .variants
                    .split(VARIANT_SEPARATOR)
                    .try_for_each(|variant| {
                        if let Some(Variant::AtRule(variant)) = get_variant(Cow::from(variant)) {
                            if !first_child {
                                write!(context.buffer, "\n\n")?;
                            }

                            indent(indentation, context.buffer)?;
                            writeln!(context.buffer, "{} {{", variant)?;
                            indentation += 1;
                            first_child = true;
                        }

                        Ok::<(), fmt::Error>(())
                    })?;
            }

            let mut screens = context
                .config
                .theme
                .screens
                .iter()
                .map(|(a, b)| (a.clone(), b.clone()))
                .chain(
                    BUILTIN_SCREENS
                        .iter()
                        .map(|(a, b)| (Cow::from(*a), Cow::from(*b))),
                )
                .collect::<Vec<(Cow<str>, Cow<str>)>>();

            // Deduplicate screens
            screens.sort_by(|a, b| a.0.cmp(&b.0));
            screens.dedup_by(|a, b| a.0.eq(&b.0));

            // Emulate Tailwind sorting (based on the JS `parseInt` function)
            screens.sort_by(|a, b| {
                let a = if let Some(first_char_a) = a.1.chars().position(char::is_alphabetic) {
                    a.1[..first_char_a].parse::<usize>().ok()
                } else {
                    a.1.parse::<usize>().ok()
                };

                let b = if let Some(first_char_b) = b.1.chars().position(char::is_alphabetic) {
                    b.1[..first_char_b].parse::<usize>().ok()
                } else {
                    b.1.parse::<usize>().ok()
                };

                if let Some(a) = a {
                    if let Some(b) = b {
                        a.cmp(&b)
                    } else {
                        Ordering::Less
                    }
                } else {
                    Ordering::Greater
                }
            });

            for (_, screen) in &screens {
                if first_child {
                    first_child = false;
                } else {
                    write!(context.buffer, "\n\n")?;
                }

                indent(indentation, context.buffer)?;
                writeln!(context.buffer, "@media (min-width: {screen}) {{")?;
                indent(indentation + 1, context.buffer)?;
                write!(
                    context.buffer,
                    "{}",
                    context.selector.get_css_class(context.custom_variants)
                )?;
                writeln!(context.buffer, " {{")?;
                indent(indentation + 2, context.buffer)?;
                writeln!(context.buffer, "max-width: {screen};")?;
                indent(indentation + 1, context.buffer)?;
                writeln!(context.buffer, "}}")?;
                indent(indentation, context.buffer)?;
                write!(context.buffer, "}}")?;
            }

            // After rule
            for i in (1..=indentation).rev() {
                writeln!(context.buffer)?;
                indent(i - 1, context.buffer)?;
                write!(context.buffer, "}}")?;
            }
        }

        Ok(())
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        if let Modifier::Builtin { .. } = context.modifier {
            writeln!(context.buffer, "width: 100%;")?;
        }

        Ok(())
    }
}
