use super::{to_css_value, Plugin};
use crate::{
    context::{ContextCanHandle, ContextHandle},
    selector::Modifier,
    utils::{color, indent, length, value_matchers::*},
};

use std::fmt::{self, Write};

#[derive(Debug)]
pub struct AccentColorPlugin;

impl Plugin for AccentColorPlugin {
    fn namespace(&self) -> &str {
        "accent"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => color::is_matching_basic_color(context.config, value),
            Modifier::Arbitrary { value, .. } => is_matching_color(value),
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        let value = match context.modifier {
            Modifier::Basic { value, .. } => color::get(context.config, value, None).unwrap(),
            Modifier::Arbitrary { value, .. } => to_css_value(*value),
        };

        writeln!(context.buffer, "accent-color: {value};")
    }
}

#[derive(Debug)]
pub struct AppearancePlugin;

impl Plugin for AppearancePlugin {
    fn namespace(&self) -> &str {
        "appearance"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => *value == "none",
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { .. } => {
                writeln!(context.buffer, "-webkit-appearance: none;")?;
                indent(context.indentation, context.buffer)?;
                writeln!(context.buffer, "-moz-appearance: none;")?;
                indent(context.indentation, context.buffer)?;
                writeln!(context.buffer, "appearance: none;")?;
            }
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn namespace(&self) -> &str {
        "cursor"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => [
                "auto",
                "default",
                "pointer",
                "wait",
                "text",
                "move",
                "help",
                "not-allowed",
                "none",
                "context-menu",
                "progress",
                "cell",
                "crosshair",
                "vertical-text",
                "alias",
                "copy",
                "no-drop",
                "grab",
                "grabbing",
                "all-scroll",
                "col-resize",
                "row-resize",
                "n-resize",
                "e-resize",
                "s-resize",
                "w-resize",
                "ne-resize",
                "nw-resize",
                "se-resize",
                "sw-resize",
                "ew-resize",
                "ns-resize",
                "nesw-resize",
                "nwse-resize",
                "zoom-in",
                "zoom-out",
            ]
            .contains(&&**value),
            Modifier::Arbitrary { value, .. } => is_matching_all(value), // TODO: Better matching
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => writeln!(context.buffer, "cursor: {value};")?,
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "cursor: {};", to_css_value(value))?
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct CaretColorPlugin;

impl Plugin for CaretColorPlugin {
    fn namespace(&self) -> &str {
        "caret"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => color::is_matching_basic_color(context.config, value),
            Modifier::Arbitrary { value, .. } => is_matching_color(value),
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        let value = match context.modifier {
            Modifier::Basic { value, .. } => color::get(context.config, value, None).unwrap(),
            Modifier::Arbitrary { value, .. } => to_css_value(*value),
        };

        writeln!(context.buffer, "caret-color: {value};")
    }
}

#[derive(Debug)]
pub struct PointerEventsPlugin;

impl Plugin for PointerEventsPlugin {
    fn namespace(&self) -> &str {
        "pointer-events"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => ["auto", "none"].contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => writeln!(context.buffer, "pointer-events: {value};")?,
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct ResizePlugin;

impl Plugin for ResizePlugin {
    fn namespace(&self) -> &str {
        "resize"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => ["", "x", "y", "none"].contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "" => writeln!(context.buffer, "resize: both;")?,
                "none" => writeln!(context.buffer, "resize: none;")?,
                "x" => writeln!(context.buffer, "resize: horizontal;")?,
                "y" => writeln!(context.buffer, "resize: vertical;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct ScrollBehaviorPlugin;

impl Plugin for ScrollBehaviorPlugin {
    fn namespace(&self) -> &str {
        "scroll"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => ["auto", "smooth"].contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => writeln!(context.buffer, "scroll-behavior: {value};")?,
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

// Scroll margin

fn scroll_margin_padding_can_handle(context: ContextCanHandle) -> bool {
    match context.modifier {
        Modifier::Basic { is_negative, value } => {
            length::get_basic(value, *is_negative).is_some() || *value == "auto"
        }
        Modifier::Arbitrary { value, .. } => is_matching_length(value),
    }
}

pub fn scroll_margin_padding_handle(
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
                    length::get_basic(value, *is_negative).unwrap(),
                )?;
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
pub struct ScrollMarginPlugin;

impl Plugin for ScrollMarginPlugin {
    fn namespace(&self) -> &str {
        "scroll-m"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        scroll_margin_padding_can_handle(context)
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        scroll_margin_padding_handle(&["scroll-margin"], context)
    }
}

#[derive(Debug)]
pub struct ScrollMarginXPlugin;

impl Plugin for ScrollMarginXPlugin {
    fn namespace(&self) -> &str {
        "scroll-mx"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        scroll_margin_padding_can_handle(context)
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        scroll_margin_padding_handle(&["scroll-margin-left", "scroll-margin-right"], context)
    }
}

#[derive(Debug)]
pub struct ScrollMarginYPlugin;

impl Plugin for ScrollMarginYPlugin {
    fn namespace(&self) -> &str {
        "scroll-my"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        scroll_margin_padding_can_handle(context)
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        scroll_margin_padding_handle(&["scroll-margin-top", "scroll-margin-bottom"], context)
    }
}

#[derive(Debug)]
pub struct ScrollMarginLeftPlugin;

impl Plugin for ScrollMarginLeftPlugin {
    fn namespace(&self) -> &str {
        "scroll-ml"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        scroll_margin_padding_can_handle(context)
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        scroll_margin_padding_handle(&["scroll-margin-left"], context)
    }
}

#[derive(Debug)]
pub struct ScrollMarginRightPlugin;

impl Plugin for ScrollMarginRightPlugin {
    fn namespace(&self) -> &str {
        "scroll-mr"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        scroll_margin_padding_can_handle(context)
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        scroll_margin_padding_handle(&["scroll-margin-right"], context)
    }
}

#[derive(Debug)]
pub struct ScrollMarginTopPlugin;

impl Plugin for ScrollMarginTopPlugin {
    fn namespace(&self) -> &str {
        "scroll-mt"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        scroll_margin_padding_can_handle(context)
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        scroll_margin_padding_handle(&["scroll-margin-top"], context)
    }
}

#[derive(Debug)]
pub struct ScrollMarginBottomPlugin;

impl Plugin for ScrollMarginBottomPlugin {
    fn namespace(&self) -> &str {
        "scroll-mb"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        scroll_margin_padding_can_handle(context)
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        scroll_margin_padding_handle(&["scroll-margin-bottom"], context)
    }
}

// Scroll padding

#[derive(Debug)]
pub struct ScrollPaddingPlugin;

impl Plugin for ScrollPaddingPlugin {
    fn namespace(&self) -> &str {
        "scroll-p"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        scroll_margin_padding_can_handle(context)
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        scroll_margin_padding_handle(&["scroll-padding"], context)
    }
}

#[derive(Debug)]
pub struct ScrollPaddingXPlugin;

impl Plugin for ScrollPaddingXPlugin {
    fn namespace(&self) -> &str {
        "scroll-px"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        scroll_margin_padding_can_handle(context)
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        scroll_margin_padding_handle(&["scroll-padding-left", "scroll-padding-right"], context)
    }
}

#[derive(Debug)]
pub struct ScrollPaddingYPlugin;

impl Plugin for ScrollPaddingYPlugin {
    fn namespace(&self) -> &str {
        "scroll-py"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        scroll_margin_padding_can_handle(context)
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        scroll_margin_padding_handle(&["scroll-padding-top", "scroll-padding-bottom"], context)
    }
}

#[derive(Debug)]
pub struct ScrollPaddingLeftPlugin;

impl Plugin for ScrollPaddingLeftPlugin {
    fn namespace(&self) -> &str {
        "scroll-pl"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        scroll_margin_padding_can_handle(context)
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        scroll_margin_padding_handle(&["scroll-padding-left"], context)
    }
}

#[derive(Debug)]
pub struct ScrollPaddingRightPlugin;

impl Plugin for ScrollPaddingRightPlugin {
    fn namespace(&self) -> &str {
        "scroll-pr"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        scroll_margin_padding_can_handle(context)
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        scroll_margin_padding_handle(&["scroll-padding-right"], context)
    }
}

#[derive(Debug)]
pub struct ScrollPaddingTopPlugin;

impl Plugin for ScrollPaddingTopPlugin {
    fn namespace(&self) -> &str {
        "scroll-pt"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        scroll_margin_padding_can_handle(context)
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        scroll_margin_padding_handle(&["scroll-padding-top"], context)
    }
}

#[derive(Debug)]
pub struct ScrollPaddingBottomPlugin;

impl Plugin for ScrollPaddingBottomPlugin {
    fn namespace(&self) -> &str {
        "scroll-pb"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        scroll_margin_padding_can_handle(context)
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        scroll_margin_padding_handle(&["scroll-padding-bottom"], context)
    }
}

#[derive(Debug)]
pub struct ScrollSnapAlignPlugin;

impl Plugin for ScrollSnapAlignPlugin {
    fn namespace(&self) -> &str {
        "snap"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => {
                ["start", "end", "center", "align-none"].contains(&&**value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "start" => writeln!(context.buffer, "scroll-snap-align: start;")?,
                "end" => writeln!(context.buffer, "scroll-snap-align: end;")?,
                "center" => writeln!(context.buffer, "scroll-snap-align: center;")?,
                "align-none" => writeln!(context.buffer, "scroll-snap-align: none;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct ScrollSnapStopPlugin;

impl Plugin for ScrollSnapStopPlugin {
    fn namespace(&self) -> &str {
        "snap"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => ["normal", "always"].contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "normal" => writeln!(context.buffer, "scroll-snap-stop: normal;")?,
                "always" => writeln!(context.buffer, "scroll-snap-stop: always;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct ScrollSnapTypePlugin;

impl Plugin for ScrollSnapTypePlugin {
    fn namespace(&self) -> &str {
        "snap"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => {
                ["x", "y", "both", "mandatory", "proximity", "none"].contains(&&**value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "none" => {
                    writeln!(context.buffer, "-ms-scroll-snap-type: none;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "scroll-snap-type: none;")?;
                }
                "x" => {
                    writeln!(
                        context.buffer,
                        "-ms-scroll-snap-type: x var(--en-scroll-snap-strictness);"
                    )?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(
                        context.buffer,
                        "scroll-snap-type: x var(--en-scroll-snap-strictness);"
                    )?;
                }
                "y" => {
                    writeln!(
                        context.buffer,
                        "-ms-scroll-snap-type: y var(--en-scroll-snap-strictness);"
                    )?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(
                        context.buffer,
                        "scroll-snap-type: y var(--en-scroll-snap-strictness);"
                    )?;
                }
                "both" => {
                    writeln!(
                        context.buffer,
                        "-ms-scroll-snap-type: both var(--en-scroll-snap-strictness);"
                    )?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(
                        context.buffer,
                        "scroll-snap-type: both var(--en-scroll-snap-strictness);"
                    )?;
                }
                "mandatory" => writeln!(context.buffer, "--en-scroll-snap-strictness: mandatory;")?,
                "proximity" => writeln!(context.buffer, "--en-scroll-snap-strictness: proximity;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct TouchActionPlugin;

impl Plugin for TouchActionPlugin {
    fn namespace(&self) -> &str {
        "touch"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => [
                "auto",
                "pan-x",
                "pan-left",
                "pan-right",
                "pan-y",
                "pan-up",
                "pan-down",
                "pinch-zoom",
                "manipulation",
                "none",
            ]
            .contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => writeln!(context.buffer, "touch-action: {value};")?,
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct UserSelectPlugin;

impl Plugin for UserSelectPlugin {
    fn namespace(&self) -> &str {
        "select"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => ["text", "all", "auto", "none"].contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => writeln!(context.buffer, "user-select: {value};")?,
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct WillChangePlugin;

impl Plugin for WillChangePlugin {
    fn namespace(&self) -> &str {
        "will-change"
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => {
                ["auto", "scroll", "contents", "transform"].contains(&&**value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "auto" => writeln!(context.buffer, "will-change: auto;")?,
                "scroll" => writeln!(context.buffer, "will-change: scroll-position;")?,
                "contents" => writeln!(context.buffer, "will-change: contents;")?,
                "transform" => writeln!(context.buffer, "will-change: transfrom;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => writeln!(context.buffer, "will-change: {value};")?,
        }

        Ok(())
    }
}
