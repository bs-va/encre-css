use super::{to_css_value, Plugin};
use crate::utils::{color, indent, value_matchers::*};
use crate::{config::Config, selector::Modifier};

use std::fmt::{self, Write};

const CSS_RING_OFFSET_SHADOW: &str = "--en-ring-offset-shadow: var(--en-ring-inset) 0 0 0 var(--en-ring-offset-width) var(--en-ring-offset-color);";

pub fn color_can_handle(config: &Config, modifier: &Modifier) -> bool {
    match modifier {
        Modifier::Basic { value, .. } => color::is_matching_basic_color(config, value),
        Modifier::Arbitrary { hint, value, .. } => {
            *hint == "color" || (hint.is_empty() && is_matching_color(value))
        }
    }
}

pub fn color_handle(
    config: &Config,
    css_props: &[&str],
    modifier: &Modifier,
    indentation: usize,
    buffer: &mut String,
) -> fmt::Result {
    indent(indentation, buffer)?;
    match modifier {
        Modifier::Basic { value, .. } => {
            let color = color::get(config, value, Some("--en-border-opacity")).unwrap();
            if color.contains("--en-border-opacity") {
                writeln!(buffer, "--en-border-opacity: 1;")?;
                indent(indentation, buffer)?;
            }

            for css_prop in css_props {
                writeln!(buffer, "{css_prop}: {color};")?;
            }
        }
        Modifier::Arbitrary { value, .. } => {
            let value = to_css_value(value);

            for css_prop in css_props {
                writeln!(buffer, "{css_prop}: {value};")?
            }
        }
    }

    Ok(())
}

#[derive(Debug)]
pub struct ColorPlugin;

impl Plugin for ColorPlugin {
    fn namespace(&self) -> &str {
        "border"
    }

    fn can_handle(&self, config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => color::is_matching_basic_color(config, value),
            Modifier::Arbitrary {
                prefix,
                hint,
                value,
                ..
            } => {
                prefix.is_empty()
                    && (*hint == "color" || (hint.is_empty() && is_matching_color(value)))
            }
        }
    }

    fn handle(
        &self,
        config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        color_handle(config, &["border-color"], modifier, indentation, buffer)
    }
}

#[derive(Debug)]
pub struct ColorXPlugin;

impl Plugin for ColorXPlugin {
    fn namespace(&self) -> &str {
        "border-x"
    }

    fn can_handle(&self, config: &Config, modifier: &Modifier) -> bool {
        color_can_handle(config, modifier)
    }

    fn handle(
        &self,
        config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        color_handle(
            config,
            &["border-left-color", "border-right-color"],
            modifier,
            indentation,
            buffer,
        )
    }
}

#[derive(Debug)]
pub struct ColorYPlugin;

impl Plugin for ColorYPlugin {
    fn namespace(&self) -> &str {
        "border-y"
    }

    fn can_handle(&self, config: &Config, modifier: &Modifier) -> bool {
        color_can_handle(config, modifier)
    }

    fn handle(
        &self,
        config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        color_handle(
            config,
            &["border-top-color", "border-bottom-color"],
            modifier,
            indentation,
            buffer,
        )
    }
}

#[derive(Debug)]
pub struct ColorLeftPlugin;

impl Plugin for ColorLeftPlugin {
    fn namespace(&self) -> &str {
        "border-l"
    }

    fn can_handle(&self, config: &Config, modifier: &Modifier) -> bool {
        color_can_handle(config, modifier)
    }

    fn handle(
        &self,
        config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        color_handle(
            config,
            &["border-left-color"],
            modifier,
            indentation,
            buffer,
        )
    }
}

#[derive(Debug)]
pub struct ColorRightPlugin;

impl Plugin for ColorRightPlugin {
    fn namespace(&self) -> &str {
        "border-r"
    }

    fn can_handle(&self, config: &Config, modifier: &Modifier) -> bool {
        color_can_handle(config, modifier)
    }

    fn handle(
        &self,
        config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        color_handle(
            config,
            &["border-right-color"],
            modifier,
            indentation,
            buffer,
        )
    }
}

#[derive(Debug)]
pub struct ColorTopPlugin;

impl Plugin for ColorTopPlugin {
    fn namespace(&self) -> &str {
        "border-t"
    }

    fn can_handle(&self, config: &Config, modifier: &Modifier) -> bool {
        color_can_handle(config, modifier)
    }

    fn handle(
        &self,
        config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        color_handle(config, &["border-top-color"], modifier, indentation, buffer)
    }
}

#[derive(Debug)]
pub struct ColorBottomPlugin;

impl Plugin for ColorBottomPlugin {
    fn namespace(&self) -> &str {
        "border-b"
    }

    fn can_handle(&self, config: &Config, modifier: &Modifier) -> bool {
        color_can_handle(config, modifier)
    }

    fn handle(
        &self,
        config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        color_handle(
            config,
            &["border-bottom-color"],
            modifier,
            indentation,
            buffer,
        )
    }
}

pub fn radius_can_handle(modifier: &Modifier) -> bool {
    match modifier {
        Modifier::Basic { value, .. } => {
            value.is_empty()
                || ["sm", "md", "lg", "xl", "2xl", "3xl", "full", "none"].contains(&&**value)
        }
        Modifier::Arbitrary { value, .. } => value
            .split('_')
            .all(|v| is_matching_length(v) || is_matching_percentage(v)),
    }
}

pub fn radius_handle(
    css_properties: &[&str],
    modifier: &Modifier,
    indentation: usize,
    buffer: &mut String,
) -> fmt::Result {
    match modifier {
        Modifier::Basic { value, .. } => {
            for css_prop in css_properties {
                indent(indentation, buffer)?;
                writeln!(
                    buffer,
                    "{}: {};",
                    css_prop,
                    match *value {
                        "" => "0.25rem",
                        "none" => "0",
                        "sm" => "0.125rem",
                        "md" => "0.375rem",
                        "lg" => "0.5rem",
                        "xl" => "0.75rem",
                        "2xl" => "1rem",
                        "3xl" => "1.5rem",
                        "full" => "9999px",
                        _ => unreachable!(),
                    }
                )?
            }
        }
        Modifier::Arbitrary { value, .. } => {
            for css_prop in css_properties {
                indent(indentation, buffer)?;
                writeln!(buffer, "{}: {};", css_prop, to_css_value(value))?;
            }
        }
    }

    Ok(())
}

#[derive(Debug)]
pub struct RadiusPlugin;

impl Plugin for RadiusPlugin {
    fn namespace(&self) -> &str {
        "rounded"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => {
                value.is_empty()
                    || ["sm", "md", "lg", "xl", "2xl", "3xl", "full", "none"].contains(&&**value)
            }
            Modifier::Arbitrary { prefix, value, .. } => {
                prefix.is_empty()
                    && value
                        .split('_')
                        .all(|v| is_matching_length(v) || is_matching_percentage(v))
            }
        }
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        radius_handle(&["border-radius"], modifier, indentation, buffer)
    }
}

#[derive(Debug)]
pub struct RadiusTopRightPlugin;

impl Plugin for RadiusTopRightPlugin {
    fn namespace(&self) -> &str {
        "rounded-tr"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        radius_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        radius_handle(&["border-top-right-radius"], modifier, indentation, buffer)
    }
}

#[derive(Debug)]
pub struct RadiusTopLeftPlugin;

impl Plugin for RadiusTopLeftPlugin {
    fn namespace(&self) -> &str {
        "rounded-tl"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        radius_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        radius_handle(&["border-top-left-radius"], modifier, indentation, buffer)
    }
}

#[derive(Debug)]
pub struct RadiusBottomRightPlugin;

impl Plugin for RadiusBottomRightPlugin {
    fn namespace(&self) -> &str {
        "rounded-br"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        radius_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        radius_handle(
            &["border-bottom-right-radius"],
            modifier,
            indentation,
            buffer,
        )
    }
}

#[derive(Debug)]
pub struct RadiusBottomLeftPlugin;

impl Plugin for RadiusBottomLeftPlugin {
    fn namespace(&self) -> &str {
        "rounded-bl"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        radius_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        radius_handle(
            &["border-bottom-left-radius"],
            modifier,
            indentation,
            buffer,
        )
    }
}

#[derive(Debug)]
pub struct RadiusTopPlugin;

impl Plugin for RadiusTopPlugin {
    fn namespace(&self) -> &str {
        "rounded-t"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        radius_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        radius_handle(
            &["border-top-left-radius", "border-top-right-radius"],
            modifier,
            indentation,
            buffer,
        )
    }
}

#[derive(Debug)]
pub struct RadiusBottomPlugin;

impl Plugin for RadiusBottomPlugin {
    fn namespace(&self) -> &str {
        "rounded-b"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        radius_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        radius_handle(
            &["border-bottom-left-radius", "border-bottom-right-radius"],
            modifier,
            indentation,
            buffer,
        )
    }
}

#[derive(Debug)]
pub struct RadiusLeftPlugin;

impl Plugin for RadiusLeftPlugin {
    fn namespace(&self) -> &str {
        "rounded-l"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        radius_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        radius_handle(
            &["border-top-left-radius", "border-bottom-left-radius"],
            modifier,
            indentation,
            buffer,
        )
    }
}

#[derive(Debug)]
pub struct RadiusRightPlugin;

impl Plugin for RadiusRightPlugin {
    fn namespace(&self) -> &str {
        "rounded-r"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        radius_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        radius_handle(
            &["border-top-right-radius", "border-bottom-right-radius"],
            modifier,
            indentation,
            buffer,
        )
    }
}

#[derive(Debug)]
pub struct StylePlugin;

impl Plugin for StylePlugin {
    fn namespace(&self) -> &str {
        "border"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => {
                ["solid", "dashed", "dotted", "double", "hidden", "none"].contains(value)
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
            Modifier::Basic { value, .. } => writeln!(buffer, "border-style: {};", value)?,
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

pub fn width_can_handle(modifier: &Modifier) -> bool {
    match modifier {
        Modifier::Basic { value, .. } => value.is_empty() || value.parse::<usize>().is_ok(),
        Modifier::Arbitrary { hint, value, .. } => {
            *hint == "length"
                || (hint.is_empty() && (is_matching_length(value) || is_matching_line_width(value)))
        }
    }
}

pub fn width_handle(
    css_properties: &[&str],
    modifier: &Modifier,
    indentation: usize,
    buffer: &mut String,
) -> fmt::Result {
    // NOTE: Not-compatible with TailwindCSS, support all values
    match modifier {
        Modifier::Basic { value, .. } => {
            for css_prop in css_properties {
                indent(indentation, buffer)?;
                writeln!(
                    buffer,
                    "{}: {}px;",
                    css_prop,
                    if value.is_empty() { "1" } else { value }
                )?;
            }
        }
        Modifier::Arbitrary { value, .. } => {
            let value = to_css_value(value);
            for css_prop in css_properties {
                indent(indentation, buffer)?;
                writeln!(buffer, "{}: {};", css_prop, value)?;
            }
        }
    }

    Ok(())
}

#[derive(Debug)]
pub struct WidthPlugin;

impl Plugin for WidthPlugin {
    fn namespace(&self) -> &str {
        "border"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => value.is_empty() || value.parse::<usize>().is_ok(),
            Modifier::Arbitrary {
                prefix,
                hint,
                value,
                ..
            } => {
                prefix.is_empty()
                    && (*hint == "length"
                        || (hint.is_empty()
                            && (is_matching_length(value) || is_matching_line_width(value))))
            }
        }
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        // NOTE: Not-compatible with TailwindCSS, support all values
        width_handle(&["border-width"], modifier, indentation, buffer)
    }
}

#[derive(Debug)]
pub struct WidthTopPlugin;

impl Plugin for WidthTopPlugin {
    fn namespace(&self) -> &str {
        "border-t"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        width_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        width_handle(&["border-top-width"], modifier, indentation, buffer)
    }
}

#[derive(Debug)]
pub struct WidthBottomPlugin;

impl Plugin for WidthBottomPlugin {
    fn namespace(&self) -> &str {
        "border-b"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        width_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        width_handle(&["border-bottom-width"], modifier, indentation, buffer)
    }
}

#[derive(Debug)]
pub struct WidthLeftPlugin;

impl Plugin for WidthLeftPlugin {
    fn namespace(&self) -> &str {
        "border-l"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        width_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        width_handle(&["border-left-width"], modifier, indentation, buffer)
    }
}

#[derive(Debug)]
pub struct WidthRightPlugin;

impl Plugin for WidthRightPlugin {
    fn namespace(&self) -> &str {
        "border-r"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        width_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        width_handle(&["border-right-width"], modifier, indentation, buffer)
    }
}

#[derive(Debug)]
pub struct WidthXPlugin;

impl Plugin for WidthXPlugin {
    fn namespace(&self) -> &str {
        "border-x"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        width_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        width_handle(
            &["border-left-width", "border-right-width"],
            modifier,
            indentation,
            buffer,
        )
    }
}

#[derive(Debug)]
pub struct WidthYPlugin;

impl Plugin for WidthYPlugin {
    fn namespace(&self) -> &str {
        "border-y"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        width_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        width_handle(
            &["border-top-width", "border-bottom-width"],
            modifier,
            indentation,
            buffer,
        )
    }
}

#[derive(Debug)]
pub struct OpacityPlugin;

impl Plugin for OpacityPlugin {
    fn namespace(&self) -> &str {
        "border-opacity"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => value.parse::<usize>().is_ok(),
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
        // NOTE: Not-compatible with TailwindCSS, support all values
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { value, .. } => writeln!(
                buffer,
                "--en-border-opacity: {};",
                value.parse::<usize>().unwrap() as f32 / 100.
            )?,
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct DivideColorPlugin;

impl Plugin for DivideColorPlugin {
    fn namespace(&self) -> &str {
        "divide"
    }

    fn can_handle(&self, config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => color::is_matching_basic_color(config, value),
            Modifier::Arbitrary { hint, value, .. } => {
                *hint == "color" || (hint.is_empty() && is_matching_color(value))
            }
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
        match modifier {
            Modifier::Basic { value, .. } => {
                let color = color::get(config, value, Some("--en-divide-opacity")).unwrap();
                if color.contains("--en-divide-opacity") {
                    writeln!(buffer, "--en-divide-opacity: 1;")?;
                    indent(indentation, buffer)?;
                }

                writeln!(buffer, "border-color: {color};")?;
            }
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "border-color: {};", to_css_value(value))?
            }
        }

        Ok(())
    }
}

pub fn divide_width_can_handle(modifier: &Modifier) -> bool {
    match modifier {
        Modifier::Basic { value, .. } => {
            value.is_empty() || *value == "reverse" || value.parse::<usize>().is_ok()
        }
        Modifier::Arbitrary { hint, value, .. } => {
            *hint == "length"
                || (hint.is_empty() && (is_matching_length(value) || is_matching_line_width(value)))
        }
    }
}

#[derive(Debug)]
pub struct DivideWidthXPlugin;

impl Plugin for DivideWidthXPlugin {
    fn namespace(&self) -> &str {
        "divide-x"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        divide_width_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        // NOTE: Not-compatible with TailwindCSS, support all values
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { value, .. } => {
                if *value == "reverse" {
                    return writeln!(buffer, "--en-divide-x-reverse: 1;");
                }

                writeln!(buffer, "--en-divide-x-reverse: 0;")?;
                indent(indentation, buffer)?;

                // TODO: class with `> :not([hidden]) ~ :not([hidden])`
                if is_matching_line_width(value) {
                    writeln!(buffer, "border-left-width: {value};")?;
                    indent(indentation, buffer)?;
                    writeln!(buffer, "border-right-width: {value};")?;
                } else {
                    writeln!(
                        buffer,
                        "border-left-width: calc({}px * calc(1 - var(--en-divide-x-reverse)));",
                        if value.is_empty() { "1" } else { value }
                    )?;
                    indent(indentation, buffer)?;
                    writeln!(
                        buffer,
                        "border-right-width: calc({}px * var(--en-divide-x-reverse));",
                        if value.is_empty() { "1" } else { value }
                    )?;
                }
            }
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "--en-divide-x-reverse: 0;")?;
                indent(indentation, buffer)?;

                if is_matching_line_width(value) {
                    let value = to_css_value(value);
                    writeln!(buffer, "border-left-width: {value};")?;
                    indent(indentation, buffer)?;
                    writeln!(buffer, "border-right-width: {value};")?;
                } else {
                    let value = to_css_value(value);
                    writeln!(
                        buffer,
                        "border-left-width: calc({value} * calc(1 - var(--en-divide-x-reverse)));"
                    )?;
                    indent(indentation, buffer)?;
                    writeln!(
                        buffer,
                        "border-right-width: calc({value} * var(--en-divide-x-reverse));"
                    )?;
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct DivideWidthYPlugin;

impl Plugin for DivideWidthYPlugin {
    fn namespace(&self) -> &str {
        "divide-y"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        divide_width_can_handle(modifier)
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        // NOTE: Not-compatible with TailwindCSS, support all values
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { value, .. } => {
                if *value == "reverse" {
                    return writeln!(buffer, "--en-divide-y-reverse: 1;");
                }

                writeln!(buffer, "--en-divide-y-reverse: 0;")?;
                indent(indentation, buffer)?;

                // TODO: class with `> :not([hidden]) ~ :not([hidden])`
                if is_matching_line_width(value) {
                    writeln!(buffer, "border-top-width: {value};")?;
                    indent(indentation, buffer)?;
                    writeln!(buffer, "border-bottom-width: {value};")?;
                } else {
                    writeln!(
                        buffer,
                        "border-top-width: calc({} * calc(1 - var(--en-divide-y-reverse)));",
                        if value.is_empty() { "1px" } else { value }
                    )?;
                    indent(indentation, buffer)?;
                    writeln!(
                        buffer,
                        "border-bottom-width: calc({} * var(--en-divide-y-reverse));",
                        if value.is_empty() { "1px" } else { value }
                    )?;
                }
            }
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "--en-divide-y-reverse: 0;")?;
                indent(indentation, buffer)?;

                if is_matching_line_width(value) {
                    let value = to_css_value(value);
                    writeln!(buffer, "border-top-width: {value};")?;
                    indent(indentation, buffer)?;
                    writeln!(buffer, "border-bottom-width: {value};")?;
                } else {
                    let value = to_css_value(value);
                    writeln!(
                        buffer,
                        "border-top-width: calc({value} * calc(1 - var(--en-divide-y-reverse)));"
                    )?;
                    indent(indentation, buffer)?;
                    writeln!(
                        buffer,
                        "border-bottom-width: calc({value} * var(--en-divide-y-reverse));"
                    )?;
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct DivideStylePlugin;

impl Plugin for DivideStylePlugin {
    fn namespace(&self) -> &str {
        "divide"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => {
                ["solid", "dashed", "dotted", "double", "none"].contains(value)
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
            Modifier::Basic { value, .. } => match *value {
                "solid" => writeln!(buffer, "border-style: solid;")?,
                "dashed" => writeln!(buffer, "border-style: dashed;")?,
                "dotted" => writeln!(buffer, "border-style: dotted;")?,
                "double" => writeln!(buffer, "border-style: double;")?,
                "none" => writeln!(buffer, "border-style: none;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct DivideOpacityPlugin;

impl Plugin for DivideOpacityPlugin {
    fn namespace(&self) -> &str {
        "divide-opacity"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => value.parse::<usize>().is_ok(),
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
        // NOTE: Not-compatible with TailwindCSS, support all values
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { value, .. } => writeln!(
                buffer,
                "--en-divide-opacity: {};",
                value.parse::<usize>().unwrap() as f32 / 100.
            )?,
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct RingColorPlugin;

impl Plugin for RingColorPlugin {
    fn namespace(&self) -> &str {
        "ring"
    }

    fn can_handle(&self, config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => color::is_matching_basic_color(config, value),
            Modifier::Arbitrary { hint, value, .. } => {
                *hint == "color" || (hint.is_empty() && is_matching_color(value))
            }
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
        match modifier {
            Modifier::Basic { value, .. } => {
                let color = color::get(config, value, Some("--en-ring-opacity")).unwrap();
                if color.contains("--en-ring-opacity") {
                    writeln!(buffer, "--en-ring-opacity: 1;")?;
                    indent(indentation, buffer)?;
                }

                writeln!(buffer, "--ring-color: {color};")?;
            }
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "--ring-color: {};", to_css_value(value))?
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct RingWidthPlugin;

impl Plugin for RingWidthPlugin {
    fn namespace(&self) -> &str {
        "ring"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => {
                value.is_empty() || *value == "inset" || value.parse::<usize>().is_ok()
            }
            Modifier::Arbitrary { hint, value, .. } => {
                *hint == "length" || (hint.is_empty() && is_matching_length(value))
            }
        }
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        // NOTE: Not-compatible with TailwindCSS, support all values
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { value, .. } => {
                if *value == "inset" {
                    return writeln!(buffer, "--en-ring-inset: inset;");
                }

                writeln!(buffer, "--en-ring-shadow: var(--en-ring-inset) 0 0 0 calc({}px + var(--en-ring-offset-width)) var(--en-ring-color);", if value.is_empty() { "3px" } else { value })?;
            }
            Modifier::Arbitrary { value, .. } => writeln!(buffer, "--en-ring-shadow: var(--en-ring-inset) 0 0 0 calc({} + var(--en-ring-offset-width)) var(--en-ring-color);", to_css_value(value))?,
        }

        indent(indentation, buffer)?;
        writeln!(buffer, "box-shadow: var(--en-ring-offset-shadow), var(--en-ring-shadow), var(--en-shadow, 0 0 #0000);")?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct RingOpacityPlugin;

impl Plugin for RingOpacityPlugin {
    fn namespace(&self) -> &str {
        "ring-opacity"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => value.parse::<usize>().is_ok(),
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
        // NOTE: Not-compatible with TailwindCSS, support all values
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { value, .. } => writeln!(
                buffer,
                "--en-ring-opacity: {};",
                value.parse::<usize>().unwrap() as f32 / 100.
            )?,
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct RingOffsetColorPlugin;

impl Plugin for RingOffsetColorPlugin {
    fn namespace(&self) -> &str {
        "ring-offset"
    }

    fn can_handle(&self, config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => color::is_matching_basic_color(config, value),
            Modifier::Arbitrary { hint, value, .. } => {
                *hint == "color" || (hint.is_empty() && is_matching_color(value))
            }
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
        writeln!(buffer, "{}", CSS_RING_OFFSET_SHADOW)?;

        indent(indentation, buffer)?;
        let value = match modifier {
            Modifier::Basic { value, .. } => color::get(config, value, None).unwrap(),
            Modifier::Arbitrary { value, .. } => to_css_value(*value),
        };

        writeln!(buffer, "--en-ring-offset-color: {value};")
    }
}

#[derive(Debug)]
pub struct RingOffsetWidthPlugin;

impl Plugin for RingOffsetWidthPlugin {
    fn namespace(&self) -> &str {
        "ring-offset"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => value.parse::<usize>().is_ok(),
            Modifier::Arbitrary { hint, value, .. } => {
                *hint == "length" || (hint.is_empty() && is_matching_length(value))
            }
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
        writeln!(buffer, "{}", CSS_RING_OFFSET_SHADOW)?;
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { value, .. } => {
                writeln!(buffer, "--en-ring-offset-width: {value}px;")?
            }
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "--en-ring-offset-width: {};", to_css_value(value))?
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct OutlineColorPlugin;

impl Plugin for OutlineColorPlugin {
    fn namespace(&self) -> &str {
        "outline"
    }
    fn can_handle(&self, config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => color::is_matching_basic_color(config, value),
            Modifier::Arbitrary { hint, value, .. } => {
                *hint == "color" || (hint.is_empty() && is_matching_color(value))
            }
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
            Modifier::Basic { value, .. } => color::get(config, value, None).unwrap(),
            Modifier::Arbitrary { value, .. } => to_css_value(*value),
        };

        writeln!(buffer, "outline-color: {value};")
    }
}

#[derive(Debug)]
pub struct OutlineWidthPlugin;

impl Plugin for OutlineWidthPlugin {
    fn namespace(&self) -> &str {
        "outline"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => value.parse::<usize>().is_ok(),
            Modifier::Arbitrary { hint, value, .. } => {
                *hint == "length" || (hint.is_empty() && is_matching_length(value))
            }
        }
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        // NOTE: Not-compatible with TailwindCSS, support all values
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { value, .. } => writeln!(buffer, "outline-width: {value}px;")?,
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "outline-width: {};", to_css_value(value))?
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct OutlineStylePlugin;

impl Plugin for OutlineStylePlugin {
    fn namespace(&self) -> &str {
        "outline"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => {
                ["", "dashed", "dotted", "double", "hidden", "none"].contains(&&**value)
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
            Modifier::Basic { value, .. } => match *value {
                "" => writeln!(buffer, "outline-style: solid;")?,
                "none" => {
                    writeln!(buffer, "outline: 2px solid transparent;")?;
                    indent(indentation, buffer)?;
                    writeln!(buffer, "outline-offset: 2px;")?;
                }
                "dashed" => writeln!(buffer, "outline-style: dashed;")?,
                "dotted" => writeln!(buffer, "outline-style: dotted;")?,
                "double" => writeln!(buffer, "outline-style: double;")?,
                "hidden" => writeln!(buffer, "outline-style: hidden;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { .. } => unreachable!(),
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct OutlineOffsetPlugin;

impl Plugin for OutlineOffsetPlugin {
    fn namespace(&self) -> &str {
        "outline-offset"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => value.parse::<usize>().is_ok(),
            Modifier::Arbitrary { value, .. } => is_matching_length(value),
        }
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        // NOTE: Not-compatible with TailwindCSS, support all values
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { value, .. } => writeln!(buffer, "outline-offset: {value}px;")?,
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "outline-offset: {};", to_css_value(value))?
            }
        }

        Ok(())
    }
}
