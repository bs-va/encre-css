use super::{to_css_value, Plugin};
use crate::{context::{ContextCanHandle, ContextHandle}, utils::{indent, length, value_matchers::*}, selector::Modifier};

use std::borrow::Cow;
use std::fmt::{self, Write};

pub fn margin_padding_can_handle(context: ContextCanHandle) -> bool {
    match context.modifier {
        Modifier::Basic { is_negative, value } => {
            *value == "auto" || length::get_basic(value, *is_negative).is_some()
        }
        Modifier::Arbitrary { value, .. } => is_matching_length(value),
    }
}

pub fn margin_padding_handle(
    css_properties: &[&str],
    context: ContextHandle,
) -> fmt::Result {
    match context.modifier {
        Modifier::Basic { is_negative, value } => {
            for css_prop in css_properties {
                indent(context.indentation, context.buffer)?;
                writeln!(
                    context.buffer,
                    "{}: {};",
                    css_prop,
                    if *value == "auto" {
                        Cow::from("auto")
                    } else {
                        length::get_basic(value, *is_negative).unwrap()
                    }
                )?
            }
        }
        Modifier::Arbitrary { value, .. } => {
            let value = to_css_value(value);
            for css_prop in css_properties {
                indent(context.indentation, context.buffer)?;
                writeln!(context.buffer, "{}: {};", css_prop, value)?;
            }
        }
    }

    Ok(())
}

// Margin

#[derive(Debug)]
pub struct MarginPlugin;

impl Plugin for MarginPlugin {
    fn namespace(&self) -> &str {
        "m"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { is_negative, value } => {
                *value == "auto" || length::get_basic(value, *is_negative).is_some()
            }
            Modifier::Arbitrary { prefix, value, .. } => {
                prefix.is_empty() && is_matching_length(value)
            }
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        margin_padding_handle(&["margin"], context)
    }
}

#[derive(Debug)]
pub struct MarginXPlugin;

impl Plugin for MarginXPlugin {
    fn namespace(&self) -> &str {
        "mx"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        margin_padding_can_handle(context)
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        margin_padding_handle(&["margin-left", "margin-right"], context)
    }
}

#[derive(Debug)]
pub struct MarginYPlugin;

impl Plugin for MarginYPlugin {
    fn namespace(&self) -> &str {
        "my"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        margin_padding_can_handle(context)
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        margin_padding_handle(&["margin-top", "margin-bottom"], context)
    }
}

#[derive(Debug)]
pub struct MarginTopPlugin;

impl Plugin for MarginTopPlugin {
    fn namespace(&self) -> &str {
        "mt"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        margin_padding_can_handle(context)
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        margin_padding_handle(&["margin-top"], context)
    }
}

#[derive(Debug)]
pub struct MarginBottomPlugin;

impl Plugin for MarginBottomPlugin {
    fn namespace(&self) -> &str {
        "mb"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        margin_padding_can_handle(context)
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        margin_padding_handle(&["margin-bottom"], context)
    }
}

#[derive(Debug)]
pub struct MarginLeftPlugin;

impl Plugin for MarginLeftPlugin {
    fn namespace(&self) -> &str {
        "ml"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        margin_padding_can_handle(context)
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        margin_padding_handle(&["margin-left"], context)
    }
}

#[derive(Debug)]
pub struct MarginRightPlugin;

impl Plugin for MarginRightPlugin {
    fn namespace(&self) -> &str {
        "mr"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        margin_padding_can_handle(context)
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        margin_padding_handle(&["margin-right"], context)
    }
}

// Padding

#[derive(Debug)]
pub struct PaddingPlugin;

impl Plugin for PaddingPlugin {
    fn namespace(&self) -> &str {
        "p"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { is_negative, value } => {
                *value == "auto" || length::get_basic(value, *is_negative).is_some()
            }
            Modifier::Arbitrary { prefix, value, .. } => {
                prefix.is_empty() && is_matching_length(value)
            }
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        margin_padding_handle(&["padding"], context)
    }
}

#[derive(Debug)]
pub struct PaddingXPlugin;

impl Plugin for PaddingXPlugin {
    fn namespace(&self) -> &str {
        "px"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        margin_padding_can_handle(context)
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        margin_padding_handle(&["padding-left", "padding-right"], context)
    }
}

#[derive(Debug)]
pub struct PaddingYPlugin;

impl Plugin for PaddingYPlugin {
    fn namespace(&self) -> &str {
        "py"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        margin_padding_can_handle(context)
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        margin_padding_handle(&["padding-top", "padding-bottom"], context)
    }
}

#[derive(Debug)]
pub struct PaddingTopPlugin;

impl Plugin for PaddingTopPlugin {
    fn namespace(&self) -> &str {
        "pt"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        margin_padding_can_handle(context)
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        margin_padding_handle(&["padding-top"], context)
    }
}

#[derive(Debug)]
pub struct PaddingBottomPlugin;

impl Plugin for PaddingBottomPlugin {
    fn namespace(&self) -> &str {
        "pb"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        margin_padding_can_handle(context)
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        margin_padding_handle(&["padding-bottom"], context)
    }
}

#[derive(Debug)]
pub struct PaddingLeftPlugin;

impl Plugin for PaddingLeftPlugin {
    fn namespace(&self) -> &str {
        "pl"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        margin_padding_can_handle(context)
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        margin_padding_handle(&["padding-left"], context)
    }
}

#[derive(Debug)]
pub struct PaddingRightPlugin;

impl Plugin for PaddingRightPlugin {
    fn namespace(&self) -> &str {
        "pr"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        margin_padding_can_handle(context)
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        margin_padding_handle(&["padding-right"], context)
    }
}

// Spacing

#[derive(Debug)]
pub struct SpaceXPlugin;

impl Plugin for SpaceXPlugin {
    fn namespace(&self) -> &str {
        "space-x"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { is_negative, value } => {
                *value == "reverse" || length::get_basic(value, *is_negative).is_some()
            }
            Modifier::Arbitrary { value, .. } => is_matching_length(value),
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        // TODO: class with `> :not([hidden]) ~ :not([hidden])`
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { is_negative, value } => {
                if *value == "reverse" {
                    return writeln!(context.buffer, "--en-space-x-reverse: 1;");
                }

                let length = length::get_basic(value, *is_negative).unwrap();
                writeln!(context.buffer, "--en-space-x-reverse: 0;")?;
                indent(context.indentation, context.buffer)?;
                writeln!(
                    context.buffer,
                    "margin-left: calc({length} * calc(1 - var(--en-space-x-reverse)));"
                )?;
                indent(context.indentation, context.buffer)?;
                writeln!(
                    context.buffer,
                    "margin-right: calc({length} * var(--en-space-x-reverse));"
                )?;
            }
            Modifier::Arbitrary { value, .. } => {
                let value = to_css_value(value);
                writeln!(context.buffer, "--en-space-x-reverse: 0;")?;
                indent(context.indentation, context.buffer)?;
                writeln!(
                    context.buffer,
                    "margin-left: calc({value} * calc(1 - var(--en-space-x-reverse)));"
                )?;
                indent(context.indentation, context.buffer)?;
                writeln!(
                    context.buffer,
                    "margin-right: calc({value} * var(--en-space-x-reverse));"
                )?;
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct SpaceYPlugin;

impl Plugin for SpaceYPlugin {
    fn namespace(&self) -> &str {
        "space-y"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { is_negative, value } => {
                *value == "reverse" || length::get_basic(value, *is_negative).is_some()
            }
            Modifier::Arbitrary { value, .. } => is_matching_length(value),
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        // TODO: class with `> :not([hidden]) ~ :not([hidden])`
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { is_negative, value } => {
                if *value == "reverse" {
                    return writeln!(context.buffer, "--en-space-y-reverse: 1;");
                }

                let length = length::get_basic(value, *is_negative).unwrap();
                writeln!(context.buffer, "--en-space-y-reverse: 0;")?;
                indent(context.indentation, context.buffer)?;
                writeln!(
                    context.buffer,
                    "margin-top: calc({length} * calc(1 - var(--en-space-y-reverse)));"
                )?;
                indent(context.indentation, context.buffer)?;
                writeln!(
                    context.buffer,
                    "margin-bottom: calc({length} * var(--en-space-y-reverse));"
                )?;
            }
            Modifier::Arbitrary { value, .. } => {
                let value = to_css_value(value);
                writeln!(context.buffer, "--en-space-y-reverse: 0;")?;
                indent(context.indentation, context.buffer)?;
                writeln!(
                    context.buffer,
                    "margin-top: calc({value} * calc(1 - var(--en-space-y-reverse)));"
                )?;
                indent(context.indentation, context.buffer)?;
                writeln!(
                    context.buffer,
                    "margin-bottom: calc({value} * var(--en-space-y-reverse));"
                )?;
            }
        }

        Ok(())
    }
}
