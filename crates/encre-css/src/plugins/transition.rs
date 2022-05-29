use super::Plugin;
use crate::utils::value_matchers::*;
use crate::{config::Config, selector::Modifier};

use lazy_static::lazy_static;
use regex::Regex;
use std::{
    fmt::Write,
    sync::atomic::{AtomicBool, Ordering},
};

lazy_static! {
    static ref PROPERTY_REGEX: Regex = Regex::new(r"[^\d]+").unwrap();
}

#[derive(Debug)]
pub struct PropertyPlugin;

impl Plugin for PropertyPlugin {
    fn namespace(&self) -> &str {
        "transition"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        PROPERTY_REGEX.is_match(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "transition-property: {val};").is_ok()
    }

    fn get_css_for_modifier(
        &self,
        _config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        match modifier.content() {
            "" => write!(css_content, "transition-property: color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter;
transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
transition-duration: 150ms;").is_ok(),
            "none" => write!(css_content, "transition-property: none;").is_ok(),
            "all" => write!(css_content, "transition-property: all;
transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1);
transition-duration: 150ms;").is_ok(),
            "colors" => write!(css_content, "transition-property: color, background-color, border-color, text-decoration-color, fill, stroke;
transition-timing-function: cubic-bezier(0.4, 0, 0);
transition-duration: 150ms;").is_ok(),
            "opacity" => write!(css_content, "transition-property: opacity;
transition-timing-function: cubic-bezier(0.4, 0, 0);
transition-duration: 150ms;").is_ok(),
            "shadow" => write!(css_content, "transition-property: box-shadow;
transition-timing-function: cubic-bezier(0.4, 0, 0);
transition-duration: 150ms;").is_ok(),
            "transform" => write!(css_content, "transition-property: transform;
transition-timing-function: cubic-bezier(0.4, 0, 0);
transition-duration: 150ms;").is_ok(),
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct DurationPlugin;

impl Plugin for DurationPlugin {
    fn namespace(&self) -> &str {
        "duration"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        is_matching_time(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "transition-duration: {val};").is_ok()
    }

    fn get_css_for_modifier(
        &self,
        _config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        // NOTE: Not-compatible with TailwindCSS, support all values
        if let Ok(duration) = modifier.to_usize() {
            self.css_template_value(&format!("{duration}ms"), css_content)
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct DelayPlugin;

impl Plugin for DelayPlugin {
    fn namespace(&self) -> &str {
        "delay"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        is_matching_time(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "transition-delay: {val};").is_ok()
    }

    fn get_css_for_modifier(
        &self,
        _config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        // NOTE: Not-compatible with TailwindCSS, support all values
        if let Ok(delay) = modifier.to_usize() {
            self.css_template_value(&format!("{delay}ms"), css_content)
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct EasePlugin;

impl Plugin for EasePlugin {
    fn namespace(&self) -> &str {
        "ease"
    }

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        is_matching_all(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(css_content, "transition-timing-function: {val};").is_ok()
    }

    fn get_css_for_modifier(
        &self,
        _config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        _custom_css: &mut String,
    ) -> bool {
        match modifier.content() {
            "linear" => self.css_template_value("linear", css_content),
            "in" => self.css_template_value("cubic-bezier(0.4, 0, 1, 1)", css_content),
            "out" => self.css_template_value("cubic-bezier(0, 0, 0.2, 1)", css_content),
            "in-out" => self.css_template_value("cubic-bezier(0.4, 0, 0.2, 1)", css_content),
            _ => false,
        }
    }
}

#[derive(Debug)]
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

    fn is_matching_value(&self, _hint: &str, val: &str) -> bool {
        is_matching_all(val)
    }

    fn css_template_value(&self, val: &str, css_content: &mut String) -> bool {
        write!(
            css_content,
            "-webkit-animation: bounce 1s infinite;
animation: {val};"
        )
        .is_ok()
    }

    fn get_css_for_modifier(
        &self,
        _config: &Config,
        modifier: &Modifier,
        css_content: &mut String,
        custom_css: &mut String,
    ) -> bool {
        println!("{}", modifier);
        match modifier.content() {
            "none" => self.css_template_value("none", css_content),
            "spin" => {
                if !self
                    .is_spin_animation_already_defined
                    .swap(true, Ordering::Relaxed)
                {
                    write!(
                        custom_css,
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
}}

"
                    )
                    .unwrap();
                }

                self.css_template_value("spin 1s linear infinite", css_content)
            }
            "ping" => {
                if !self
                    .is_ping_animation_already_defined
                    .swap(true, Ordering::Relaxed)
                {
                    write!(
                        custom_css,
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
}}

"
                    )
                    .unwrap();
                }

                self.css_template_value("ping 1s cubic-bezier(0, 0, 0.2, 1) infinite", css_content)
            }
            "pulse" => {
                if !self
                    .is_pulse_animation_already_defined
                    .swap(true, Ordering::Relaxed)
                {
                    write!(
                        custom_css,
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
}}

"
                    )
                    .unwrap();
                }

                self.css_template_value(
                    "pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite",
                    css_content,
                )
            }
            "bounce" => {
                if !self
                    .is_bounce_animation_already_defined
                    .swap(true, Ordering::Relaxed)
                {
                    write!(
                        custom_css,
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
}}

"
                    )
                    .unwrap();
                }

                self.css_template_value("bounce 1s infinite", css_content)
            }
            _ => false,
        }
    }
}
