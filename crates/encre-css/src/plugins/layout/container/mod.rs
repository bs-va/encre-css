#![doc = include_str!("README.md")]
use crate::{
    config::BUILTIN_SCREENS,
    generator::{
        generate_at_rules, generate_class, generate_wrapper, ContextCanHandle, ContextHandle,
    },
    plugins::Plugin,
    selector::Modifier,
    utils::indent,
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

    fn needs_wrapping(&self) -> bool {
        false
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => value.is_empty(),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        if let Modifier::Builtin { .. } = context.modifier {
            generate_wrapper(context, |context| {
                indent(context.indentation, context.buffer)?;
                writeln!(context.buffer, "width: 100%;")
            })?;

            write!(context.buffer, "\n\n")?;

            let mut first_child = true;

            generate_at_rules(context, |context| {
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
                    } else if context.indentation == 0 {
                        write!(context.buffer, "\n\n")?;
                    } else {
                        writeln!(context.buffer)?;
                    }

                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "@media (min-width: {screen}) {{")?;
                    context.indentation += 1;

                    generate_class(
                        context,
                        |context| {
                            indent(context.indentation, context.buffer)?;
                            writeln!(context.buffer, "max-width: {screen};")
                        },
                        "",
                    )?;

                    context.indentation -= 1;
                    if context.indentation == 0 {
                        write!(context.buffer, "}}")?;
                    } else {
                        indent(context.indentation, context.buffer)?;
                        writeln!(context.buffer, "}}")?;
                    }
                }

                // After rule
                while context.indentation > 0 {
                    context.indentation -= 1;

                    if context.indentation == 0 {
                        write!(context.buffer, "}}")?;
                    } else {
                        indent(context.indentation, context.buffer)?;
                        writeln!(context.buffer, "}}")?;
                    }
                }

                Ok(())
            })
        } else {
            Ok(())
        }
    }
}
