use super::Plugin;
use crate::utils::{indent, value_matchers::*};
use crate::{config::Config, selector::Modifier};

use lazy_static::lazy_static;
use regex::Regex;
use std::{
    fmt::{self, Write},
    sync::atomic::{AtomicBool, Ordering},
};

lazy_static! {
    static ref PROPERTY_REGEX: Regex = Regex::new(r"[^\d]+").unwrap();
}

pub struct PropertyPlugin;

impl Plugin for PropertyPlugin {
    fn namespace(&self) -> &str {
        "transition"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => [
                "",
                "none",
                "all",
                "colors",
                "opacity",
                "shadow",
                "transform",
            ]
            .contains(&&**value),
            Modifier::Arbitrary { value, .. } => PROPERTY_REGEX.is_match(value),
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
                "" => {
                    writeln!(buffer, "transition-property: color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter;")?;
                    indent(indentation, buffer)?;
                    writeln!(
                        buffer,
                        "transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);"
                    )?;
                    indent(indentation, buffer)?;
                    writeln!(buffer, "transition-duration: 150ms;")?;
                }
                "none" => writeln!(buffer, "transition-property: none;")?,
                "all" => {
                    writeln!(buffer, "transition-property: all;")?;
                    indent(indentation, buffer)?;
                    writeln!(
                        buffer,
                        "transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);"
                    )?;
                    indent(indentation, buffer)?;
                    writeln!(buffer, "transition-duration: 150ms;")?;
                }
                "colors" => {
                    writeln!(buffer, "transition-property: color, background-color, border-color, text-decoration-color, fill, stroke;")?;
                    indent(indentation, buffer)?;
                    writeln!(
                        buffer,
                        "transition-timing-function: cubic-bezier(0.4, 0, 0);"
                    )?;
                    indent(indentation, buffer)?;
                    writeln!(buffer, "transition-duration: 150ms;")?;
                }
                "opacity" => {
                    writeln!(buffer, "transition-property: opacity;")?;
                    indent(indentation, buffer)?;
                    writeln!(
                        buffer,
                        "transition-timing-function: cubic-bezier(0.4, 0, 0);"
                    )?;
                    indent(indentation, buffer)?;
                    writeln!(buffer, "transition-duration: 150ms;")?;
                }
                "shadow" => {
                    writeln!(buffer, "transition-property: box-shadow;")?;
                    indent(indentation, buffer)?;
                    writeln!(
                        buffer,
                        "transition-timing-function: cubic-bezier(0.4, 0, 0);"
                    )?;
                    indent(indentation, buffer)?;
                    writeln!(buffer, "transition-duration: 150ms;")?;
                }
                "transform" => {
                    writeln!(buffer, "transition-property: transform;")?;
                    indent(indentation, buffer)?;
                    writeln!(
                        buffer,
                        "transition-timing-function: cubic-bezier(0.4, 0, 0);"
                    )?;
                    indent(indentation, buffer)?;
                    writeln!(buffer, "transition-duration: 150ms;")?;
                }
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => writeln!(buffer, "transition-property: {value};")?,
        }

        Ok(())
    }
}

pub struct DurationPlugin;

impl Plugin for DurationPlugin {
    fn namespace(&self) -> &str {
        "duration"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => value.parse::<usize>().is_ok(),
            Modifier::Arbitrary { value, .. } => is_matching_time(value),
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
            Modifier::Basic { value, .. } => writeln!(buffer, "transition-duration: {value}ms;")?,
            Modifier::Arbitrary { value, .. } => writeln!(buffer, "transition-duration: {value};")?,
        }

        Ok(())
    }
}

pub struct DelayPlugin;

impl Plugin for DelayPlugin {
    fn namespace(&self) -> &str {
        "delay"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => value.parse::<usize>().is_ok(),
            Modifier::Arbitrary { value, .. } => is_matching_time(value),
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
            Modifier::Basic { value, .. } => writeln!(buffer, "transition-delay: {value}ms;")?,
            Modifier::Arbitrary { value, .. } => writeln!(buffer, "transition-delay: {value};")?,
        }

        Ok(())
    }
}

pub struct EasePlugin;

impl Plugin for EasePlugin {
    fn namespace(&self) -> &str {
        "ease"
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => value.parse::<usize>().is_ok(),
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
            Modifier::Basic { value, .. } => match value.as_str() {
                "linear" => writeln!(buffer, "transition-timing-function: linear;")?,
                "in" => writeln!(
                    buffer,
                    "transition-timing-function: cubic-bezier(0.4, 0, 1, 1);"
                )?,
                "out" => writeln!(
                    buffer,
                    "transition-timing-function: cubic-bezier(0, 0, 0.2, 1);"
                )?,
                "in-out" => writeln!(
                    buffer,
                    "transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);"
                )?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "transition-timing-function: {value};")?
            }
        }

        Ok(())
    }
}

pub struct AnimatePlugin {
    is_spin_animation_already_defined: AtomicBool,
    is_ping_animation_already_defined: AtomicBool,
    is_pulse_animation_already_defined: AtomicBool,
    is_bounce_animation_already_defined: AtomicBool,
}

impl AnimatePlugin {
    pub fn new() -> Self {
        Self {
            is_spin_animation_already_defined: AtomicBool::new(false),
            is_ping_animation_already_defined: AtomicBool::new(false),
            is_pulse_animation_already_defined: AtomicBool::new(false),
            is_bounce_animation_already_defined: AtomicBool::new(false),
        }
    }
}

impl Default for AnimatePlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for AnimatePlugin {
    fn namespace(&self) -> &str {
        "animate"
    }

    fn css_before_rule(&self, modifier: &Modifier, buffer: &mut String) -> fmt::Result {
        match modifier {
            Modifier::Basic { value, .. } => {
                match value.as_str() {
                    "spin" => {
                        if !self
                            .is_spin_animation_already_defined
                            .swap(true, Ordering::Relaxed)
                        {
                            writeln!(
                                buffer,
                                "@-webkit-keyframes spin {{
  to {{
    transform: rotate(360deg);
  }}
}}

@keyframes spin {{
  from {{
    transform: rotate(0deg);
  }}
  to {{
    transform: rotate(360deg);
  }}
}}\n"
                            )?;
                        }
                    }
                    "ping" => {
                        if !self
                            .is_ping_animation_already_defined
                            .swap(true, Ordering::Relaxed)
                        {
                            writeln!(
                                buffer,
                                "@-webkit-keyframes ping {{
  75%, 100% {{
    transform: scale(2);
    opacity: 0;
  }}
}}

@keyframes ping {{
  75%, 100% {{
    transform: scale(2);
    opacity: 0;
  }}
}}\n"
                            )?;
                        }
                    }
                    "pulse" => {
                        if !self
                            .is_pulse_animation_already_defined
                            .swap(true, Ordering::Relaxed)
                        {
                            writeln!(
                                buffer,
                                "@-webkit-keyframes pulse {{
  50% {{
    opacity: .5;
  }}
}}

@keyframes pulse {{
  0%, 100% {{
    opacity: 1;
  }}
  50% {{
    opacity: .5;
  }}
}}\n"
                            )?;
                        }
                    }
                    "bounce" => {
                        if !self
                            .is_bounce_animation_already_defined
                            .swap(true, Ordering::Relaxed)
                        {
                            writeln!(
                                buffer,
                                "@-webkit-keyframes bounce {{
  0%, 100% {{
    transform: translateY(-25%);
    -webkit-animation-timing-function: cubic-bezier(0.8,0,1,1);
    animation-timing-function: cubic-bezier(0.8,0,1,1);
  }}

  50% {{
    transform: none;
    -webkit-animation-timing-function: cubic-bezier(0,0,0.2,1);
    animation-timing-function: cubic-bezier(0,0,0.2,1);
  }}
}}

@keyframes bounce {{
  0%, 100% {{
    transform: translateY(-25%);
    -webkit-animation-timing-function: cubic-bezier(0.8,0,1,1);
    animation-timing-function: cubic-bezier(0.8, 0, 1, 1);
  }}
  50% {{
    transform: translateY(0);
    -webkit-animation-timing-function: cubic-bezier(0,0,0.2,1);
    animation-timing-function: cubic-bezier(0, 0, 0.2, 1);
  }}
}}\n"
                            )?;
                        }
                    }
                    _ => unreachable!(),
                };
            }
            Modifier::Arbitrary { .. } => (),
        }

        Ok(())
    }

    fn can_handle(&self, _config: &Config, modifier: &Modifier) -> bool {
        match modifier {
            Modifier::Basic { value, .. } => {
                ["spin", "ping", "pulse", "bounce", "none"].contains(&value.as_str())
            }
            Modifier::Arbitrary { value, .. } => is_matching_all(value),
        }
    }

    fn handle(
        &self,
        _config: &Config,
        modifier: &Modifier,
        indentation: usize,
        buffer: &mut String,
    ) -> fmt::Result {
        match modifier {
            Modifier::Basic { value, .. } => {
                let animation = match value.as_str() {
                    "none" => "none",
                    "spin" => "spin 1s linear infinite",
                    "ping" => "ping 1s cubic-bezier(0, 0, 0.2, 1) infinite",
                    "pulse" => "pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite",
                    "bounce" => "bounce 1s infinite",
                    _ => unreachable!(),
                };

                indent(indentation, buffer)?;
                writeln!(buffer, "-webkit-animation: {animation};")?;
                indent(indentation, buffer)?;
                writeln!(buffer, "animation: {animation};")?;
            }
            Modifier::Arbitrary { value, .. } => {
                indent(indentation, buffer)?;
                writeln!(buffer, "-webkit-animation: {value};")?;
                indent(indentation, buffer)?;
                writeln!(buffer, "animation: {value};")?;
            }
        }

        Ok(())
    }
}
