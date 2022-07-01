#![doc = include_str!("README.md")]
use crate::{
    plugins::{to_css_value, Plugin},
    context::{ContextCanHandle, ContextHandle, ContextBeforeRule},
    selector::Modifier,
    utils::{indent, value_matchers::is_matching_all},
};

use std::{
    fmt::{self, Write},
    sync::atomic::{AtomicBool, Ordering},
};

pub(crate) static ANIMATIONS_ALREADY_DEFINED: [AtomicBool; 4] = [
    AtomicBool::new(false), // Spin
    AtomicBool::new(false), // Ping
    AtomicBool::new(false), // Pulse
    AtomicBool::new(false), // Bounce
];

#[derive(Debug)]
pub(crate) struct PluginDefinition;

impl Plugin for PluginDefinition {
    fn namespace(&self) -> &str {
        "animate"
    }

    fn css_before_rule(&self, context: ContextBeforeRule) -> fmt::Result {
        match context.selector.modifier {
            Modifier::Builtin { value, .. } => {
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
            Modifier::Builtin { value, .. } => {
                ["spin", "ping", "pulse", "bounce", "none"].contains(value)
            }
            Modifier::Arbitrary { value, .. } => is_matching_all(value),
        }
    }

    fn handle(&self, context: ContextHandle) -> fmt::Result {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
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
