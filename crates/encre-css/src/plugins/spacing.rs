use super::Plugin;
use crate::utils::{default_lengths, indent, value_matchers::*};
use crate::{config::Config, selector::Modifier};

use std::borrow::Cow;
use std::fmt::{self, Write};

pub fn margin_padding_can_handle(modifier: &Modifier) -> bool {
    match modifier {
        Modifier::Basic { is_negative, value } => {
            value == "auto" || default_lengths::get_basic(value, *is_negative).is_some()
        }
        Modifier::Arbitrary { hint, value } => hint == "length" || is_matching_length(value),
    }
}

pub fn margin_padding_handle(
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
                    if value == "auto" {
                        Cow::from("auto")
                    } else {
                        default_lengths::get_basic(value, *is_negative).unwrap()
                    }
                )?
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

// Margin

pub struct MarginPlugin;

impl Plugin for MarginPlugin {
    fn namespace(&self) -> &str {
        "m"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        margin_padding_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        margin_padding_handle(&["margin"], modifier, indentation, buffer)
    }
}

pub struct MarginXPlugin;

impl Plugin for MarginXPlugin {
    fn namespace(&self) -> &str {
        "mx"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        margin_padding_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        margin_padding_handle(
            &["margin-left", "margin-right"],
            modifier,
            indentation,
            buffer,
        )
    }
}

pub struct MarginYPlugin;

impl Plugin for MarginYPlugin {
    fn namespace(&self) -> &str {
        "my"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        margin_padding_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        margin_padding_handle(
            &["margin-top", "margin-bottom"],
            modifier,
            indentation,
            buffer,
        )
    }
}

pub struct MarginTopPlugin;

impl Plugin for MarginTopPlugin {
    fn namespace(&self) -> &str {
        "mt"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        margin_padding_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        margin_padding_handle(&["margin-top"], modifier, indentation, buffer)
    }
}

pub struct MarginBottomPlugin;

impl Plugin for MarginBottomPlugin {
    fn namespace(&self) -> &str {
        "mb"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        margin_padding_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        margin_padding_handle(&["margin-bottom"], modifier, indentation, buffer)
    }
}

pub struct MarginLeftPlugin;

impl Plugin for MarginLeftPlugin {
    fn namespace(&self) -> &str {
        "ml"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        margin_padding_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        margin_padding_handle(&["margin-left"], modifier, indentation, buffer)
    }
}

pub struct MarginRightPlugin;

impl Plugin for MarginRightPlugin {
    fn namespace(&self) -> &str {
        "mr"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        margin_padding_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        margin_padding_handle(&["margin-right"], modifier, indentation, buffer)
    }
}

// Padding

pub struct PaddingPlugin;

impl Plugin for PaddingPlugin {
    fn namespace(&self) -> &str {
        "p"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        margin_padding_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        margin_padding_handle(&["padding"], modifier, indentation, buffer)
    }
}

pub struct PaddingXPlugin;

impl Plugin for PaddingXPlugin {
    fn namespace(&self) -> &str {
        "px"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        margin_padding_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        margin_padding_handle(
            &["padding-left", "padding-right"],
            modifier,
            indentation,
            buffer,
        )
    }
}

pub struct PaddingYPlugin;

impl Plugin for PaddingYPlugin {
    fn namespace(&self) -> &str {
        "py"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        margin_padding_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        margin_padding_handle(
            &["padding-top", "padding-bottom"],
            modifier,
            indentation,
            buffer,
        )
    }
}

pub struct PaddingTopPlugin;

impl Plugin for PaddingTopPlugin {
    fn namespace(&self) -> &str {
        "pt"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        margin_padding_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        margin_padding_handle(&["padding-top"], modifier, indentation, buffer)
    }
}

pub struct PaddingBottomPlugin;

impl Plugin for PaddingBottomPlugin {
    fn namespace(&self) -> &str {
        "pb"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        margin_padding_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        margin_padding_handle(&["padding-bottom"], modifier, indentation, buffer)
    }
}

pub struct PaddingLeftPlugin;

impl Plugin for PaddingLeftPlugin {
    fn namespace(&self) -> &str {
        "pl"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        margin_padding_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        margin_padding_handle(&["padding-left"], modifier, indentation, buffer)
    }
}

pub struct PaddingRightPlugin;

impl Plugin for PaddingRightPlugin {
    fn namespace(&self) -> &str {
        "pr"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        margin_padding_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        margin_padding_handle(&["padding-right"], modifier, indentation, buffer)
    }
}

// Spacing

pub struct SpaceXPlugin;

impl Plugin for SpaceXPlugin {
    fn namespace(&self) -> &str {
        "space-x"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { is_negative, value } => {
                value == "reverse" || default_lengths::get_basic(value, *is_negative).is_some()
            }
            Modifier::Arbitrary { hint, value } => hint == "length" || is_matching_length(value),
        }
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        // TODO: class with `> :not([hidden]) ~ :not([hidden])`
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { is_negative, value } => {
                if value == "reverse" {
                    return writeln!(buffer, "--en-space-x-reverse: 1;");
                }

                let length = default_lengths::get_basic(value, *is_negative).unwrap();
                writeln!(buffer, "--en-space-x-reverse: 0;")?;
                indent(indentation, buffer)?;
                writeln!(
                    buffer,
                    "margin-left: calc({length} * calc(1 - var(--en-space-x-reverse)));"
                )?;
                indent(indentation, buffer)?;
                writeln!(
                    buffer,
                    "margin-right: calc({length} * var(--en-space-x-reverse));"
                )?;
            }
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "--en-space-x-reverse: 0;")?;
                indent(indentation, buffer)?;
                writeln!(
                    buffer,
                    "margin-left: calc({value} * calc(1 - var(--en-space-x-reverse)));"
                )?;
                indent(indentation, buffer)?;
                writeln!(
                    buffer,
                    "margin-right: calc({value} * var(--en-space-x-reverse));"
                )?;
            }
        }

        Ok(())
    }
}

pub struct SpaceYPlugin;

impl Plugin for SpaceYPlugin {
    fn namespace(&self) -> &str {
        "space-y"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { is_negative, value } => {
                value == "reverse" || default_lengths::get_basic(value, *is_negative).is_some()
            }
            Modifier::Arbitrary { hint, value } => hint == "length" || is_matching_length(value),
        }
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        // TODO: class with `> :not([hidden]) ~ :not([hidden])`
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { is_negative, value } => {
                if value == "reverse" {
                    return writeln!(buffer, "--en-space-y-reverse: 1;");
                }

                let length = default_lengths::get_basic(value, *is_negative).unwrap();
                writeln!(buffer, "--en-space-y-reverse: 0;")?;
                indent(indentation, buffer)?;
                writeln!(
                    buffer,
                    "margin-top: calc({length} * calc(1 - var(--en-space-y-reverse)));"
                )?;
                indent(indentation, buffer)?;
                writeln!(
                    buffer,
                    "margin-bottom: calc({length} * var(--en-space-y-reverse));"
                )?;
            }
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "--en-space-y-reverse: 0;")?;
                indent(indentation, buffer)?;
                writeln!(
                    buffer,
                    "margin-top: calc({value} * calc(1 - var(--en-space-y-reverse)));"
                )?;
                indent(indentation, buffer)?;
                writeln!(
                    buffer,
                    "margin-bottom: calc({value} * var(--en-space-y-reverse));"
                )?;
            }
        }

        Ok(())
    }
}
