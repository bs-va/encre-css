use super::{to_css_value, Plugin};
use crate::utils::{indent, length, value_matchers::*};
use crate::{config::Config, selector::Modifier};

use std::fmt::{self, Write};

#[derive(Debug)]
pub struct WidthPlugin;

impl Plugin for WidthPlugin {
    fn namespace(&self) -> &str {
        "w"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { is_negative, value } => {
                *value == "screen" || length::get_extended_size(value, *is_negative).is_some()
            }
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
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { is_negative, value } => {
                if *value == "screen" {
                    return writeln!(buffer, "width: 100vw;");
                }

                writeln!(
                    buffer,
                    "width: {};",
                    length::get_extended_size(value, *is_negative).unwrap()
                )?;
            }
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "width: {};", to_css_value(value))?
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct MinWidthPlugin;

impl Plugin for MinWidthPlugin {
    fn namespace(&self) -> &str {
        "min-w"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => ["0", "full", "min", "max", "fit"].contains(&&**value),
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
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { value, .. } => match *value {
                "0" => writeln!(buffer, "min-width: 0;")?,
                "full" => writeln!(buffer, "min-width: 100%;")?,
                "min" => writeln!(buffer, "min-width: min-content;")?,
                "max" => writeln!(buffer, "min-width: max-content;")?,
                "fit" => writeln!(buffer, "min-width: fit-content;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "min-width: {};", to_css_value(value))?
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct MaxWidthPlugin;

impl Plugin for MaxWidthPlugin {
    fn namespace(&self) -> &str {
        "max-w"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => [
                "0", "xs", "sm", "md", "lg", "xl", "2xl", "3xl", "4xl", "5xl", "6xl", "7xl",
                "full", "min", "max", "screen", "fit", "none",
            ]
            .contains(&&**value),
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
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { value, .. } => match *value {
                "0" => writeln!(buffer, "max-width: 0rem;")?,
                "none" => writeln!(buffer, "max-width: none;")?,
                "xs" => writeln!(buffer, "max-width: 20rem;")?,
                "sm" => writeln!(buffer, "max-width: 24rem;")?,
                "md" => writeln!(buffer, "max-width: 28rem;")?,
                "lg" => writeln!(buffer, "max-width: 32rem;")?,
                "xl" => writeln!(buffer, "max-width: 36rem;")?,
                "2xl" => writeln!(buffer, "max-width: 42rem;")?,
                "3xl" => writeln!(buffer, "max-width: 48rem;")?,
                "4xl" => writeln!(buffer, "max-width: 56rem;")?,
                "5xl" => writeln!(buffer, "max-width: 64rem;")?,
                "6xl" => writeln!(buffer, "max-width: 72rem;")?,
                "7xl" => writeln!(buffer, "max-width: 80rem;")?,
                "full" => writeln!(buffer, "max-width: 100%;")?,
                "min" => writeln!(buffer, "max-width: min-content;")?,
                "max" => writeln!(buffer, "max-width: max-content;")?,
                "fit" => writeln!(buffer, "max-width: fit-content;")?,
                "prose" => writeln!(buffer, "max-width: 65ch;")?,
                "screen-sm" => writeln!(buffer, "max-width: 640px;")?,
                "screen-md" => writeln!(buffer, "max-width: 768px;")?,
                "screen-lg" => writeln!(buffer, "max-width: 1024px;")?,
                "screen-xl" => writeln!(buffer, "max-width: 1280px;")?,
                "screen-2xl" => writeln!(buffer, "max-width: 1536px;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "max-width: {};", to_css_value(value))?
            }
        }

        Ok(())
    }
}

// Height

#[derive(Debug)]
pub struct HeightPlugin;

impl Plugin for HeightPlugin {
    fn namespace(&self) -> &str {
        "h"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { is_negative, value } => {
                *value == "screen" || length::get_extended_size(value, *is_negative).is_some()
            }
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
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { is_negative, value } => {
                if *value == "screen" {
                    return writeln!(buffer, "height: 100vh;");
                }

                writeln!(
                    buffer,
                    "height: {};",
                    length::get_extended_size(value, *is_negative).unwrap()
                )?;
            }
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "height: {};", to_css_value(value))?
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct MinHeightPlugin;

impl Plugin for MinHeightPlugin {
    fn namespace(&self) -> &str {
        "min-h"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => ["0", "full", "min", "max", "fit"].contains(&&**value),
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
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { value, .. } => match *value {
                "0" => writeln!(buffer, "min-height: 0;")?,
                "full" => writeln!(buffer, "min-height: 100%;")?,
                "min" => writeln!(buffer, "min-height: min-content;")?,
                "max" => writeln!(buffer, "min-height: max-content;")?,
                "fit" => writeln!(buffer, "min-height: fit-content;")?,
                "screen" => writeln!(buffer, "min-height: 100vh;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "min-width: {};", to_css_value(value))?
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct MaxHeightPlugin;

impl Plugin for MaxHeightPlugin {
    fn namespace(&self) -> &str {
        "max-h"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => [
                "0", "xs", "sm", "md", "lg", "xl", "2xl", "3xl", "4xl", "5xl", "6xl", "7xl",
                "full", "min", "max", "screen", "fit", "none",
            ]
            .contains(&&**value),
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
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { value, .. } => match *value {
                "0" => writeln!(buffer, "max-height: 0rem;")?,
                "none" => writeln!(buffer, "max-height: none;")?,
                "xs" => writeln!(buffer, "max-height: 20rem;")?,
                "sm" => writeln!(buffer, "max-height: 24rem;")?,
                "md" => writeln!(buffer, "max-height: 28rem;")?,
                "lg" => writeln!(buffer, "max-height: 32rem;")?,
                "xl" => writeln!(buffer, "max-height: 36rem;")?,
                "2xl" => writeln!(buffer, "max-height: 42rem;")?,
                "3xl" => writeln!(buffer, "max-height: 48rem;")?,
                "4xl" => writeln!(buffer, "max-height: 56rem;")?,
                "5xl" => writeln!(buffer, "max-height: 64rem;")?,
                "6xl" => writeln!(buffer, "max-height: 72rem;")?,
                "7xl" => writeln!(buffer, "max-height: 80rem;")?,
                "full" => writeln!(buffer, "max-height: 100%;")?,
                "min" => writeln!(buffer, "max-height: min-content;")?,
                "max" => writeln!(buffer, "max-height: max-content;")?,
                "screen" => writeln!(buffer, "max-height: 100vh;")?,
                "fit" => writeln!(buffer, "max-height: fit-content;")?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "max-height: {};", to_css_value(value))?
            }
        }

        Ok(())
    }
}
