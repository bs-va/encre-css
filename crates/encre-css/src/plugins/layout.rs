use super::{to_css_value, Plugin};
use crate::{
    config::BUILTIN_SCREENS,
    context::{ContextAfterRule, ContextCanHandle, ContextHandle},
    selector::Modifier,
    utils::{indent, length, value_matchers::*},
    variant::{VARIANT_SEPARATOR, BUILTIN_VARIANTS, Variant},
};

use std::{
    borrow::Cow,
    cmp::Ordering,
    fmt::{self, Write},
};

#[derive(Debug)]
pub struct AspectRatioPlugin;

impl Plugin for AspectRatioPlugin {
    fn namespace(&self) -> &'static str {
        "aspect"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => ["auto", "square", "video"].contains(&&**value),
            Modifier::Arbitrary { value, .. } => is_matching_all(value),
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "auto" => writeln!(context.buffer, "aspect-ratio: auto;")?,
                "square" => writeln!(context.buffer, "aspect-ratio: 1 / 1;")?,
                "video" => writeln!(context.buffer, "aspect-ratio: 16 / 9;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => writeln!(
                context.buffer,
                "aspect-ratio: {};",
                to_css_value(&value.replace('/', " / "))
            )?,
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct PositionPlugin;

impl Plugin for PositionPlugin {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => {
                ["static", "fixed", "absolute", "relative", "sticky"].contains(&&**value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => writeln!(context.buffer, "position: {value};")?,
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct DisplayPlugin;

impl Plugin for DisplayPlugin {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => [
                "hidden",
                "contents",
                "list-item",
                "block",
                "inline-block",
                "flex",
                "inline-flex",
                "inline",
                "table",
                "inline-table",
                "table-cell",
                "table-caption",
                "table-column",
                "table-column-group",
                "table-footer-group",
                "table-header-group",
                "table-row-group",
                "table-row",
                "flow-root",
                "grid",
                "inline-grid",
            ]
            .contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "hidden" => writeln!(context.buffer, "display: none;")?,
                "contents" => writeln!(context.buffer, "display: contents;")?,
                "list-item" => writeln!(context.buffer, "display: list-item;")?,
                "block" => writeln!(context.buffer, "display: block;")?,
                "inline-block" => writeln!(context.buffer, "display: inline-block;")?,
                "flex" => writeln!(context.buffer, "display: flex;")?,
                "inline-flex" => writeln!(context.buffer, "display: inline-flex;")?,
                "inline" => writeln!(context.buffer, "display: inline;")?,
                "table" => writeln!(context.buffer, "display: table;")?,
                "inline-table" => writeln!(context.buffer, "display: inline-table;")?,
                "table-cell" => writeln!(context.buffer, "display: table-cell;")?,
                "table-caption" => writeln!(context.buffer, "display: table-caption;")?,
                "table-column" => writeln!(context.buffer, "display: table-column;")?,
                "table-column-group" => writeln!(context.buffer, "display: table-column-group;")?,
                "table-footer-group" => writeln!(context.buffer, "display: table-footer-group;")?,
                "table-header-group" => writeln!(context.buffer, "display: table-header-group;")?,
                "table-row-group" => writeln!(context.buffer, "display: table-row-group;")?,
                "table-row" => writeln!(context.buffer, "display: table-row;")?,
                "flow-root" => writeln!(context.buffer, "display: flow-root;")?,
                "grid" => writeln!(context.buffer, "display: grid;")?,
                "inline-grid" => writeln!(context.buffer, "display: inline-grid;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct VisibilityPlugin;

impl Plugin for VisibilityPlugin {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => ["visible", "invisible"].contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "visible" => writeln!(context.buffer, "visibility: visible;")?,
                "invisible" => writeln!(context.buffer, "visibility: hidden;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct IsolationPlugin;

impl Plugin for IsolationPlugin {
    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => ["isolate", "isolation-auto"].contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "isolate" => writeln!(context.buffer, "isolation: isolate;")?,
                "isolation-auto" => writeln!(context.buffer, "isolation: auto;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

pub fn position_can_handle(context: ContextCanHandle) -> bool {
    match context.modifier {
        Modifier::Basic { is_negative, value } => {
            length::get_extended(value, *is_negative).is_some()
        }
        Modifier::Arbitrary { value, .. } => {
            is_matching_length(value) || is_matching_percentage(value) || *value == "auto"
        }
    }
}

pub fn position_handle(css_properties: &[&str], context: ContextHandle) -> fmt::Result {
    match context.modifier {
        Modifier::Basic { is_negative, value } => {
            for css_prop in css_properties {
                indent(context.indentation, context.buffer)?;
                writeln!(
                    context.buffer,
                    "{}: {};",
                    css_prop,
                    length::get_extended(value, *is_negative).unwrap(),
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

#[derive(Debug)]
pub struct InsetPlugin;

impl Plugin for InsetPlugin {
    fn namespace(&self) -> &str {
        "inset"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { is_negative, value } => {
                length::get_extended(value, *is_negative).is_some()
            }
            Modifier::Arbitrary { prefix, value, .. } => {
                prefix.is_empty()
                    && (is_matching_length(value)
                        || is_matching_percentage(value)
                        || *value == "auto")
            }
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        position_handle(&["top", "bottom", "left", "right"], context)
    }
}

#[derive(Debug)]
pub struct InsetXPlugin;

impl Plugin for InsetXPlugin {
    fn namespace(&self) -> &str {
        "inset-x"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        position_can_handle(context)
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        position_handle(&["left", "right"], context)
    }
}

#[derive(Debug)]
pub struct InsetYPlugin;

impl Plugin for InsetYPlugin {
    fn namespace(&self) -> &str {
        "inset-y"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        position_can_handle(context)
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        position_handle(&["top", "bottom"], context)
    }
}

#[derive(Debug)]
pub struct TopPlugin;

impl Plugin for TopPlugin {
    fn namespace(&self) -> &str {
        "top"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        position_can_handle(context)
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        position_handle(&["top"], context)
    }
}

#[derive(Debug)]
pub struct BottomPlugin;

impl Plugin for BottomPlugin {
    fn namespace(&self) -> &str {
        "bottom"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        position_can_handle(context)
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        position_handle(&["bottom"], context)
    }
}

#[derive(Debug)]
pub struct LeftPlugin;

impl Plugin for LeftPlugin {
    fn namespace(&self) -> &str {
        "left"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        position_can_handle(context)
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        position_handle(&["left"], context)
    }
}

#[derive(Debug)]
pub struct RightPlugin;

impl Plugin for RightPlugin {
    fn namespace(&self) -> &str {
        "right"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        position_can_handle(context)
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        position_handle(&["right"], context)
    }
}

#[derive(Debug)]
pub struct ZIndexPlugin;

impl Plugin for ZIndexPlugin {
    fn namespace(&self) -> &str {
        "z"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => value.parse::<usize>().is_ok() || *value == "auto",
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        // NOTE: Not-compatible with TailwindCSS, support all values
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => writeln!(context.buffer, "z-index: {value};")?,
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct ContainerPlugin;

impl Plugin for ContainerPlugin {
    fn namespace(&self) -> &str {
        "container"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => value.is_empty(),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn css_after_rule(&self, context: ContextAfterRule) -> fmt::Result {
        if let Modifier::Basic { .. } = context.selector.modifier {
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
                context.selector
                    .variants
                    .split(VARIANT_SEPARATOR)
                    .try_for_each(|variant| {
                        if let Some(Variant::BeforeRule(variant)) = get_variant(Cow::from(variant))
                        {
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

            if context.config.theme.screens.is_empty() {
                for (_, screen) in BUILTIN_SCREENS.iter() {
                    if !first_child {
                        write!(context.buffer, "\n\n")?;
                    } else {
                        first_child = false;
                    }

                    indent(indentation, context.buffer)?;
                    writeln!(context.buffer, "@media (min-width: {screen}) {{")?;

                    indentation += 1;
                    indent(indentation, context.buffer)?;
                    write!(context.buffer, ".")?;
                    context.selector.write_css_class(context.buffer)?;
                    writeln!(context.buffer, " {{")?;

                    indentation += 1;
                    indent(indentation, context.buffer)?;
                    writeln!(context.buffer, "max-width: {screen};")?;

                    indentation -= 1;
                    indent(indentation, context.buffer)?;
                    writeln!(context.buffer, "}}")?;

                    indentation -= 1;
                    indent(indentation, context.buffer)?;
                    write!(context.buffer, "}}")?;
                }
            } else {
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
                    let a =
                        if let Some(first_char_a) = a.1.chars().position(char::is_alphabetic) {
                            a.1[..first_char_a].parse::<usize>().ok()
                        } else {
                            a.1.parse::<usize>().ok()
                        };

                    let b =
                        if let Some(first_char_b) = b.1.chars().position(char::is_alphabetic) {
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

                for (_, screen) in screens.iter() {
                    if !first_child {
                        write!(context.buffer, "\n\n")?;
                    } else {
                        first_child = false;
                    }

                    indent(indentation, context.buffer)?;
                    writeln!(context.buffer, "@media (min-width: {screen}) {{")?;

                    indentation += 1;
                    indent(indentation, context.buffer)?;
                    write!(context.buffer, ".")?;
                    context.selector.write_css_class(context.buffer)?;
                    writeln!(context.buffer, " {{")?;

                    indentation += 1;
                    indent(indentation, context.buffer)?;
                    writeln!(context.buffer, "max-width: {screen};")?;

                    indentation -= 1;
                    indent(indentation, context.buffer)?;
                    writeln!(context.buffer, "}}")?;

                    indentation -= 1;
                    indent(indentation, context.buffer)?;
                    write!(context.buffer, "}}")?;
                }
            }

            // After rule
            for i in (1..indentation + 1).rev() {
                writeln!(context.buffer)?;
                indent(i - 1, context.buffer)?;
                write!(context.buffer, "}}")?;
            }
        }

        Ok(())
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        if let Modifier::Basic { .. } = context.modifier {
            writeln!(context.buffer, "width: 100%;")?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct ColumnsPlugin;

impl Plugin for ColumnsPlugin {
    fn namespace(&self) -> &'static str {
        "columns"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => value.parse::<usize>().is_ok(),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => writeln!(context.buffer, "columns: {value};")?,
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct BreakBeforePlugin;

impl Plugin for BreakBeforePlugin {
    fn namespace(&self) -> &'static str {
        "break-before"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => [
                "auto",
                "avoid",
                "all",
                "avoid-page",
                "page",
                "left",
                "right",
                "column",
            ]
            .contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => writeln!(context.buffer, "break-before: {value};")?,
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct BreakInsidePlugin;

impl Plugin for BreakInsidePlugin {
    fn namespace(&self) -> &'static str {
        "break-inside"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => {
                ["auto", "avoid", "avoid-page", "avoid-column"].contains(&&**value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => writeln!(context.buffer, "break-inside: {value};")?,
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct BreakAfterPlugin;

impl Plugin for BreakAfterPlugin {
    fn namespace(&self) -> &'static str {
        "break-after"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => [
                "auto",
                "avoid",
                "all",
                "avoid-page",
                "page",
                "left",
                "right",
                "column",
            ]
            .contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => writeln!(context.buffer, "break-after: {value};")?,
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct BoxDecorationBreakPlugin;

impl Plugin for BoxDecorationBreakPlugin {
    fn namespace(&self) -> &str {
        "decoration"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => ["slice", "clone"].contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => {
                writeln!(context.buffer, "box-decoration-break: {value};")?
            }
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct BoxSizingPlugin;

impl Plugin for BoxSizingPlugin {
    fn namespace(&self) -> &str {
        "box"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => ["border", "content"].contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => writeln!(context.buffer, "box-sizing: {value}-box;")?,
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct FloatPlugin;

impl Plugin for FloatPlugin {
    fn namespace(&self) -> &str {
        "float"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => ["left", "right", "none"].contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => writeln!(context.buffer, "float: {value};")?,
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct ClearPlugin;

impl Plugin for ClearPlugin {
    fn namespace(&self) -> &str {
        "clear"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => ["left", "right", "both", "none"].contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => writeln!(context.buffer, "clear: {value};")?,
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct ObjectFitPlugin;

impl Plugin for ObjectFitPlugin {
    fn namespace(&self) -> &str {
        "object"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => {
                ["contain", "cover", "fill", "scale-down", "none"].contains(&&**value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => writeln!(context.buffer, "object-fit: {value};")?,
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct ObjectPositionPlugin;

impl Plugin for ObjectPositionPlugin {
    fn namespace(&self) -> &str {
        "object"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => [
                "bottom",
                "center",
                "left",
                "left-bottom",
                "left-top",
                "right",
                "right-bottom",
                "right-top",
                "top",
            ]
            .contains(&&**value),
            Modifier::Arbitrary { value, .. } => value.split('_').all(is_matching_position),
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "bottom" => writeln!(context.buffer, "object-position: bottom;")?,
                "center" => writeln!(context.buffer, "object-position: center;")?,
                "left" => writeln!(context.buffer, "object-position: left;")?,
                "left-bottom" => writeln!(context.buffer, "object-position: left bottom;")?,
                "left-top" => writeln!(context.buffer, "object-position: left top;")?,
                "right" => writeln!(context.buffer, "object-position: right;")?,
                "right-bottom" => writeln!(context.buffer, "object-position: right bottom;")?,
                "right-top" => writeln!(context.buffer, "object-position: right top;")?,
                "top" => writeln!(context.buffer, "object-position: top;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "object-position: {};", to_css_value(value))?
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct OverflowPlugin;

impl Plugin for OverflowPlugin {
    fn namespace(&self) -> &str {
        "overflow"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => [
                "auto",
                "x-auto",
                "y-auto",
                "hidden",
                "x-hidden",
                "y-hidden",
                "visible",
                "x-visible",
                "y-visible",
                "scroll",
                "x-scroll",
                "y-scroll",
            ]
            .contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "auto" => writeln!(context.buffer, "overflow: auto;")?,
                "x-auto" => writeln!(context.buffer, "overflow-x: auto;")?,
                "y-auto" => writeln!(context.buffer, "overflow-y: auto;")?,
                "hidden" => writeln!(context.buffer, "overflow: hidden;")?,
                "x-hidden" => writeln!(context.buffer, "overflow-x: hidden;")?,
                "y-hidden" => writeln!(context.buffer, "overflow-y: hidden;")?,
                "visible" => writeln!(context.buffer, "overflow: visible;")?,
                "x-visible" => writeln!(context.buffer, "overflow-x: visible;")?,
                "y-visible" => writeln!(context.buffer, "overflow-y: visible;")?,
                "scroll" => writeln!(context.buffer, "overflow: scroll;")?,
                "x-scroll" => writeln!(context.buffer, "overflow-x: scroll;")?,
                "y-scroll" => writeln!(context.buffer, "overflow-y: scroll;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct OverscrollPlugin;

impl Plugin for OverscrollPlugin {
    fn namespace(&self) -> &str {
        "overscroll"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => [
                "auto",
                "x-auto",
                "y-auto",
                "contain",
                "x-contain",
                "y-contain",
                "none",
                "x-none",
                "y-none",
            ]
            .contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "auto" => writeln!(context.buffer, "overscroll-behavior: auto;")?,
                "x-auto" => writeln!(context.buffer, "overscroll-behavior-x: auto;")?,
                "y-auto" => writeln!(context.buffer, "overscroll-behavior-y: auto;")?,
                "contain" => writeln!(context.buffer, "overscroll-behavior: contain;")?,
                "x-contain" => writeln!(context.buffer, "overscroll-behavior-x: contain;")?,
                "y-contain" => writeln!(context.buffer, "overscroll-behavior-y: contain;")?,
                "none" => writeln!(context.buffer, "overscroll-behavior: none;")?,
                "x-none" => writeln!(context.buffer, "overscroll-behavior-x: none;")?,
                "y-none" => writeln!(context.buffer, "overscroll-behavior-y: none;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}
