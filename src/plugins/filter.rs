use super::Plugin;

const CSS_FILTER: &str = "filter: var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow);";
const CSS_BACKDROP_FILTER: &str = "-webkit-backdrop-filter: var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia);
  backdrop-filter: var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia);";

#[derive(Debug)]
pub struct FilterPlugin;

impl Plugin for FilterPlugin {
    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        if modifier == "filter" {
            return Some(CSS_FILTER.to_string());
        } else if modifier == "filter-none" {
            return Some("filter: none;".to_string());
        }

        let result = match modifier {
            // TODO: Better negative values + arbitrary values
            "blur-0" => Some("--tw-blur: blur(0);"),
            "blur-sm" => Some("--tw-blur: blur(4px);"),
            "blur" => Some("--tw-blur: blur(8px);"),
            "blur-md" => Some("--tw-blur: blur(12px);"),
            "blur-lg" => Some("--tw-blur: blur(16px);"),
            "blur-xl" => Some("--tw-blur: blur(24px);"),
            "blur-2xl" => Some("--tw-blur: blur(40px);"),
            "blur-3xl" => Some("--tw-blur: blur(64px);"),
            "brightness-0" => Some("--tw-brightness: brightness(0);"),
            "brightness-50" => Some("--tw-brightness: brightness(.5);"),
            "brightness-75" => Some("--tw-brightness: brightness(.75);"),
            "brightness-90" => Some("--tw-brightness: brightness(.9);"),
            "brightness-95" => Some("--tw-brightness: brightness(.95);"),
            "brightness-100" => Some("--tw-brightness: brightness(1);"),
            "brightness-105" => Some("--tw-brightness: brightness(1.05);"),
            "brightness-110" => Some("--tw-brightness: brightness(1.1);"),
            "brightness-125" => Some("--tw-brightness: brightness(1.25);"),
            "brightness-150" => Some("--tw-brightness: brightness(1.5);"),
            "brightness-200" => Some("--tw-brightness: brightness(2);"),
            "contrast-0" => Some("--tw-contrast: contrast(0);"),
            "contrast-50" => Some("--tw-contrast: contrast(.5);"),
            "contrast-75" => Some("--tw-contrast: contrast(.75);"),
            "contrast-100" => Some("--tw-contrast: contrast(1);"),
            "contrast-125" => Some("--tw-contrast: contrast(1.25);"),
            "contrast-150" => Some("--tw-contrast: contrast(1.5);"),
            "contrast-200" => Some("--tw-contrast: contrast(2);"),
            "drop-shadow-sm" => Some("--tw-drop-shadow: drop-shadow(0 1px 1px rgba(0,0,0,0.05));"),
            "drop-shadow" => Some("--tw-drop-shadow: drop-shadow(0 1px 2px rgba(0, 0, 0, 0.1)) drop-shadow(0 1px 1px rgba(0, 0, 0, 0.06));"),
            "drop-shadow-md" => Some("--tw-drop-shadow: drop-shadow(0 4px 3px rgba(0, 0, 0, 0.07)) drop-shadow(0 2px 2px rgba(0, 0, 0, 0.06));"),
            "drop-shadow-lg" => Some("--tw-drop-shadow: drop-shadow(0 10px 8px rgba(0, 0, 0, 0.04)) drop-shadow(0 4px 3px rgba(0, 0, 0, 0.1));"),
            "drop-shadow-xl" => Some("--tw-drop-shadow: drop-shadow(0 20px 13px rgba(0, 0, 0, 0.03)) drop-shadow(0 8px 5px rgba(0, 0, 0, 0.08));"),
            "drop-shadow-2xl" => Some("--tw-drop-shadow: drop-shadow: drop-shadow(0 25px 25px rgba(0, 0, 0, 0.15));"),
            "drop-shadow-none" => Some("--tw-drop-shadow: drop-shadow: drop-shadow(0 0 #0000);"),
            "grayscale-0" => Some("--tw-grayscale: grayscale(0);"),
            "grayscale" => Some("--tw-grayscale: grayscale(100%);"),
            "-hue-rotate-180" => Some("--tw-hue-rotate: hue-rotate(-180deg);"),
            "-hue-rotate-90" => Some("--tw-hue-rotate: hue-rotate(-90deg);"),
            "-hue-rotate-60" => Some("--tw-hue-rotate: hue-rotate(-60deg);"),
            "-hue-rotate-30" => Some("--tw-hue-rotate: hue-rotate(-30deg);"),
            "-hue-rotate-15" => Some("--tw-hue-rotate: hue-rotate(-15deg);"),
            "hue-rotate-0" => Some("--tw-hue-rotate: hue-rotate(0deg);"),
            "hue-rotate-15" => Some("--tw-hue-rotate: hue-rotate(15deg);"),
            "hue-rotate-30" => Some("--tw-hue-rotate: hue-rotate(30deg);"),
            "hue-rotate-60" => Some("--tw-hue-rotate: hue-rotate(60deg);"),
            "hue-rotate-90" => Some("--tw-hue-rotate: hue-rotate(90deg);"),
            "hue-rotate-180" => Some("--tw-hue-rotate: hue-rotate(180deg);"),
            "invert-0" => Some("--tw-invert: invert(0);"),
            "invert" => Some("--tw-invert: invert(100%);"),
            "saturate-0" => Some("--tw-saturate: saturate(0);"),
            "saturate-50" => Some("--tw-saturate: saturate(.5);"),
            "saturate-100" => Some("--tw-saturate: saturate(1);"),
            "saturate-150" => Some("--tw-saturate: saturate(1.5);"),
            "saturate-200" => Some("--tw-saturate: saturate(2);"),
            "sepia-0" => Some("--tw-sepia: sepia(0);"),
            "sepia" => Some("--tw-sepia: sepia(100%);"),
            _ => None,
        };

        result.map(|result| format!("{}
  {}", result, CSS_FILTER))
    }
}

#[derive(Debug)]
pub struct BackdropFilterPlugin;

impl Plugin for BackdropFilterPlugin {
    fn namespace(&self) -> String {
        "backdrop".to_string()
    }

    fn get_css_for_modifier(&self, modifier: &str) -> Option<String> {
        if modifier == "filter" {
            return Some(CSS_BACKDROP_FILTER.to_string());
        } else if modifier == "filter-none" {
            return Some("filter: none;".to_string());
        }

        let result = match modifier {
            // TODO: Avoid duplication with the plugin above
            "blur-0" => Some("--tw-backdrop-blur: blur(0);"),
            "blur-sm" => Some("--tw-backdrop-blur: blur(4px);"),
            "blur" => Some("--tw-backdrop-blur: blur(8px);"),
            "blur-md" => Some("--tw-backdrop-blur: blur(12px);"),
            "blur-lg" => Some("--tw-backdrop-blur: blur(16px);"),
            "blur-xl" => Some("--tw-backdrop-blur: blur(24px);"),
            "blur-2xl" => Some("--tw-backdrop-blur: blur(40px);"),
            "blur-3xl" => Some("--tw-backdrop-blur: blur(64px);"),
            "brightness-0" => Some("--tw-backdrop-brightness: brightness(0);"),
            "brightness-50" => Some("--tw-backdrop-brightness: brightness(.5);"),
            "brightness-75" => Some("--tw-backdrop-brightness: brightness(.75);"),
            "brightness-90" => Some("--tw-backdrop-brightness: brightness(.9);"),
            "brightness-95" => Some("--tw-backdrop-brightness: brightness(.95);"),
            "brightness-100" => Some("--tw-backdrop-brightness: brightness(1);"),
            "brightness-105" => Some("--tw-backdrop-brightness: brightness(1.05);"),
            "brightness-110" => Some("--tw-backdrop-brightness: brightness(1.1);"),
            "brightness-125" => Some("--tw-backdrop-brightness: brightness(1.25);"),
            "brightness-150" => Some("--tw-backdrop-brightness: brightness(1.5);"),
            "brightness-200" => Some("--tw-backdrop-brightness: brightness(2);"),
            "contrast-0" => Some("--tw-backdrop-contrast: contrast(0);"),
            "contrast-50" => Some("--tw-backdrop-contrast: contrast(.5);"),
            "contrast-75" => Some("--tw-backdrop-contrast: contrast(.75);"),
            "contrast-100" => Some("--tw-backdrop-contrast: contrast(1);"),
            "contrast-125" => Some("--tw-backdrop-contrast: contrast(1.25);"),
            "contrast-150" => Some("--tw-backdrop-contrast: contrast(1.5);"),
            "contrast-200" => Some("--tw-backdrop-contrast: contrast(2);"),
            "drop-shadow-sm" => Some("--tw-backdrop-drop-shadow: drop-shadow(0 1px 1px rgba(0,0,0,0.05));"),
            "drop-shadow" => Some("--tw-backdrop-drop-shadow: drop-shadow(0 1px 2px rgba(0, 0, 0, 0.1)) drop-shadow(0 1px 1px rgba(0, 0, 0, 0.06));"),
            "drop-shadow-md" => Some("--tw-backdrop-drop-shadow: drop-shadow(0 4px 3px rgba(0, 0, 0, 0.07)) drop-shadow(0 2px 2px rgba(0, 0, 0, 0.06));"),
            "drop-shadow-lg" => Some("--tw-backdrop-drop-shadow: drop-shadow(0 10px 8px rgba(0, 0, 0, 0.04)) drop-shadow(0 4px 3px rgba(0, 0, 0, 0.1));"),
            "drop-shadow-xl" => Some("--tw-backdrop-drop-shadow: drop-shadow(0 20px 13px rgba(0, 0, 0, 0.03)) drop-shadow(0 8px 5px rgba(0, 0, 0, 0.08));"),
            "drop-shadow-2xl" => Some("--tw-backdrop-drop-shadow: drop-shadow: drop-shadow(0 25px 25px rgba(0, 0, 0, 0.15));"),
            "drop-shadow-none" => Some("--tw-backdrop-drop-shadow: drop-shadow: drop-shadow(0 0 #0000);"),
            "grayscale-0" => Some("--tw-backdrop-grayscale: grayscale(0);"),
            "grayscale" => Some("--tw-backdrop-grayscale: grayscale(100%);"),
            "-hue-rotate-180" => Some("--tw-backdrop-hue-rotate: hue-rotate(-180deg);"),
            "-hue-rotate-90" => Some("--tw-backdrop-hue-rotate: hue-rotate(-90deg);"),
            "-hue-rotate-60" => Some("--tw-backdrop-hue-rotate: hue-rotate(-60deg);"),
            "-hue-rotate-30" => Some("--tw-backdrop-hue-rotate: hue-rotate(-30deg);"),
            "-hue-rotate-15" => Some("--tw-backdrop-hue-rotate: hue-rotate(-15deg);"),
            "hue-rotate-0" => Some("--tw-backdrop-hue-rotate: hue-rotate(0deg);"),
            "hue-rotate-15" => Some("--tw-backdrop-hue-rotate: hue-rotate(15deg);"),
            "hue-rotate-30" => Some("--tw-backdrop-hue-rotate: hue-rotate(30deg);"),
            "hue-rotate-60" => Some("--tw-backdrop-hue-rotate: hue-rotate(60deg);"),
            "hue-rotate-90" => Some("--tw-backdrop-hue-rotate: hue-rotate(90deg);"),
            "hue-rotate-180" => Some("--tw-backdrop-hue-rotate: hue-rotate(180deg);"),
            "invert-0" => Some("--tw-backdrop-invert: invert(0);"),
            "invert" => Some("--tw-backdrop-invert: invert(100%);"),
            "saturate-0" => Some("--tw-backdrop-saturate: saturate(0);"),
            "saturate-50" => Some("--tw-backdrop-saturate: saturate(.5);"),
            "saturate-100" => Some("--tw-backdrop-saturate: saturate(1);"),
            "saturate-150" => Some("--tw-backdrop-saturate: saturate(1.5);"),
            "saturate-200" => Some("--tw-backdrop-saturate: saturate(2);"),
            "sepia-0" => Some("--tw-backdrop-sepia: sepia(0);"),
            "sepia" => Some("--tw-backdrop-sepia: sepia(100%);"),
            _ => None,
        };

        result.map(|result| format!("{}
  {}", result, CSS_BACKDROP_FILTER))
    }
}
