use super::{to_css_value, Plugin};
use crate::{
    context::{ContextCanHandle, ContextHandle},
    selector::Modifier,
    utils::{indent, value_matchers::*},
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub struct OrderPlugin;

impl Plugin for OrderPlugin {
    fn namespace(&self) -> &str {
        "order"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => ["first", "last", "none"].contains(&&**value),
            Modifier::Arbitrary { value, .. } => is_matching_number(value),
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "first" => return writeln!(context.buffer, "order: -9999;"),
                "last" => return writeln!(context.buffer, "order: 9999;"),
                "none" => return writeln!(context.buffer, "order: 0;"),
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "order: {};", to_css_value(value))?
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct DirectionPlugin;

impl Plugin for DirectionPlugin {
    fn namespace(&self) -> &str {
        "flex"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => {
                ["row", "row-reverse", "col", "col-reverse"].contains(&&**value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "row" => writeln!(context.buffer, "flex-direction: row;")?,
                "row-reverse" => writeln!(context.buffer, "flex-direction: row-reverse;")?,
                "col" => writeln!(context.buffer, "flex-direction: column;")?,
                "col-reverse" => writeln!(context.buffer, "flex-direction: column-reverse;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct WrapPlugin;

impl Plugin for WrapPlugin {
    fn namespace(&self) -> &str {
        "flex"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => ["nowrap", "wrap", "wrap-reverse"].contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "nowrap" => writeln!(context.buffer, "flex-wrap: nowrap;")?,
                "wrap" => writeln!(context.buffer, "flex-wrap: wrap;")?,
                "wrap-reverse" => writeln!(context.buffer, "flex-wrap: wrap-reverse;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct GrowShrinkBasisPlugin;

impl Plugin for GrowShrinkBasisPlugin {
    fn namespace(&self) -> &str {
        "flex"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => [
                "1", "auto", "initial", "grow", "grow-0", "shrink", "shrink-0", "none",
            ]
            .contains(&&**value),
            Modifier::Arbitrary { value, .. } => {
                let mut split = value.split('_');
                let mut is_matching = (false, false, false);

                // flex-grow
                // https://developer.mozilla.org/en-US/docs/Web/CSS/flex-grow#values
                if let Some(value) = split.next() {
                    if is_matching_number(value) {
                        is_matching.0 = true;
                    }
                }

                // flex-shrink
                // https://developer.mozilla.org/en-US/docs/Web/CSS/flex-shrink#values
                if let Some(value) = split.next() {
                    if is_matching_number(value) {
                        is_matching.1 = true;
                    }
                }

                // flex-basis
                // https://developer.mozilla.org/en-US/docs/Web/CSS/flex-basis#values
                if let Some(value) = split.next() {
                    if is_matching_length(value) || is_matching_percentage(value) || value == "auto"
                    {
                        is_matching.2 = true;
                    }
                }

                is_matching.0 && is_matching.1 && is_matching.2
            }
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "1" => writeln!(context.buffer, "flex: 1 1 0%;")?,
                "auto" => writeln!(context.buffer, "flex: 1 1 auto;")?,
                "initial" => writeln!(context.buffer, "flex: 0 1 auto;")?,
                "none" => writeln!(context.buffer, "flex: none;")?,
                "grow" => writeln!(context.buffer, "flex-grow: 1;")?,
                "grow-0" => writeln!(context.buffer, "flex-grow: 0;")?,
                "shrink" => writeln!(context.buffer, "flex-shrink: 1;")?,
                "shrink-0" => writeln!(context.buffer, "flex-shrink: 0;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "flex: {};", to_css_value(value))?
            }
        }

        Ok(())
    }
}
