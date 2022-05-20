use super::Plugin;
use crate::{config::Config, selector::Modifier};

use std::fmt::Write;

const CSS_FILTER: &str = "filter: var(--en-blur) var(--en-brightness) var(--en-contrast) var(--en-grayscale) var(--en-hue-rotate) var(--en-invert) var(--en-saturate) var(--en-sepia) var(--en-drop-shadow);";
const CSS_BACKDROP_FILTER: &str = "-webkit-backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
  backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);";

#[derive(Debug)]
pub struct FilterPlugin;

impl Plugin for FilterPlugin {
    fn get_css_for_modifier(&self, _config: &Config, modifier: &Modifier, css_content: &mut String) -> bool {
        if modifier.content() == "filter" {
            return write!(css_content, "{}", CSS_FILTER).is_ok();
        } else if modifier.content() == "filter-none" {
            return write!(css_content, "filter: none;").is_ok();
        }

        let result = match modifier.content() {
            // TODO: Better arbitrary values
            "blur-0" => write!(css_content, "--en-blur: blur(0);").is_ok(),
            "blur-sm" => write!(css_content, "--en-blur: blur(4px);").is_ok(),
            "blur" => write!(css_content, "--en-blur: blur(8px);").is_ok(),
            "blur-md" => write!(css_content, "--en-blur: blur(12px);").is_ok(),
            "blur-lg" => write!(css_content, "--en-blur: blur(16px);").is_ok(),
            "blur-xl" => write!(css_content, "--en-blur: blur(24px);").is_ok(),
            "blur-2xl" => write!(css_content, "--en-blur: blur(40px);").is_ok(),
            "blur-3xl" => write!(css_content, "--en-blur: blur(64px);").is_ok(),
            "brightness-0" => write!(css_content, "--en-brightness: brightness(0);").is_ok(),
            "brightness-50" => write!(css_content, "--en-brightness: brightness(.5);").is_ok(),
            "brightness-75" => write!(css_content, "--en-brightness: brightness(.75);").is_ok(),
            "brightness-90" => write!(css_content, "--en-brightness: brightness(.9);").is_ok(),
            "brightness-95" => write!(css_content, "--en-brightness: brightness(.95);").is_ok(),
            "brightness-100" => write!(css_content, "--en-brightness: brightness(1);").is_ok(),
            "brightness-105" => write!(css_content, "--en-brightness: brightness(1.05);").is_ok(),
            "brightness-110" => write!(css_content, "--en-brightness: brightness(1.1);").is_ok(),
            "brightness-125" => write!(css_content, "--en-brightness: brightness(1.25);").is_ok(),
            "brightness-150" => write!(css_content, "--en-brightness: brightness(1.5);").is_ok(),
            "brightness-200" => write!(css_content, "--en-brightness: brightness(2);").is_ok(),
            "contrast-0" => write!(css_content, "--en-contrast: contrast(0);").is_ok(),
            "contrast-50" => write!(css_content, "--en-contrast: contrast(.5);").is_ok(),
            "contrast-75" => write!(css_content, "--en-contrast: contrast(.75);").is_ok(),
            "contrast-100" => write!(css_content, "--en-contrast: contrast(1);").is_ok(),
            "contrast-125" => write!(css_content, "--en-contrast: contrast(1.25);").is_ok(),
            "contrast-150" => write!(css_content, "--en-contrast: contrast(1.5);").is_ok(),
            "contrast-200" => write!(css_content, "--en-contrast: contrast(2);").is_ok(),
            "drop-shadow-sm" => write!(css_content, "--en-drop-shadow: drop-shadow(0 1px 1px rgba(0,0,0,0.05));").is_ok(),
            "drop-shadow" => write!(css_content, "--en-drop-shadow: drop-shadow(0 1px 2px rgba(0, 0, 0, 0.1)) drop-shadow(0 1px 1px rgba(0, 0, 0, 0.06));").is_ok(),
            "drop-shadow-md" => write!(css_content, "--en-drop-shadow: drop-shadow(0 4px 3px rgba(0, 0, 0, 0.07)) drop-shadow(0 2px 2px rgba(0, 0, 0, 0.06));").is_ok(),
            "drop-shadow-lg" => write!(css_content, "--en-drop-shadow: drop-shadow(0 10px 8px rgba(0, 0, 0, 0.04)) drop-shadow(0 4px 3px rgba(0, 0, 0, 0.1));").is_ok(),
            "drop-shadow-xl" => write!(css_content, "--en-drop-shadow: drop-shadow(0 20px 13px rgba(0, 0, 0, 0.03)) drop-shadow(0 8px 5px rgba(0, 0, 0, 0.08));").is_ok(),
            "drop-shadow-2xl" => write!(css_content, "--en-drop-shadow: drop-shadow: drop-shadow(0 25px 25px rgba(0, 0, 0, 0.15));").is_ok(),
            "drop-shadow-none" => write!(css_content, "--en-drop-shadow: drop-shadow: drop-shadow(0 0 #0000);").is_ok(),
            "grayscale-0" => write!(css_content, "--en-grayscale: grayscale(0);").is_ok(),
            "grayscale" => write!(css_content, "--en-grayscale: grayscale(100%);").is_ok(),
            "hue-rotate-0" => write!(css_content, "--en-hue-rotate: hue-rotate(0deg);").is_ok(),
            "hue-rotate-15" => write!(css_content, "--en-hue-rotate: hue-rotate({}15deg);", if modifier.is_negative() { "-" } else { "" }).is_ok(),
            "hue-rotate-30" => write!(css_content, "--en-hue-rotate: hue-rotate({}30deg);", if modifier.is_negative() { "-" } else { "" }).is_ok(),
            "hue-rotate-60" => write!(css_content, "--en-hue-rotate: hue-rotate({}60deg);", if modifier.is_negative() { "-" } else { "" }).is_ok(),
            "hue-rotate-90" => write!(css_content, "--en-hue-rotate: hue-rotate({}90deg);", if modifier.is_negative() { "-" } else { "" }).is_ok(),
            "hue-rotate-180" => write!(css_content, "--en-hue-rotate: hue-rotate({}180deg);", if modifier.is_negative() { "-" } else { "" }).is_ok(),
            "invert-0" => write!(css_content, "--en-invert: invert(0);").is_ok(),
            "invert" => write!(css_content, "--en-invert: invert(100%);").is_ok(),
            "saturate-0" => write!(css_content, "--en-saturate: saturate(0);").is_ok(),
            "saturate-50" => write!(css_content, "--en-saturate: saturate(.5);").is_ok(),
            "saturate-100" => write!(css_content, "--en-saturate: saturate(1);").is_ok(),
            "saturate-150" => write!(css_content, "--en-saturate: saturate(1.5);").is_ok(),
            "saturate-200" => write!(css_content, "--en-saturate: saturate(2);").is_ok(),
            "sepia-0" => write!(css_content, "--en-sepia: sepia(0);").is_ok(),
            "sepia" => write!(css_content, "--en-sepia: sepia(100%);").is_ok(),
            _ => return false,
        };

        result && write!(css_content, "\n{}", CSS_FILTER).is_ok()
    }
}

#[derive(Debug)]
pub struct BackdropFilterPlugin;

impl Plugin for BackdropFilterPlugin {
    fn namespace(&self) -> &str {
        "backdrop"
    }

    fn get_css_for_modifier(&self, _config: &Config, modifier: &Modifier, css_content: &mut String) -> bool {
        if modifier.content() == "filter" {
            return write!(css_content, "{}", CSS_BACKDROP_FILTER).is_ok();
        } else if modifier.content() == "filter-none" {
            return write!(css_content, "filter: none;").is_ok();
        }

        let result = match modifier.content() {
            // TODO: Avoid duplication with the plugin above
            "blur-0" => write!(css_content, "--en-backdrop-blur: blur(0);").is_ok(),
            "blur-sm" => write!(css_content, "--en-backdrop-blur: blur(4px);").is_ok(),
            "blur" => write!(css_content, "--en-backdrop-blur: blur(8px);").is_ok(),
            "blur-md" => write!(css_content, "--en-backdrop-blur: blur(12px);").is_ok(),
            "blur-lg" => write!(css_content, "--en-backdrop-blur: blur(16px);").is_ok(),
            "blur-xl" => write!(css_content, "--en-backdrop-blur: blur(24px);").is_ok(),
            "blur-2xl" => write!(css_content, "--en-backdrop-blur: blur(40px);").is_ok(),
            "blur-3xl" => write!(css_content, "--en-backdrop-blur: blur(64px);").is_ok(),
            "brightness-0" => write!(css_content, "--en-backdrop-brightness: brightness(0);").is_ok(),
            "brightness-50" => write!(css_content, "--en-backdrop-brightness: brightness(.5);").is_ok(),
            "brightness-75" => write!(css_content, "--en-backdrop-brightness: brightness(.75);").is_ok(),
            "brightness-90" => write!(css_content, "--en-backdrop-brightness: brightness(.9);").is_ok(),
            "brightness-95" => write!(css_content, "--en-backdrop-brightness: brightness(.95);").is_ok(),
            "brightness-100" => write!(css_content, "--en-backdrop-brightness: brightness(1);").is_ok(),
            "brightness-105" => write!(css_content, "--en-backdrop-brightness: brightness(1.05);").is_ok(),
            "brightness-110" => write!(css_content, "--en-backdrop-brightness: brightness(1.1);").is_ok(),
            "brightness-125" => write!(css_content, "--en-backdrop-brightness: brightness(1.25);").is_ok(),
            "brightness-150" => write!(css_content, "--en-backdrop-brightness: brightness(1.5);").is_ok(),
            "brightness-200" => write!(css_content, "--en-backdrop-brightness: brightness(2);").is_ok(),
            "contrast-0" => write!(css_content, "--en-backdrop-contrast: contrast(0);").is_ok(),
            "contrast-50" => write!(css_content, "--en-backdrop-contrast: contrast(.5);").is_ok(),
            "contrast-75" => write!(css_content, "--en-backdrop-contrast: contrast(.75);").is_ok(),
            "contrast-100" => write!(css_content, "--en-backdrop-contrast: contrast(1);").is_ok(),
            "contrast-125" => write!(css_content, "--en-backdrop-contrast: contrast(1.25);").is_ok(),
            "contrast-150" => write!(css_content, "--en-backdrop-contrast: contrast(1.5);").is_ok(),
            "contrast-200" => write!(css_content, "--en-backdrop-contrast: contrast(2);").is_ok(),
            "drop-shadow-sm" => write!(css_content, "--en-backdrop-drop-shadow: drop-shadow(0 1px 1px rgba(0,0,0,0.05));").is_ok(),
            "drop-shadow" => write!(css_content, "--en-backdrop-drop-shadow: drop-shadow(0 1px 2px rgba(0, 0, 0, 0.1)) drop-shadow(0 1px 1px rgba(0, 0, 0, 0.06));").is_ok(),
            "drop-shadow-md" => write!(css_content, "--en-backdrop-drop-shadow: drop-shadow(0 4px 3px rgba(0, 0, 0, 0.07)) drop-shadow(0 2px 2px rgba(0, 0, 0, 0.06));").is_ok(),
            "drop-shadow-lg" => write!(css_content, "--en-backdrop-drop-shadow: drop-shadow(0 10px 8px rgba(0, 0, 0, 0.04)) drop-shadow(0 4px 3px rgba(0, 0, 0, 0.1));").is_ok(),
            "drop-shadow-xl" => write!(css_content, "--en-backdrop-drop-shadow: drop-shadow(0 20px 13px rgba(0, 0, 0, 0.03)) drop-shadow(0 8px 5px rgba(0, 0, 0, 0.08));").is_ok(),
            "drop-shadow-2xl" => write!(css_content, "--en-backdrop-drop-shadow: drop-shadow: drop-shadow(0 25px 25px rgba(0, 0, 0, 0.15));").is_ok(),
            "drop-shadow-none" => write!(css_content, "--en-backdrop-drop-shadow: drop-shadow: drop-shadow(0 0 #0000);").is_ok(),
            "grayscale-0" => write!(css_content, "--en-backdrop-grayscale: grayscale(0);").is_ok(),
            "grayscale" => write!(css_content, "--en-backdrop-grayscale: grayscale(100%);").is_ok(),
            "hue-rotate-0" => write!(css_content, "--en-backdrop-hue-rotate: hue-rotate(0deg);").is_ok(),
            "hue-rotate-15" => write!(css_content, "--en-backdrop-hue-rotate: hue-rotate({}15deg);", if modifier.is_negative() { "-" } else { "" }).is_ok(),
            "hue-rotate-30" => write!(css_content, "--en-backdrop-hue-rotate: hue-rotate({}30deg);", if modifier.is_negative() { "-" } else { "" }).is_ok(),
            "hue-rotate-60" => write!(css_content, "--en-backdrop-hue-rotate: hue-rotate({}60deg);", if modifier.is_negative() { "-" } else { "" }).is_ok(),
            "hue-rotate-90" => write!(css_content, "--en-backdrop-hue-rotate: hue-rotate({}90deg);", if modifier.is_negative() { "-" } else { "" }).is_ok(),
            "hue-rotate-180" => write!(css_content, "--en-backdrop-hue-rotate: hue-rotate({}180deg);", if modifier.is_negative() { "-" } else { "" }).is_ok(),
            "invert-0" => write!(css_content, "--en-backdrop-invert: invert(0);").is_ok(),
            "invert" => write!(css_content, "--en-backdrop-invert: invert(100%);").is_ok(),
            "saturate-0" => write!(css_content, "--en-backdrop-saturate: saturate(0);").is_ok(),
            "saturate-50" => write!(css_content, "--en-backdrop-saturate: saturate(.5);").is_ok(),
            "saturate-100" => write!(css_content, "--en-backdrop-saturate: saturate(1);").is_ok(),
            "saturate-150" => write!(css_content, "--en-backdrop-saturate: saturate(1.5);").is_ok(),
            "saturate-200" => write!(css_content, "--en-backdrop-saturate: saturate(2);").is_ok(),
            "sepia-0" => write!(css_content, "--en-backdrop-sepia: sepia(0);").is_ok(),
            "sepia" => write!(css_content, "--en-backdrop-sepia: sepia(100%);").is_ok(),
            _ => return false,
        };

        result && write!(css_content, "\n{}", CSS_FILTER).is_ok()
    }
}
