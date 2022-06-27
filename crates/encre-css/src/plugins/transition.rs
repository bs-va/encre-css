use super::{to_css_value, Plugin};
use crate::{
    context::{ContextBeforeRule, ContextCanHandle, ContextHandle},
    selector::Modifier,
    utils::{indent, value_matchers::*},
};

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

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
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

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "" => {
                    writeln!(context.buffer, "transition-property: color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(
                        context.buffer,
                        "transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);"
                    )?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "transition-duration: 150ms;")?;
                }
                "none" => writeln!(context.buffer, "transition-property: none;")?,
                "all" => {
                    writeln!(context.buffer, "transition-property: all;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(
                        context.buffer,
                        "transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);"
                    )?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "transition-duration: 150ms;")?;
                }
                "colors" => {
                    writeln!(context.buffer, "transition-property: color, background-color, border-color, text-decoration-color, fill, stroke;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(
                        context.buffer,
                        "transition-timing-function: cubic-bezier(0.4, 0, 0);"
                    )?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "transition-duration: 150ms;")?;
                }
                "opacity" => {
                    writeln!(context.buffer, "transition-property: opacity;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(
                        context.buffer,
                        "transition-timing-function: cubic-bezier(0.4, 0, 0);"
                    )?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "transition-duration: 150ms;")?;
                }
                "shadow" => {
                    writeln!(context.buffer, "transition-property: box-shadow;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(
                        context.buffer,
                        "transition-timing-function: cubic-bezier(0.4, 0, 0);"
                    )?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "transition-duration: 150ms;")?;
                }
                "transform" => {
                    writeln!(context.buffer, "transition-property: transform;")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(
                        context.buffer,
                        "transition-timing-function: cubic-bezier(0.4, 0, 0);"
                    )?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "transition-duration: 150ms;")?;
                }
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => writeln!(
                context.buffer,
                "transition-property: {};",
                to_css_value(value)
            )?,
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

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => value.parse::<usize>().is_ok(),
            Modifier::Arbitrary { value, .. } => is_matching_time(value),
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        // NOTE: Not-compatible with TailwindCSS, support all values
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => {
                writeln!(context.buffer, "transition-duration: {value}ms;")?
            }
            Modifier::Arbitrary { value, .. } => writeln!(
                context.buffer,
                "transition-duration: {};",
                to_css_value(value)
            )?,
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

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => value.parse::<usize>().is_ok(),
            Modifier::Arbitrary { value, .. } => is_matching_time(value),
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        // NOTE: Not-compatible with TailwindCSS, support all values
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => {
                writeln!(context.buffer, "transition-delay: {value}ms;")?
            }
            Modifier::Arbitrary { value, .. } => {
                writeln!(context.buffer, "transition-delay: {};", to_css_value(value))?
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

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => value.parse::<usize>().is_ok(),
            Modifier::Arbitrary { value, .. } => is_matching_all(value), // TODO: Better matching
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        indent(context.indentation, context.buffer)?;
        match context.modifier {
            Modifier::Basic { value, .. } => match *value {
                "linear" => writeln!(context.buffer, "transition-timing-function: linear;")?,
                "in" => writeln!(
                    context.buffer,
                    "transition-timing-function: cubic-bezier(0.4, 0, 1, 1);"
                )?,
                "out" => writeln!(
                    context.buffer,
                    "transition-timing-function: cubic-bezier(0, 0, 0.2, 1);"
                )?,
                "in-out" => writeln!(
                    context.buffer,
                    "transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);"
                )?,
                _ => unreachable!(),
            },
            Modifier::Arbitrary { value, .. } => writeln!(
                context.buffer,
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

    fn css_before_rule(&self, context: ContextBeforeRule) -> fmt::Result {
        match context.selector.modifier {
            Modifier::Basic { value, .. } => {
                match value {
                    "spin" => {
                        if !ANIMATIONS_ALREADY_DEFINED[0].swap(true, Ordering::Relaxed) {
                            writeln!(
                                context.buffer,
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
                                context.buffer,
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
                                context.buffer,
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
                                context.buffer,
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

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Basic { value, .. } => {
                ["spin", "ping", "pulse", "bounce", "none"].contains(value)
            }
            Modifier::Arbitrary { value, .. } => is_matching_all(value),
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        match context.modifier {
            Modifier::Basic { value, .. } => {
                let animation = match *value {
                    "none" => "none",
                    "spin" => "spin 1s linear infinite",
                    "ping" => "ping 1s cubic-bezier(0, 0, 0.2, 1) infinite",
                    "pulse" => "pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite",
                    "bounce" => "bounce 1s infinite",
                    _ => unreachable!(),
                };

                indent(context.indentation, context.buffer)?;
                writeln!(context.buffer, "-webkit-animation: {animation};")?;
                indent(context.indentation, context.buffer)?;
                writeln!(context.buffer, "animation: {animation};")?;
            }
            Modifier::Arbitrary { value, .. } => {
                let value = to_css_value(value);
                indent(context.indentation, context.buffer)?;
                writeln!(context.buffer, "-webkit-animation: {value};")?;
                indent(context.indentation, context.buffer)?;
                writeln!(context.buffer, "animation: {value};")?;
            }
        }

        Ok(())
    }
}
