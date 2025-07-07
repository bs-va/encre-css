#![doc = include_str!("README.md")]
#![doc(alias("spacing", "space"))]
use crate::prelude::build_plugin::*;

#[derive(Debug)]
pub(crate) struct PluginXDefinition;

impl Plugin for PluginXDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                *value == "reverse" || spacing::is_matching_builtin_spacing(value)
            }
            Modifier::Arbitrary { value, .. } => {
                is_matching_length(value) || is_matching_percentage(value)
            }
        }
    }

    fn needs_wrapping(&self) -> bool {
        false
    }

    fn handle(&self, context: &mut ContextHandle) {
        generate_at_rules(context, |context| {
            generate_class(
                context,
                |context| match context.modifier {
                    Modifier::Builtin { is_negative, value } => {
                        if *value == "reverse" {
                            return context.buffer.line("--en-space-x-reverse: 1;");
                        }

                        let length = spacing::get(value, *is_negative).unwrap();
                        context.buffer.lines([
                            format_args!("--en-space-x-reverse: 0;"),
                            format_args!("margin-inline-start: calc({length} * var(--en-space-x-reverse));"),
                            format_args!("margin-inline-end: calc({length} * calc(1 - var(--en-space-x-reverse)));"),
                        ]);
                    }
                    Modifier::Arbitrary { value, .. } => {
                        context.buffer.lines([
                            format_args!("--en-space-x-reverse: 0;"),
                            format_args!(
                                "margin-inline-start: calc({value} * var(--en-space-x-reverse));"
                            ),
                            format_args!(
                                "margin-inline-end: calc({value} * calc(1 - var(--en-space-x-reverse)));"
                            ),
                        ]);
                    }
                },
                " > :not(:last-child)",
            );
        });
    }
}

#[derive(Debug)]
pub(crate) struct PluginYDefinition;

impl Plugin for PluginYDefinition {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                *value == "reverse" || spacing::is_matching_builtin_spacing(value)
            }
            Modifier::Arbitrary { value, .. } => {
                is_matching_length(value) || is_matching_percentage(value)
            }
        }
    }

    fn needs_wrapping(&self) -> bool {
        false
    }

    fn handle(&self, context: &mut ContextHandle) {
        generate_at_rules(context, |context| {
            generate_class(
                context,
                |context| match context.modifier {
                    Modifier::Builtin { is_negative, value } => {
                        if *value == "reverse" {
                            return context.buffer.line("--en-space-y-reverse: 1;");
                        }

                        let length = spacing::get(value, *is_negative).unwrap();
                        context.buffer.lines([
                            format_args!("--en-space-y-reverse: 0;"),
                            format_args!(
                                "margin-block-start: calc({length} * calc(1 - var(--en-space-y-reverse)));"
                            ),
                            format_args!(
                                "margin-block-end: calc({length} * var(--en-space-y-reverse));"
                            ),
                        ]);
                    }
                    Modifier::Arbitrary { value, .. } => {
                        context.buffer.lines([
                            format_args!("--en-space-y-reverse: 0;"),
                            format_args!(
                                "margin-block-start: calc({value} * calc(1 - var(--en-space-y-reverse)));"
                            ),
                            format_args!(
                                "margin-block-end: calc({value} * var(--en-space-y-reverse));"
                            ),
                        ]);
                    }
                },
                " > :not(:last-child)",
            );
        });
    }
}
