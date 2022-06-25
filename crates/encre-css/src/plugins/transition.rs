use super::{to_css_value, Plugin};
use crate::utils::{indent, value_matchers::*};
use crate::{config::Config, selector::Modifier};

use std::{
    fmt::{self, Write},
    sync::atomic::{AtomicBool, Ordering},
};

#[derive(Debug)]
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
        indent(indentation, buffer)?;
        match modifier {
            Modifier::Basic { value, .. } => match *value {
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
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "transition-property: {};", to_css_value(value))?
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
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
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "transition-duration: {};", to_css_value(value))?
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
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
            Modifier::Arbitrary { value, .. } => {
                writeln!(buffer, "transition-delay: {};", to_css_value(value))?
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
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
            Modifier::Basic { value, .. } => match *value {
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
            Modifier::Arbitrary { value, .. } => writeln!(
                buffer,
                "transition-timing-function: {};",
                to_css_value(value)
            )?,
        }

        Ok(())
    }
}

pub static ANIMATIONS_ALREADY_DEFINED: [AtomicBool; 4] = [
    AtomicBool::new(false), // Spin
    AtomicBool::new(false), // Ping
    AtomicBool::new(false), // Pulse
    AtomicBool::new(false), // Bounce
];

#[derive(Debug)]
pub struct AnimatePlugin;

impl Plugin for AnimatePlugin {
    fn namespace(&self) -> &str {
        "animate"
    }

    fn css_before_rule(
        &self,
        _config: &Config,
        modifier: &Modifier,
        buffer: &mut String,
    ) -> fmt::Result {
        match modifier {
            Modifier::Basic { value, .. } => {
                match *value {
                    "spin" => {
                        if !ANIMATIONS_ALREADY_DEFINED[0].swap(true, Ordering::Relaxed) {
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
                        if !ANIMATIONS_ALREADY_DEFINED[1].swap(true, Ordering::Relaxed) {
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
                        if !ANIMATIONS_ALREADY_DEFINED[2].swap(true, Ordering::Relaxed) {
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
                        if !ANIMATIONS_ALREADY_DEFINED[3].swap(true, Ordering::Relaxed) {
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
                ["spin", "ping", "pulse", "bounce", "none"].contains(value)
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
                let animation = match *value {
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
                let value = to_css_value(value);
                indent(indentation, buffer)?;
                writeln!(buffer, "-webkit-animation: {value};")?;
                indent(indentation, buffer)?;
                writeln!(buffer, "animation: {value};")?;
            }
        }

        Ok(())
    }
}
