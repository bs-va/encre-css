#![doc = include_str!("README.md")]
#![doc(alias = "transition")]
use crate::{
    generator::generate_wrapper,
    generator::{ContextCanHandle, ContextHandle},
    plugins::Plugin,
    selector::Modifier,
    utils::value_matchers::is_matching_all,
};

use std::fmt::{self, Write};

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
        let ContextHandle { modifier, buffer, .. } = context;

        match modifier {
            Modifier::Builtin { value, .. } => {
                let animation = match *value {
                    "none" => "none",
                    "spin" => {
                        writeln!(buffer, "{}", SPIN_ANIMATION)?;
                        "spin 1s linear infinite"
                    }
                    "ping" => {
                        writeln!(buffer, "{}", PING_ANIMATION)?;
                        "ping 1s cubic-bezier(0, 0, 0.2, 1) infinite"
                    }
                    "pulse" => {
                        writeln!(buffer, "{}", PULSE_ANIMATION)?;
                        "pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite"
                    }
                    "bounce" => {
                        writeln!(buffer, "{}", BOUNCE_ANIMATION)?;
                        "bounce 1s infinite"
                    }
                    _ => unreachable!(),
                };

                generate_wrapper(context, |ContextHandle { indentation, buffer, .. }| {
                    writeln!(buffer, "{indentation}-webkit-animation: {animation};\n{indentation}animation: {animation};")
                })
            }
            Modifier::Arbitrary { value, .. } => generate_wrapper(context, |ContextHandle { indentation, buffer, .. }| {
                writeln!(buffer, "{indentation}-webkit-animation: {value};\n{indentation}animation: {value};")
            }),
        }
    }
}
