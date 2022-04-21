/*use super::SelectorList;

pub fn init(selectors: &mut SelectorList) {
    selectors.register("filter", "".to_string());
    selectors.register("filter-none", "".to_string());
    selectors.register("blur-0", "".to_string());
    selectors.register("blur-sm", "".to_string());
    selectors.register("blur", "".to_string());
    selectors.register("blur-md", "".to_string());
    selectors.register("blur-lg", "".to_string());
    selectors.register("blur-xl", "".to_string());
    selectors.register("blur-2xl", "".to_string());
    selectors.register("blur-3xl", "".to_string());
    selectors.register("brightness-0", "".to_string());
    selectors.register("brightness-50", "".to_string());
    selectors.register("brightness-75", "".to_string());
    selectors.register("brightness-90", "".to_string());
    selectors.register("brightness-95", "".to_string());
    selectors.register("brightness-100", "".to_string());
    selectors.register("brightness-105", "".to_string());
    selectors.register("brightness-110", "".to_string());
    selectors.register("brightness-125", "".to_string());
    selectors.register("brightness-150", "".to_string());
    selectors.register("brightness-200", "".to_string());
    selectors.register("contrast-0", "".to_string());
    selectors.register("contrast-50", "".to_string());
    selectors.register("contrast-75", "".to_string());
    selectors.register("contrast-100", "".to_string());
    selectors.register("contrast-125", "".to_string());
    selectors.register("contrast-150", "".to_string());
    selectors.register("contrast-200", "".to_string());
    selectors.register(
        "drop-shadow-sm",
        "drop-shadow: drop-shadow(0 1px 1px rgba(0,0,0,0.05));".to_string(),
    );
    selectors.register("drop-shadow", "drop-shadow: drop-shadow(0 1px 2px rgba(0, 0, 0, 0.1)) drop-shadow(0 1px 1px rgba(0, 0, 0, 0.06));".to_string());
    selectors.register("drop-shadow-md", "drop-shadow: drop-shadow(0 4px 3px rgba(0, 0, 0, 0.07)) drop-shadow(0 2px 2px rgba(0, 0, 0, 0.06));".to_string());
    selectors.register("drop-shadow-lg", "drop-shadow: drop-shadow(0 10px 8px rgba(0, 0, 0, 0.04)) drop-shadow(0 4px 3px rgba(0, 0, 0, 0.1));".to_string());
    selectors.register("drop-shadow-xl", "drop-shadow: drop-shadow(0 20px 13px rgba(0, 0, 0, 0.03)) drop-shadow(0 8px 5px rgba(0, 0, 0, 0.08));".to_string());
    selectors.register(
        "drop-shadow-2xl",
        "drop-shadow: drop-shadow: drop-shadow(0 25px 25px rgba(0, 0, 0, 0.15));".to_string(),
    );
    selectors.register(
        "drop-shadow-none",
        "drop-shadow: drop-shadow: drop-shadow(0 0 #0000);".to_string(),
    );
    selectors.register("grayscale-0", "--tw-grayscale: grayscale(0); filter: var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow);".to_string());
    selectors.register("grayscale", "--tw-grayscale: grayscale(100%); filter: var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia) var(--tw-drop-shadow);".to_string());
    selectors.register(
        "-hue-rotate-180",
        "hue-rotate: hue-rotate(-180deg);".to_string(),
    );
    selectors.register(
        "-hue-rotate-90",
        "hue-rotate: hue-rotate(-90deg);".to_string(),
    );
    selectors.register(
        "-hue-rotate-60",
        "hue-rotate: hue-rotate(-60deg);".to_string(),
    );
    selectors.register(
        "-hue-rotate-30",
        "hue-rotate: hue-rotate(-30deg);".to_string(),
    );
    selectors.register(
        "-hue-rotate-15",
        "hue-rotate: hue-rotate(-15deg);".to_string(),
    );
    selectors.register("hue-rotate-0", "hue-rotate: hue-rotate(0deg);".to_string());
    selectors.register(
        "hue-rotate-15",
        "hue-rotate: hue-rotate(15deg);".to_string(),
    );
    selectors.register(
        "hue-rotate-30",
        "hue-rotate: hue-rotate(30deg);".to_string(),
    );
    selectors.register(
        "hue-rotate-60",
        "hue-rotate: hue-rotate(60deg);".to_string(),
    );
    selectors.register(
        "hue-rotate-90",
        "hue-rotate: hue-rotate(90deg);".to_string(),
    );
    selectors.register(
        "hue-rotate-180",
        "hue-rotate: hue-rotate(180deg);".to_string(),
    );
    selectors.register("invert-0", "invert: invert(0);".to_string());
    selectors.register("invert", "invert: invert(1);".to_string());
    selectors.register("saturate-0", "".to_string());
    selectors.register("saturate-50", "".to_string());
    selectors.register("saturate-100", "".to_string());
    selectors.register("saturate-150", "".to_string());
    selectors.register("saturate-200", "".to_string());
    selectors.register("sepia-0", "".to_string());
    selectors.register("sepia", "".to_string());
    selectors.register("backdrop-filter", "".to_string());
    selectors.register("backdrop-filter-none", "".to_string());
    selectors.register("backdrop-blur-0", "".to_string());
    selectors.register("backdrop-blur-sm", "".to_string());
    selectors.register("backdrop-blur", "".to_string());
    selectors.register("backdrop-blur-md", "".to_string());
    selectors.register("backdrop-blur-lg", "".to_string());
    selectors.register("backdrop-blur-xl", "".to_string());
    selectors.register("backdrop-blur-2xl", "".to_string());
    selectors.register("backdrop-blur-3xl", "".to_string());
    selectors.register("backdrop-brightness-0", "".to_string());
    selectors.register("backdrop-brightness-sm", "".to_string());
    selectors.register("backdrop-brightness", "".to_string());
    selectors.register("backdrop-brightness-md", "".to_string());
    selectors.register("backdrop-brightness-lg", "".to_string());
    selectors.register("backdrop-brightness-xl", "".to_string());
    selectors.register("backdrop-brightness-2xl", "".to_string());
    selectors.register("backdrop-brightness-3xl", "".to_string());
    selectors.register("backdrop-contrast-0", "".to_string());
    selectors.register("backdrop-contrast-50", "".to_string());
    selectors.register("backdrop-contrast-75", "".to_string());
    selectors.register("backdrop-contrast-100", "".to_string());
    selectors.register("backdrop-contrast-125", "".to_string());
    selectors.register("backdrop-contrast-150", "".to_string());
    selectors.register("backdrop-contrast-200", "".to_string());
    selectors.register("backdrop-grayscale-0", "".to_string());
    selectors.register("backdrop-grayscale", "".to_string());
    selectors.register("-backdrop-hue-rotate-180", "".to_string());
    selectors.register("-backdrop-hue-rotate-90", "".to_string());
    selectors.register("-backdrop-hue-rotate-60", "".to_string());
    selectors.register("-backdrop-hue-rotate-30", "".to_string());
    selectors.register("-backdrop-hue-rotate-15", "".to_string());
    selectors.register("backdrop-hue-rotate-0", "".to_string());
    selectors.register("backdrop-hue-rotate-15", "".to_string());
    selectors.register("backdrop-hue-rotate-30", "".to_string());
    selectors.register("backdrop-hue-rotate-60", "".to_string());
    selectors.register("backdrop-hue-rotate-90", "".to_string());
    selectors.register("backdrop-hue-rotate-180", "".to_string());
    selectors.register("backdrop-invert-0", "".to_string());
    selectors.register("backdrop-invert", "".to_string());
    selectors.register("backdrop-opacity-0", "".to_string());
    selectors.register("backdrop-opacity-5", "".to_string());
    selectors.register("backdrop-opacity-10", "".to_string());
    selectors.register("backdrop-opacity-20", "".to_string());
    selectors.register("backdrop-opacity-25", "".to_string());
    selectors.register("backdrop-opacity-30", "".to_string());
    selectors.register("backdrop-opacity-40", "".to_string());
    selectors.register("backdrop-opacity-50", "".to_string());
    selectors.register("backdrop-opacity-60", "".to_string());
    selectors.register("backdrop-opacity-70", "".to_string());
    selectors.register("backdrop-opacity-75", "".to_string());
    selectors.register("backdrop-opacity-80", "".to_string());
    selectors.register("backdrop-opacity-90", "".to_string());
    selectors.register("backdrop-opacity-95", "".to_string());
    selectors.register("backdrop-opacity-100", "".to_string());
    selectors.register("backdrop-saturate-0", "".to_string());
    selectors.register("backdrop-saturate-50", "".to_string());
    selectors.register("backdrop-saturate-100", "".to_string());
    selectors.register("backdrop-saturate-150", "".to_string());
    selectors.register("backdrop-saturate-200", "".to_string());
    selectors.register("backdrop-sepia-0", "".to_string());
    selectors.register("backdrop-sepia", "".to_string());
}*/
