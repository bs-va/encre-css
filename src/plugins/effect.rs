/*use super::SelectorList;

pub fn init(selectors: &mut SelectorList) {
    selectors.register(
        "shadow-xs",
        "box-shadow: 0 0 0 1px rgba(0, 0, 0, 0.05);".to_string(),
    );
    selectors.register(
        "shadow-sm",
        "box-shadow: 0 1px 2px 0 rgba(0, 0, 0, 0.05);".to_string(),
    );
    selectors.register(
        "shadow",
        "box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.1), 0 1px 2px 0 rgba(0, 0, 0, 0.06);".to_string(),
    );
    selectors.register(
        "shadow-md",
        "box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);"
            .to_string(),
    );
    selectors.register(
        "shadow-lg",
        "box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.1), 0 4px 6px -2px rgba(0, 0, 0, 0.05);"
            .to_string(),
    );
    selectors.register(
        "shadow-xl",
        "box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04);"
            .to_string(),
    );
    selectors.register(
        "shadow-2xl",
        "box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);".to_string(),
    );
    selectors.register(
        "shadow-inner",
        "box-shadow: inset 0 2px 4px 0 rgba(0, 0, 0, 0.06);".to_string(),
    );
    selectors.register(
        "shadow-outline",
        "box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.5);".to_string(),
    );
    selectors.register("shadow-none", "box-shadow: none;".to_string());
    selectors.register("opacity-0", "opacity: 0;".to_string());
    selectors.register("opacity-5", "opacity: 0.05;".to_string());
    selectors.register("opacity-10", "opacity: 0.1;".to_string());
    selectors.register("opacity-20", "opacity: 0.2;".to_string());
    selectors.register("opacity-25", "opacity: 0.25;".to_string());
    selectors.register("opacity-30", "opacity: 0.3;".to_string());
    selectors.register("opacity-40", "opacity: 0.4;".to_string());
    selectors.register("opacity-50", "opacity: 0.5;".to_string());
    selectors.register("opacity-60", "opacity: 0.6;".to_string());
    selectors.register("opacity-70", "opacity: 0.7;".to_string());
    selectors.register("opacity-75", "opacity: 0.75;".to_string());
    selectors.register("opacity-80", "opacity: 0.8;".to_string());
    selectors.register("opacity-90", "opacity: 0.9;".to_string());
    selectors.register("opacity-100", "opacity: 1;".to_string());
    selectors.register("mix-blend-normal", "mix-blend-mode: normal;".to_string());
    selectors.register(
        "mix-blend-multiply",
        "mix-blend-mode: multiply;".to_string(),
    );
    selectors.register("mix-blend-screen", "mix-blend-mode: screen;".to_string());
    selectors.register("mix-blend-overlay", "mix-blend-mode: overlay;".to_string());
    selectors.register("mix-blend-darken", "mix-blend-mode: darken;".to_string());
    selectors.register("mix-blend-lighten", "mix-blend-mode: lighten;".to_string());
    selectors.register(
        "mix-blend-color-dodge",
        "mix-blend-mode: color-dodge;".to_string(),
    );
    selectors.register(
        "mix-blend-color-burn",
        "mix-blend-mode: color-burn;".to_string(),
    );
    selectors.register(
        "mix-blend-hard-light",
        "mix-blend-mode: hard-light;".to_string(),
    );
    selectors.register(
        "mix-blend-soft-light",
        "mix-blend-mode: soft-light;".to_string(),
    );
    selectors.register(
        "mix-blend-difference",
        "mix-blend-mode: difference;".to_string(),
    );
    selectors.register(
        "mix-blend-exclusion",
        "mix-blend-mode: exclusion;".to_string(),
    );
    selectors.register("mix-blend-hue", "mix-blend-mode: hue;".to_string());
    selectors.register(
        "mix-blend-saturation",
        "mix-blend-mode: saturation;".to_string(),
    );
    selectors.register("mix-blend-color", "mix-blend-mode: color;".to_string());
    selectors.register(
        "mix-blend-luminosity",
        "mix-blend-mode: luminosity;".to_string(),
    );
    selectors.register(
        "bg-blend-normal",
        "background-blend-mode: normal;".to_string(),
    );
    selectors.register(
        "bg-blend-multiply",
        "background-blend-mode: multiply;".to_string(),
    );
    selectors.register(
        "bg-blend-screen",
        "background-blend-mode: screen;".to_string(),
    );
    selectors.register(
        "bg-blend-overlay",
        "background-blend-mode: overlay;".to_string(),
    );
    selectors.register(
        "bg-blend-darken",
        "background-blend-mode: darken;".to_string(),
    );
    selectors.register(
        "bg-blend-lighten",
        "background-blend-mode: lighten;".to_string(),
    );
    selectors.register(
        "bg-blend-color-dodge",
        "background-blend-mode: color-dodge;".to_string(),
    );
    selectors.register(
        "bg-blend-color-burn",
        "background-blend-mode: color-burn;".to_string(),
    );
    selectors.register(
        "bg-blend-hard-light",
        "background-blend-mode: hard-light;".to_string(),
    );
    selectors.register(
        "bg-blend-soft-light",
        "background-blend-mode: soft-light;".to_string(),
    );
    selectors.register(
        "bg-blend-difference",
        "background-blend-mode: difference;".to_string(),
    );
    selectors.register(
        "bg-blend-exclusion",
        "background-blend-mode: exclusion;".to_string(),
    );
    selectors.register("bg-blend-hue", "background-blend-mode: hue;".to_string());
    selectors.register(
        "bg-blend-saturation",
        "background-blend-mode: saturation;".to_string(),
    );
    selectors.register(
        "bg-blend-color",
        "background-blend-mode: color;".to_string(),
    );
    selectors.register(
        "bg-blend-luminosity",
        "background-blend-mode: luminosity;".to_string(),
    );
}*/
