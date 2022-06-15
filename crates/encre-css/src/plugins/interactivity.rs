use super::Plugin;
use crate::utils::{default_colors, default_lengths, indent, value_matchers::*};
use crate::{config::Config, selector::Modifier};

use std::{
    borrow::Cow,
    fmt::{self, Write},
};

pub struct AccentColorPlugin;

impl Plugin for AccentColorPlugin {
    fn namespace(&self) -> &str {
        "accent"
    }

    fn can_handle(&self, config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => default_colors::get(config, value, None).is_some(),
            Modifier::Arbitrary { hint, value } => hint == "color" || is_matching_color(value),
        }
    }

    fn handle(
        &self,
        config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        indent(indentation, buffer)?;
        let value = match modifier {
            Modifier::Basic { value, .. } => default_colors::get(config, value, None).unwrap(),
            Modifier::Arbitrary { value, .. } => Cow::from(&**value),
        };

        writeln!(buffer, "accent-color: {value};")
    }
}

pub struct AppearancePlugin;

impl Plugin for AppearancePlugin {
    fn namespace(&self) -> &str {
        "appearance"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => value == "none",
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { .. } => {
                writeln!(buffer, "-webkit-appearance: none;")?;
                indent(indentation, buffer)?;
                writeln!(buffer, "-moz-appearance: none;")?;
                indent(indentation, buffer)?;
                writeln!(buffer, "appearance: none;")?;
            }
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn namespace(&self) -> &str {
        "cursor"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
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

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { value, .. } => writeln!(buffer, "cursor: {value};")?,
            Modifier::Arbitrary { value, .. } => writeln!(buffer, "cursor: {value};")?,
        }

        Ok(())
    }
}

pub struct CaretColorPlugin;

impl Plugin for CaretColorPlugin {
    fn namespace(&self) -> &str {
        "caret"
    }

    fn can_handle(&self, config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => default_colors::get(config, value, None).is_some(),
            Modifier::Arbitrary { hint, value } => hint == "color" || is_matching_color(value),
        }
    }

    fn handle(
        &self,
        config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        indent(indentation, buffer)?;
        let value = match modifier {
            Modifier::Basic { value, .. } => default_colors::get(config, value, None).unwrap(),
            Modifier::Arbitrary { value, .. } => Cow::from(&**value),
        };

        writeln!(buffer, "caret-color: {value};")
    }
}

pub struct PointerEventsPlugin;

impl Plugin for PointerEventsPlugin {
    fn namespace(&self) -> &str {
        "pointer-events"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => ["auto", "none"].contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { value, .. } => writeln!(buffer, "pointer-events: {value};")?,
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

pub struct ResizePlugin;

impl Plugin for ResizePlugin {
    fn namespace(&self) -> &str {
        "resize"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => ["", "x", "y", "none"].contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { value, .. } => match value.as_str() {
                "" => writeln!(buffer, "resize: both;")?,
                "none" => writeln!(buffer, "resize: none;")?,
                "x" => writeln!(buffer, "resize: horizontal;")?,
                "y" => writeln!(buffer, "resize: vertical;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

pub struct ScrollBehaviorPlugin;

impl Plugin for ScrollBehaviorPlugin {
    fn namespace(&self) -> &str {
        "scroll"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => ["auto", "smooth"].contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { value, .. } => writeln!(buffer, "scroll-behavior: {value};")?,
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

// Scroll margin

fn scroll_margin_padding_can_handle(modifier: &Modifier) -> bool {
    match modifier {
        Modifier::Basic { is_negative, value } => {
            default_lengths::get_basic(value, *is_negative).is_some() || value == "auto"
        }
        Modifier::Arbitrary { hint, value } => hint == "length" || is_matching_length(value),
    }
}

pub fn scroll_margin_padding_handle(
    css_properties: &[&str],
    modifier: &Modifier,
    indentation: usize,
    buffer: &mut String,
) -> fmt::Result {
    match modifier {
        Modifier::Basic { is_negative, value } => {
            for css_prop in css_properties {
                indent(indentation, buffer)?;
                writeln!(
                    buffer,
                    "{}: {};",
                    css_prop,
                    default_lengths::get_basic(value, *is_negative).unwrap(),
                )?;
            }
        }
        Modifier::Arbitrary { value, .. } => {
            for css_prop in css_properties {
                indent(indentation, buffer)?;
                writeln!(buffer, "{}: {};", css_prop, value)?;
            }
        }
    }

    Ok(())
}

pub struct ScrollMarginPlugin;

impl Plugin for ScrollMarginPlugin {
    fn namespace(&self) -> &str {
        "scroll-m"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        scroll_margin_padding_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        scroll_margin_padding_handle(&["scroll-margin"], modifier, indentation, buffer)
    }
}

pub struct ScrollMarginXPlugin;

impl Plugin for ScrollMarginXPlugin {
    fn namespace(&self) -> &str {
        "scroll-mx"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        scroll_margin_padding_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        scroll_margin_padding_handle(
            &["scroll-margin-left", "scroll-margin-right"],
            modifier,
            indentation,
            buffer,
        )
    }
}

pub struct ScrollMarginYPlugin;

impl Plugin for ScrollMarginYPlugin {
    fn namespace(&self) -> &str {
        "scroll-my"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        scroll_margin_padding_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        scroll_margin_padding_handle(
            &["scroll-margin-top", "scroll-margin-bottom"],
            modifier,
            indentation,
            buffer,
        )
    }
}

pub struct ScrollMarginLeftPlugin;

impl Plugin for ScrollMarginLeftPlugin {
    fn namespace(&self) -> &str {
        "scroll-ml"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        scroll_margin_padding_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        scroll_margin_padding_handle(&["scroll-margin-left"], modifier, indentation, buffer)
    }
}

pub struct ScrollMarginRightPlugin;

impl Plugin for ScrollMarginRightPlugin {
    fn namespace(&self) -> &str {
        "scroll-mr"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        scroll_margin_padding_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        scroll_margin_padding_handle(&["scroll-margin-right"], modifier, indentation, buffer)
    }
}

pub struct ScrollMarginTopPlugin;

impl Plugin for ScrollMarginTopPlugin {
    fn namespace(&self) -> &str {
        "scroll-mt"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        scroll_margin_padding_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        scroll_margin_padding_handle(&["scroll-margin-top"], modifier, indentation, buffer)
    }
}

pub struct ScrollMarginBottomPlugin;

impl Plugin for ScrollMarginBottomPlugin {
    fn namespace(&self) -> &str {
        "scroll-mb"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        scroll_margin_padding_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        scroll_margin_padding_handle(&["scroll-margin-bottom"], modifier, indentation, buffer)
    }
}

// Scroll padding

pub struct ScrollPaddingPlugin;

impl Plugin for ScrollPaddingPlugin {
    fn namespace(&self) -> &str {
        "scroll-p"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        scroll_margin_padding_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        scroll_margin_padding_handle(&["scroll-padding"], modifier, indentation, buffer)
    }
}

pub struct ScrollPaddingXPlugin;

impl Plugin for ScrollPaddingXPlugin {
    fn namespace(&self) -> &str {
        "scroll-px"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        scroll_margin_padding_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        scroll_margin_padding_handle(
            &["scroll-padding-left", "scroll-padding-right"],
            modifier,
            indentation,
            buffer,
        )
    }
}

pub struct ScrollPaddingYPlugin;

impl Plugin for ScrollPaddingYPlugin {
    fn namespace(&self) -> &str {
        "scroll-py"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        scroll_margin_padding_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        scroll_margin_padding_handle(
            &["scroll-padding-top", "scroll-padding-bottom"],
            modifier,
            indentation,
            buffer,
        )
    }
}

pub struct ScrollPaddingLeftPlugin;

impl Plugin for ScrollPaddingLeftPlugin {
    fn namespace(&self) -> &str {
        "scroll-pl"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        scroll_margin_padding_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        scroll_margin_padding_handle(&["scroll-padding-left"], modifier, indentation, buffer)
    }
}

pub struct ScrollPaddingRightPlugin;

impl Plugin for ScrollPaddingRightPlugin {
    fn namespace(&self) -> &str {
        "scroll-pr"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        scroll_margin_padding_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        scroll_margin_padding_handle(&["scroll-padding-right"], modifier, indentation, buffer)
    }
}

pub struct ScrollPaddingTopPlugin;

impl Plugin for ScrollPaddingTopPlugin {
    fn namespace(&self) -> &str {
        "scroll-pt"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        scroll_margin_padding_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        scroll_margin_padding_handle(&["scroll-padding-top"], modifier, indentation, buffer)
    }
}

pub struct ScrollPaddingBottomPlugin;

impl Plugin for ScrollPaddingBottomPlugin {
    fn namespace(&self) -> &str {
        "scroll-pb"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        scroll_margin_padding_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        scroll_margin_padding_handle(&["scroll-padding-bottom"], modifier, indentation, buffer)
    }
}

pub struct ScrollSnapAlignPlugin;

impl Plugin for ScrollSnapAlignPlugin {
    fn namespace(&self) -> &str {
        "snap"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => {
                ["start", "end", "center", "align-none"].contains(&&**value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { value, .. } => match value.as_str() {
                "start" => writeln!(buffer, "scroll-snap-align: start;")?,
                "end" => writeln!(buffer, "scroll-snap-align: end;")?,
                "center" => writeln!(buffer, "scroll-snap-align: center;")?,
                "align-none" => writeln!(buffer, "scroll-snap-align: none;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

pub struct ScrollSnapStopPlugin;

impl Plugin for ScrollSnapStopPlugin {
    fn namespace(&self) -> &str {
        "snap"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => ["normal", "always"].contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { value, .. } => match value.as_str() {
                "normal" => writeln!(buffer, "scroll-snap-stop: normal;")?,
                "always" => writeln!(buffer, "scroll-snap-stop: always;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

pub struct ScrollSnapTypePlugin;

impl Plugin for ScrollSnapTypePlugin {
    fn namespace(&self) -> &str {
        "snap"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => {
                ["x", "y", "both", "mandatory", "proximity", "none"].contains(&&**value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { value, .. } => match value.as_str() {
                "none" => {
                    writeln!(buffer, "-ms-scroll-snap-type: none;")?;
                    indent(indentation, buffer)?;
                    writeln!(buffer, "scroll-snap-type: none;")?;
                }
                "x" => {
                    writeln!(
                        buffer,
                        "-ms-scroll-snap-type: x var(--en-scroll-snap-strictness);"
                    )?;
                    indent(indentation, buffer)?;
                    writeln!(
                        buffer,
                        "scroll-snap-type: x var(--en-scroll-snap-strictness);"
                    )?;
                }
                "y" => {
                    writeln!(
                        buffer,
                        "-ms-scroll-snap-type: y var(--en-scroll-snap-strictness);"
                    )?;
                    indent(indentation, buffer)?;
                    writeln!(
                        buffer,
                        "scroll-snap-type: y var(--en-scroll-snap-strictness);"
                    )?;
                }
                "both" => {
                    writeln!(
                        buffer,
                        "-ms-scroll-snap-type: both var(--en-scroll-snap-strictness);"
                    )?;
                    indent(indentation, buffer)?;
                    writeln!(
                        buffer,
                        "scroll-snap-type: both var(--en-scroll-snap-strictness);"
                    )?;
                }
                "mandatory" => writeln!(buffer, "--en-scroll-snap-strictness: mandatory;")?,
                "proximity" => writeln!(buffer, "--en-scroll-snap-strictness: proximity;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

pub struct TouchActionPlugin;

impl Plugin for TouchActionPlugin {
    fn namespace(&self) -> &str {
        "touch"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
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

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { value, .. } => writeln!(buffer, "touch-action: {value};")?,
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

pub struct UserSelectPlugin;

impl Plugin for UserSelectPlugin {
    fn namespace(&self) -> &str {
        "select"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => ["text", "all", "auto", "none"].contains(&&**value),
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { value, .. } => writeln!(buffer, "user-select: {value};")?,
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

pub struct WillChangePlugin;

impl Plugin for WillChangePlugin {
    fn namespace(&self) -> &str {
        "will-change"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => {
                ["auto", "scroll", "contents", "transform"].contains(&&**value)
            }
            Modifier::Arbitrary { .. } => false,
        }
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { value, .. } => match value.as_str() {
                "auto" => writeln!(buffer, "will-change: auto;")?,
                "scroll" => writeln!(buffer, "will-change: scroll-position;")?,
                "contents" => writeln!(buffer, "will-change: contents;")?,
                "transform" => writeln!(buffer, "will-change: transfrom;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => writeln!(buffer, "will-change: {value};")?,
        }

        Ok(())
    }
}
