#![doc = include_str!("README.md")]
use crate::{
    generator::generate_wrapper,
    generator::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
    utils::{indent, value_matchers::is_matching_all},
};

use std::{
    fmt::{self, Write},
    sync::atomic::{AtomicBool, Ordering},
};

const SPIN_ANIMATION: &str = "@-webkit-keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

@keyframes spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}\n";

const PING_ANIMATION: &str = "@-webkit-keyframes ping {
  75%, 100% {
    transform: scale(2);
    opacity: 0;
  }
}

@keyframes ping {
  75%, 100% {
    transform: scale(2);
    opacity: 0;
  }
}\n";

const PULSE_ANIMATION: &str = "@-webkit-keyframes pulse {
  50% {
    opacity: .5;
  }
}

@keyframes pulse {
  0%, 100% {
    opacity: 1;
  }
  50% {
    opacity: .5;
  }
}\n";

const BOUNCE_ANIMATION: &str = "@-webkit-keyframes bounce {
  0%, 100% {
    transform: translateY(-25%);
    -webkit-animation-timing-function: cubic-bezier(0.8,0,1,1);
    animation-timing-function: cubic-bezier(0.8,0,1,1);
  }

  50% {
    transform: none;
    -webkit-animation-timing-function: cubic-bezier(0,0,0.2,1);
    animation-timing-function: cubic-bezier(0,0,0.2,1);
  }
}

@keyframes bounce {
  0%, 100% {
    transform: translateY(-25%);
    -webkit-animation-timing-function: cubic-bezier(0.8,0,1,1);
    animation-timing-function: cubic-bezier(0.8, 0, 1, 1);
  }
  50% {
    transform: translateY(0);
    -webkit-animation-timing-function: cubic-bezier(0,0,0.2,1);
    animation-timing-function: cubic-bezier(0, 0, 0.2, 1);
  }
}\n";

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

    fn needs_wrapping(&self) -> bool {
        false
    }

    fn can_handle(&self, context: ContextCanHandle) -> bool {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                ["spin", "ping", "pulse", "bounce", "none"].contains(value)
            }
            Modifier::Arbitrary { value, .. } => is_matching_all(value),
        }
    }

    fn handle(&self, context: &mut ContextHandle) -> fmt::Result {
        match context.modifier {
            Modifier::Builtin { value, .. } => {
                let animation = match *value {
                    "none" => "none",
                    "spin" => {
                        if !ANIMATIONS_ALREADY_DEFINED[0].swap(true, Ordering::Relaxed) {
                            writeln!(context.buffer, "{}", SPIN_ANIMATION)?;
                        }
                        "spin 1s linear infinite"
                    }
                    "ping" => {
                        if !ANIMATIONS_ALREADY_DEFINED[1].swap(true, Ordering::Relaxed) {
                            writeln!(context.buffer, "{}", PING_ANIMATION)?;
                        }
                        "ping 1s cubic-bezier(0, 0, 0.2, 1) infinite"
                    }
                    "pulse" => {
                        if !ANIMATIONS_ALREADY_DEFINED[2].swap(true, Ordering::Relaxed) {
                            writeln!(context.buffer, "{}", PULSE_ANIMATION)?;
                        }
                        "pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite"
                    }
                    "bounce" => {
                        if !ANIMATIONS_ALREADY_DEFINED[3].swap(true, Ordering::Relaxed) {
                            writeln!(context.buffer, "{}", BOUNCE_ANIMATION)?;
                        }
                        "bounce 1s infinite"
                    }
                    _ => unreachable!(),
                };

                generate_wrapper(context, |context| {
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "-webkit-animation: {animation};")?;
                    indent(context.indentation, context.buffer)?;
                    writeln!(context.buffer, "animation: {animation};")
                })
            }
            Modifier::Arbitrary { value, .. } => generate_wrapper(context, |context| {
                indent(context.indentation, context.buffer)?;
                writeln!(context.buffer, "-webkit-animation: {value};")?;
                indent(context.indentation, context.buffer)?;
                writeln!(context.buffer, "animation: {value};")
            }),
        }
    }
}
