//! Define the main [`EncreGenerator`] structure used to scan content and to generate CSS styles.
use crate::{
    config::Config,
    plugins::transition::animation,
    preflight::Preflight,
    selector::{parse, Modifier, Selector, Variant, VariantType},
    utils::{indent, unindent},
};

use std::{
    collections::BTreeSet,
    fmt::{self, Write},
    path::Path,
    sync::{atomic::Ordering, Arc},
};

/// The context used in the [`Plugin::can_handle`] method.
///
/// [`Plugin::can_handle`]: crate::plugins::Plugin::can_handle
#[derive(Debug)]
pub struct ContextCanHandle<'a, 'b, 'c> {
    /// The generator's configuration.
    pub config: &'a Config,

    /// The modifier which will be checked.
    pub modifier: &'b Modifier<'c>,
}

/// The context used in the [`Plugin::handle`] method.
///
/// [`Plugin::handle`]: crate::plugins::Plugin::handle
#[derive(Debug)]
pub struct ContextHandle<'a, 'b, 'c, 'd, 'e> {
    /// The generator's configuration.
    pub config: &'a Config,

    /// The modifier which will have its CSS generated.
    pub modifier: &'b Modifier<'c>,

    /// The current indentation of the CSS rule.
    pub indentation: String,

    /// The buffer containing the whole generated CSS.
    pub buffer: &'d mut String,

    // Private fields used in `generate_rule`
    selector: &'e Selector<'e>,
}

/// Generate the needed CSS at-rules (e.g @media).
///
/// Note: The inner class (e.g. .foo-bar) is not handled by this function, see [`generate_wrapper`].
///
/// The second argument, a closure, is called to generate the CSS content of the rule.
///
/// # Errors
///
/// Returns [`fmt::Error`] indicating whether writing to the buffer succeeded.
///
/// [`fmt::Error`]: std::fmt::Error
pub fn generate_at_rules<T: FnOnce(&mut ContextHandle) -> fmt::Result>(
    context: &mut ContextHandle,
    rule_content_fn: T,
) -> fmt::Result {
    let ContextHandle { indentation, buffer, selector, .. } = context;

    if !selector.variants.is_empty() {
        selector.variants.iter().try_for_each(|variant| {
            match variant {
                Variant::Builtin(_, VariantType::AtRule(variant)) => {
                    writeln!(buffer, "{indentation}{} {{", variant)?;
                    indent(indentation);
                }
                Variant::Arbitrary(variant) if variant.starts_with('@') => {
                    writeln!(buffer, "{indentation}{} {{", variant)?;
                    indent(indentation);
                }
                _ => (),
            }

            Ok::<(), fmt::Error>(())
        })?;
    }

    rule_content_fn(context)?;

    let ContextHandle { indentation, buffer, .. } = context;
    while !indentation.is_empty() {
        unindent(indentation);

        if indentation.is_empty() {
            write!(buffer, "}}")?;
        } else {
            writeln!(buffer, "{indentation}}}")?;
        }
    }

    Ok(())
}

/// Generate a CSS rule with a class.
///
/// Note: At-rules (e.g. @media) are not handled by this function, see [`generate_wrapper`].
///
/// The second argument, a closure, is called to generate the CSS content of the rule.
/// The third argument is used to add a custom string just after the class (e.g. `> *`).
///
/// # Errors
///
/// Returns [`fmt::Error`] indicating whether writing to the buffer succeeded.
///
/// [`fmt::Error`]: std::fmt::Error
#[allow(clippy::too_many_lines)]
pub fn generate_class<T: FnOnce(&mut ContextHandle) -> fmt::Result>(
    context: &mut ContextHandle,
    rule_content_fn: T,
    custom_after_class: &str,
) -> fmt::Result {
    let ContextHandle { indentation, buffer, selector, .. } = context;

    // Write the class
    let mut base_class = ".".to_string()
        + &selector
            .full
            .chars()
            .enumerate()
            .map(|(i, ch)| {
                if !ch.is_alphanumeric() && ch != '-' && ch != '_' {
                    format!("\\{}", ch)
                } else if i == 0 && ch.is_numeric() {
                    // CSS classes must not start with a number, we need to escape it
                    "\\3".to_string() + &ch.to_string()
                } else {
                    ch.to_string()
                }
            })
            .collect::<String>();

    if !selector.variants.is_empty() {
       selector
            .variants
            .iter()
            .rev()
            .for_each(|variant| match variant {
                Variant::Builtin(_, variant) => match variant {
                    VariantType::PseudoElement(element) => {
                        write!(base_class, "::{}", element)
                            .expect("writing to a String can't fail");
                    }
                    VariantType::PseudoClass(class) => {
                        write!(base_class, ":{}", class).expect("writing to a String can't fail");
                    }
                    VariantType::WrapClass(template) => {
                        base_class = template.replace('&', &base_class);
                    }
                    VariantType::AtRule(_) => (),
                    VariantType::Group(class) => {
                        base_class = format!(".group:{class} {base_class}");
                    }
                    VariantType::Peer(class) => {
                        base_class = format!(".peer:{class} ~ {base_class}");
                    }
                    VariantType::PeerNot(class) => {
                        base_class = format!(".peer:not(:{class}) ~ {base_class}");
                    }
                },
                Variant::Arbitrary(template) if !template.starts_with('@') => {
                    base_class = template.replace('&', &base_class);
                }
                Variant::Arbitrary(_) => (),
            });
    }
    writeln!(buffer, "{indentation}{base_class}{} {{", custom_after_class)?;

    // Store the index of the start of the class content (useful when the `important` flag is present)
    let content_start = buffer.len();

    // Rule content
    indent(indentation);
    rule_content_fn(context)?;

    let ContextHandle { indentation, buffer, selector, .. } = context;

    // If the rule is selecting the `::before` or `::after` pseudo elements, we need to generate a
    // default `content` property
    if selector.variants.iter().any(|variant| {
        if let Variant::Builtin(_, variant) = variant {
            *variant == VariantType::PseudoElement("before")
                || *variant == VariantType::PseudoElement("after")
        } else {
            false
        }
    }) {
        writeln!(buffer, "{indentation}content: var(--en-content);")?;
    }

    // If the `important` flag is present we need to replace all `;\n` or `;\r\n`
    // to ` !important;\n` or ` !important;\r\n`
    if selector.is_important {
        let mut extra_index = 0;
        let positions = buffer[content_start..]
            .match_indices('\n')
            .map(|i| i.0)
            .collect::<Vec<usize>>();

        for index in positions {
            if index - 1 == 0 {
                continue;
            }

            let index = content_start + extra_index + index;
            let index = if &buffer[index - 1..index] == "\r" {
                index - 1
            } else {
                index
            };
            let replace_with = " !important;";
            buffer.replace_range(index - 1..index, replace_with);
            extra_index += replace_with.len() - 1;
        }
    }

    unindent(indentation);
    if indentation.is_empty() {
        write!(buffer, "}}")?;
    } else {
        writeln!(buffer, "{indentation}}}")?;
    }

    Ok(())
}

/// Generate the complete CSS wrapper needed for a single rule.
///
/// This function is a combination of the [`generate_at_rules`] and [`generate_class`] functions.
///
/// The second argument, a closure, is called to generate the CSS content of the rule.
///
/// # Errors
///
/// Returns [`fmt::Error`] indicating whether writing to the buffer succeeded.
///
/// [`fmt::Error`]: std::fmt::Error
pub fn generate_wrapper<T: FnOnce(&mut ContextHandle) -> fmt::Result>(
    context: &mut ContextHandle,
    rule_content_fn: T,
) -> fmt::Result {
    generate_at_rules(context, |context| {
        generate_class(context, rule_content_fn, "")
    })
}

/// Main structure used to generate CSS from selectors.
///
/// Please note that the scanned content **must live as long as the [`EncreGenerator`] structure**,
/// but because it is pretty cheap to make one (it just stores scanned atomic classes), it is
/// recommended to make a new one (or clone it) each time you call [`EncreGenerator::generate`] and
/// to pass an `Arc<Config>` to it (to avoid cloning the configuration).
#[derive(Debug, Clone)]
pub struct EncreGenerator<'a> {
    config: Arc<Config>,
    pub(crate) scanned_selectors: BTreeSet<Selector<'a>>,
}

impl<'a> EncreGenerator<'a> {
    /// Create a new [`EncreGenerator`] by trying to read a configuration file.
    ///
    /// If the file does not exist, a warning will be emitted and the default configuration will be
    /// used.
    pub fn new<T: AsRef<Path>>(path: T) -> Self {
        let config = match Config::from_file(path) {
            Ok(config) => config,
            Err(e) => {
                eprintln!("{}", e);
                Config::default()
            }
        };

        Self::from_config(config)
    }

    /// Create a new [`EncreGenerator`] using a given configuration.
    ///
    /// The configuration can either be a [`Config`] structure or an [`Arc<Config>`].
    pub fn from_config<T: Into<Arc<Config>>>(config: T) -> Self {
        let config = config.into();
        Self {
            config,
            scanned_selectors: BTreeSet::new(),
        }
    }

    /// Add a single selector which will have its CSS generated.
    ///
    /// You need to use this function if you already have scanned a selector, otherwise use
    /// [scan].
    ///
    /// This function automatically handles duplicated selectors and sorting.
    ///
    /// [scan]: EncreGenerator::scan
    pub fn add_selector(&mut self, val: &'a str) {
        self.scanned_selectors.extend(
            parse(val, None, &self.config)
                .into_iter()
                .filter_map(Result::ok),
        );
    }

    /// Add several selectors which will have their CSS generated.
    ///
    /// You need to use this function if you already have scanned a list of selectors, otherwise use
    /// [scan].
    ///
    /// This function automatically handles duplicated selectors and sorting.
    ///
    /// [scan]: EncreGenerator::scan
    pub fn add_selectors<T: IntoIterator<Item = &'a str>>(&mut self, val: T) {
        self.scanned_selectors.extend(
            val.into_iter()
                .flat_map(|v| parse(v.trim(), None, &self.config))
                .filter_map(Result::ok),
        );
    }

    /// Scan the contents of a file and store all the selectors found.
    ///
    /// By default, it splits the content by spaces, double quotes, single quotes and backticks and
    /// ignores arbitrary values/variants and variant groups.
    ///
    /// This function automatically handles duplicated selectors and sorting.
    pub fn scan(&mut self, content: &'a str) {
        self.add_selectors(self.config.scanner.scan(content));
    }

    /// Generate the CSS styles needed based on the scanned selectors.
    ///
    /// Don't forget to scan selectors before, using:
    /// - [`add_selector`] to add a single selector to the scanned list;
    /// - [`add_selectors`] to add a list of selectors to the scanned list;
    /// - [`scan`] to scan a string (e.g. the contents of a file).
    ///
    /// [`add_selector`]: EncreGenerator::add_selector
    /// [`add_selectors`]: EncreGenerator::add_selectors
    /// [`scan`]: EncreGenerator::scan
    pub fn generate(&self) -> String {
        // Make sure that animations are not defined
        animation::ANIMATIONS_ALREADY_DEFINED
            .iter()
            .for_each(|animation| animation.store(false, Ordering::Relaxed));

        let preflight = self.config.preflight.build();
        let mut buffer = String::with_capacity(10 * self.scanned_selectors.len()); // TODO: More accurate value
        buffer.push_str(&preflight); // TODO: Push and reserve at the same time

        self.scanned_selectors.iter().for_each(|selector| {
            if buffer.len() != preflight.len() || self.config.preflight != Preflight::None {
                write!(buffer, "\n\n").expect("writing to a String can't fail");
            }

            let mut context = ContextHandle {
                config: &self.config,
                modifier: &selector.modifier,
                indentation: String::new(),
                buffer: &mut buffer,
                selector,
            };

            if selector.plugin.needs_wrapping() {
                generate_wrapper(&mut context, |context| selector.plugin.handle(context))
            } else {
                selector.plugin.handle(&mut context)
            }
            .expect("writing to a String can't fail");
        });

        buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::DarkMode;

    use pretty_assertions::assert_eq;
    use std::collections::BTreeSet;

    fn base_config() -> Config {
        // Disable the preflight to simplify test assertions
        let mut config = Config::default();
        config.preflight = Preflight::None;
        config
    }

    #[test]
    fn simple_scan() {
        let config = base_config();
        let expected = BTreeSet::from([
            parse("flex", None, &config)[0].as_ref().unwrap().clone(),
            parse("w-full", None, &config)[0].as_ref().unwrap().clone(),
            parse("h-full", None, &config)[0].as_ref().unwrap().clone(),
            parse("absolute", None, &config)[0]
                .as_ref()
                .unwrap()
                .clone(),
            parse("bg-blue-500", None, &config)[0]
                .as_ref()
                .unwrap()
                .clone(),
            parse("border-[#333]", None, &config)[0]
                .as_ref()
                .unwrap()
                .clone(),
            parse("text-[color:var(--hello)]", None, &config)[0]
                .as_ref()
                .unwrap()
                .clone(),
            parse("sm:focus:ring", None, &config)[0]
                .as_ref()
                .unwrap()
                .clone(),
            parse("hover:bg-black", None, &config)[0]
                .as_ref()
                .unwrap()
                .clone(),
        ]);

        let mut generator = EncreGenerator::from_config(base_config());
        generator.scan(
            r#"<div class="flex w-full h-full absolute bg-blue-500 foo-bar sm:focus:ring hover:bg-black border-[#333] text-[color:var(--hello)]"></div>"#
        );

        assert_eq!(expected, generator.scanned_selectors);
    }

    #[test]
    fn utf8_scan() {
        let config = base_config();
        let expected = BTreeSet::from([
            parse("before:content-[J\u{e4}s\u{f8}n_Doe]", None, &config)[0]
                .as_ref()
                .unwrap()
                .clone(),
            parse("content-[\u{2192}]", None, &config)[0]
                .as_ref()
                .unwrap()
                .clone(),
        ]);

        let mut generator = EncreGenerator::from_config(base_config());
        generator.scan(
            "<div class=\"before:content-[J\u{e4}s\u{f8}n_Doe] content-[\u{2192}]\">\u{306}</div>",
        );

        assert_eq!(expected, generator.scanned_selectors);
    }

    #[test]
    fn scan_prevent_splitting_arbitrary_values() {
        let config = base_config();
        let expected = BTreeSet::from([
            parse("bg-red-300", None, &config)[0]
                .as_ref()
                .unwrap()
                .clone(),
            parse("content-['hello']", None, &config)[0]
                .as_ref()
                .unwrap()
                .clone(),
        ]);

        let mut generator = EncreGenerator::from_config(base_config());
        generator.scan(r#"<div class="bg-red-300 content-['hello']"></div>"#);

        assert_eq!(expected, generator.scanned_selectors);
    }

    #[test]
    fn divide_and_space_between_special_class() {
        let mut generator = EncreGenerator::from_config(base_config());
        generator.add_selector("hover:space-x-1");
        generator.add_selector("space-x-2");
        generator.add_selector("[&:has(.class)_>_*]:space-y-3");
        generator.add_selector("divide-red-100");
        generator.add_selector("divide-dashed");
        generator.add_selector("divide-x-[11px]");
        generator.add_selector("xl:[&_>_*]:divide-y-2");

        assert_eq!(
            generator.generate(),
            String::from(
                r#".space-x-2 > :not([hidden]) ~ :not([hidden]) {
  --en-space-x-reverse: 0;
  margin-right: calc(0.5rem * var(--en-space-x-reverse));
  margin-left: calc(0.5rem * calc(1 - var(--en-space-x-reverse)));
}

.divide-x-\[11px\] > :not([hidden]) ~ :not([hidden]) {
  --en-divide-x-reverse: 0;
  border-right-width: calc(11px * var(--en-divide-x-reverse));
  border-left-width: calc(11px * calc(1 - var(--en-divide-x-reverse)));
}

.divide-dashed > :not([hidden]) ~ :not([hidden]) {
  border-style: dashed;
}

.divide-red-100 > :not([hidden]) ~ :not([hidden]) {
  --en-divide-opacity: 1;
  border-color: rgb(254 226 226 / var(--en-divide-opacity));
}

.hover\:space-x-1:hover > :not([hidden]) ~ :not([hidden]) {
  --en-space-x-reverse: 0;
  margin-right: calc(0.25rem * var(--en-space-x-reverse));
  margin-left: calc(0.25rem * calc(1 - var(--en-space-x-reverse)));
}

@media (min-width: 1280px) {
  .xl\:\[\&_\>_\*\]\:divide-y-2 > * > :not([hidden]) ~ :not([hidden]) {
    --en-divide-y-reverse: 0;
    border-top-width: calc(2px * calc(1 - var(--en-divide-y-reverse)));
    border-bottom-width: calc(2px * var(--en-divide-y-reverse));
  }
}

.\[\&\:has\(\.class\)_\>_\*\]\:space-y-3:has(.class) > * > :not([hidden]) ~ :not([hidden]) {
  --en-space-y-reverse: 0;
  margin-top: calc(0.75rem * calc(1 - var(--en-space-y-reverse)));
  margin-bottom: calc(0.75rem * var(--en-space-y-reverse));
}"#
            )
        );
    }

    #[test]
    fn negative_values() {
        let mut generator = EncreGenerator::from_config(base_config());
        generator.add_selector("-top-2");
        generator.add_selector("-z-2");
        generator.add_selector("-order-2");
        generator.add_selector("-mb8");
        generator.add_selector("-translate-x-52");
        generator.add_selector("-rotate-90");
        generator.add_selector("-skew-x-2");
        generator.add_selector("-scale-50");
        generator.add_selector("-scroll-mt-2");
        generator.add_selector("-space-x-2");
        generator.add_selector("-indent-2");
        generator.add_selector("-hue-rotate-60");
        generator.add_selector("hover:-hue-rotate-60");
        generator.add_selector("-backdrop-hue-rotate-90");

        assert_eq!(
            generator.generate(),
            String::from(
                r#".-top-2 {
  top: -0.5rem;
}

.-z-2 {
  z-index: -2;
}

.-order-2 {
  order: -2;
}

.-mb8 {
  margin-bottom: -2rem;
}

.-translate-x-52 {
  --en-translate-x: -13rem;
  transform: translate(var(--en-translate-x), var(--en-translate-y)) rotate(var(--en-rotate)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scaleX(var(--en-scale-x)) scaleY(var(--en-scale-y));
}

.-rotate-90 {
  --en-rotate: -90deg;
  transform: translate(var(--en-translate-x), var(--en-translate-y)) rotate(var(--en-rotate)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scaleX(var(--en-scale-x)) scaleY(var(--en-scale-y));
}

.-skew-x-2 {
  --en-skew-x: -2deg;
  transform: translate(var(--en-translate-x), var(--en-translate-y)) rotate(var(--en-rotate)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scaleX(var(--en-scale-x)) scaleY(var(--en-scale-y));
}

.-scale-50 {
  --en-scale-x: -0.5;
  --en-scale-y: -0.5;
  transform: translate(var(--en-translate-x), var(--en-translate-y)) rotate(var(--en-rotate)) skewX(var(--en-skew-x)) skewY(var(--en-skew-y)) scaleX(var(--en-scale-x)) scaleY(var(--en-scale-y));
}

.-scroll-mt-2 {
  scroll-margin-top: -0.5rem;
}

.-space-x-2 > :not([hidden]) ~ :not([hidden]) {
  --en-space-x-reverse: 0;
  margin-right: calc(-0.5rem * var(--en-space-x-reverse));
  margin-left: calc(-0.5rem * calc(1 - var(--en-space-x-reverse)));
}

.-indent-2 {
  text-indent: -0.5rem;
}

.-hue-rotate-60 {
  --en-hue-rotate: hue-rotate(-60deg);
  filter: var(--en-blur) var(--en-brightness) var(--en-contrast) var(--en-grayscale) var(--en-hue-rotate) var(--en-invert) var(--en-saturate) var(--en-sepia) var(--en-drop-shadow);
}

.-backdrop-hue-rotate-90 {
  --en-backdrop-hue-rotate: hue-rotate(-90deg);
  -webkit-backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
  backdrop-filter: var(--en-backdrop-blur) var(--en-backdrop-brightness) var(--en-backdrop-contrast) var(--en-backdrop-grayscale) var(--en-backdrop-hue-rotate) var(--en-backdrop-invert) var(--en-backdrop-opacity) var(--en-backdrop-saturate) var(--en-backdrop-sepia);
}

.hover\:-hue-rotate-60:hover {
  --en-hue-rotate: hue-rotate(-60deg);
  filter: var(--en-blur) var(--en-brightness) var(--en-contrast) var(--en-grayscale) var(--en-hue-rotate) var(--en-invert) var(--en-saturate) var(--en-sepia) var(--en-drop-shadow);
}"#
            )
        );
    }

    #[test]
    fn gen_css_for_simple_selector() {
        let mut generator = EncreGenerator::from_config(base_config());
        generator.add_selector("w-full");

        assert_eq!(
            generator.generate(),
            String::from(
                r#".w-full {
  width: 100%;
}"#,
            )
        );
    }

    #[test]
    fn gen_css_with_important_flag() {
        let mut generator = EncreGenerator::from_config(base_config());
        generator.add_selector("!w-full");
        generator.add_selector("!-mb-8");
        generator.add_selector("!shadow");
        generator.add_selector("!-hue-rotate-60");
        generator.add_selector("focus:!w-2");
        generator.add_selector("focus:!-mb-2");

        assert_eq!(
            generator.generate(),
            String::from(
                r#".\!-mb-8 {
  margin-bottom: -2rem !important;
}

.\!w-full {
  width: 100% !important;
}

.\!shadow {
  --en-shadow: 0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1) !important;
  --en-shadow-colored: 0 1px 3px 0 var(--en-shadow-color), 0 1px 2px -1px var(--en-shadow-color) !important;
  box-shadow: var(--en-ring-offset-shadow, 0 0 #0000), var(--en-ring-shadow, 0 0 #0000), var(--en-shadow) !important;
}

.\!-hue-rotate-60 {
  --en-hue-rotate: hue-rotate(-60deg) !important;
  filter: var(--en-blur) var(--en-brightness) var(--en-contrast) var(--en-grayscale) var(--en-hue-rotate) var(--en-invert) var(--en-saturate) var(--en-sepia) var(--en-drop-shadow) !important;
}

.focus\:\!-mb-2:focus {
  margin-bottom: -0.5rem !important;
}

.focus\:\!w-2:focus {
  width: 0.5rem !important;
}"#,
            )
        );
    }

    #[test]
    fn gen_css_for_selector_needing_custom_css() {
        let mut generator = EncreGenerator::from_config(base_config());
        generator.add_selector("animate-pulse");
        generator.add_selector("animate-pulse");

        assert_eq!(
            generator.generate(),
            String::from(
                r#"@-webkit-keyframes pulse {
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
}

.animate-pulse {
  -webkit-animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
  animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
}"#
            )
        );
    }

    #[test]
    fn gen_css_for_arbitrary_value() {
        let mut generator = EncreGenerator::from_config(base_config());
        generator.add_selector("w[12px]");
        generator.add_selector("bg-[red]");
        generator.add_selector("bg-[url('../img/image_with_underscores.png')]");
        generator.add_selector("mt-[calc(100%-10px)]");
        generator.add_selector("2xl:pb-[calc((100%/2)-10px+2rem)]");

        assert_eq!(
            generator.generate(),
            String::from(
                r#".mt-\[calc\(100\%-10px\)\] {
  margin-top: calc(100% - 10px);
}

.w\[12px\] {
  width: 12px;
}

.bg-\[red\] {
  background-color: red;
}

.bg-\[url\(\'\.\.\/img\/image_with_underscores\.png\'\)\] {
  background-image: url('../img/image_with_underscores.png');
}

@media (min-width: 1536px) {
  .\32xl\:pb-\[calc\(\(100\%\/2\)-10px\+2rem\)\] {
    padding-bottom: calc((100% / 2) - 10px + 2rem);
  }
}"#
            )
        );
    }

    #[test]
    fn gen_css_for_arbitrary_value_with_hint() {
        let mut generator = EncreGenerator::from_config(base_config());
        generator.add_selector("bg-[color:red]");
        generator.add_selector("hover:bg-[color:red]");

        assert_eq!(
            generator.generate(),
            String::from(
                r#".bg-\[color\:red\] {
  background-color: red;
}

.hover\:bg-\[color\:red\]:hover {
  background-color: red;
}"#
            )
        );
    }

    #[test]
    fn gen_css_for_selector_with_simple_variant() {
        let mut generator = EncreGenerator::from_config(base_config());
        generator.add_selector("focus:w-full");

        assert_eq!(
            generator.generate(),
            String::from(
                r#".focus\:w-full:focus {
  width: 100%;
}"#
            )
        );
    }

    #[test]
    fn gen_selector_css_variants_test() {
        let mut generator = EncreGenerator::from_config(base_config());
        generator.add_selector("sm:hover:bg-red-400");
        generator.add_selector("focus:hover:bg-red-600");
        generator.add_selector("active:rtl:bg-red-800");
        generator.add_selector("md:focus:selection:bg-blue-100");
        generator.add_selector("rtl:active:focus:lg:underline");
        generator.add_selector("print:ltr:xl:hover:focus:active:text-yellow-300");
        generator.add_selector("2xl:motion-safe:landscape:focus-within:visited:first:odd:checked:open:rtl:bg-purple-100");
        generator.add_selector("hover:file:bg-pink-600");
        generator.add_selector("file:hover:bg-pink-600");
        generator.add_selector("sm:before:target:content-['Hello_world!']");
        generator.add_selector("marker:selection:hover:bg-green-200");
        generator.add_selector("group-hover:bg-green-300");
        generator.add_selector("group-focus:bg-green-400");
        generator.add_selector("peer-invalid:bg-red-500");
        generator.add_selector("peer-not-invalid:bg-green-500");

        assert_eq!(
            generator.generate(),
            String::from(
                r#".marker\:selection\:hover\:bg-green-200:hover *::selection, .marker\:selection\:hover\:bg-green-200:hover::selection *::marker, .marker\:selection\:hover\:bg-green-200:hover *::selection, .marker\:selection\:hover\:bg-green-200:hover::selection::marker {
  --en-bg-opacity: 1;
  background-color: rgb(187 247 208 / var(--en-bg-opacity));
}

.file\:hover\:bg-pink-600:hover::file-selector-button, .file\:hover\:bg-pink-600:hover::-webkit-file-upload-button {
  --en-bg-opacity: 1;
  background-color: rgb(219 39 119 / var(--en-bg-opacity));
}

.hover\:file\:bg-pink-600::file-selector-button, .hover\:file\:bg-pink-600::-webkit-file-upload-button:hover {
  --en-bg-opacity: 1;
  background-color: rgb(219 39 119 / var(--en-bg-opacity));
}

.focus\:hover\:bg-red-600:hover:focus {
  --en-bg-opacity: 1;
  background-color: rgb(220 38 38 / var(--en-bg-opacity));
}

[dir="rtl"] .active\:rtl\:bg-red-800:active {
  --en-bg-opacity: 1;
  background-color: rgb(153 27 27 / var(--en-bg-opacity));
}

@media (min-width: 1024px) {
  [dir="rtl"] .rtl\:active\:focus\:lg\:underline:focus:active {
    -webkit-text-decoration-line: underline;
    text-decoration-line: underline;
  }
}

@media print {
  @media (min-width: 1280px) {
    [dir="ltr"] .print\:ltr\:xl\:hover\:focus\:active\:text-yellow-300:active:focus:hover {
      --en-text-opacity: 1;
      color: rgb(253 224 71 / var(--en-text-opacity));
    }
  }
}

@media (min-width: 640px) {
  .sm\:hover\:bg-red-400:hover {
    --en-bg-opacity: 1;
    background-color: rgb(248 113 113 / var(--en-bg-opacity));
  }
}

@media (min-width: 640px) {
  .sm\:before\:target\:content-\[\'Hello_world\!\'\]:target::before {
    --en-content: 'Hello world!';
    content: var(--en-content);
  }
}

@media (min-width: 768px) {
  .md\:focus\:selection\:bg-blue-100 *::selection, .md\:focus\:selection\:bg-blue-100::selection:focus {
    --en-bg-opacity: 1;
    background-color: rgb(219 234 254 / var(--en-bg-opacity));
  }
}

@media (min-width: 1536px) {
  @media (prefers-reduced-motion: no-preference) {
    @media (orientation: landscape) {
      [dir="rtl"] .\32xl\:motion-safe\:landscape\:focus-within\:visited\:first\:odd\:checked\:open\:rtl\:bg-purple-100[open]:checked:nth-child(odd):first-child:visited:focus-within {
        --en-bg-opacity: 1;
        background-color: rgb(243 232 255 / var(--en-bg-opacity));
      }
    }
  }
}

.group:hover .group-hover\:bg-green-300 {
  --en-bg-opacity: 1;
  background-color: rgb(134 239 172 / var(--en-bg-opacity));
}

.group:focus .group-focus\:bg-green-400 {
  --en-bg-opacity: 1;
  background-color: rgb(74 222 128 / var(--en-bg-opacity));
}

.peer:not(:invalid) ~ .peer-not-invalid\:bg-green-500 {
  --en-bg-opacity: 1;
  background-color: rgb(34 197 94 / var(--en-bg-opacity));
}

.peer:invalid ~ .peer-invalid\:bg-red-500 {
  --en-bg-opacity: 1;
  background-color: rgb(239 68 68 / var(--en-bg-opacity));
}"#
            )
        );
    }

    #[test]
    fn gen_css_for_duplicated_selectors() {
        let mut generator = EncreGenerator::from_config(base_config());
        generator.add_selector("bg-red-500");
        generator.add_selector("bg-red-500");
        generator.add_selector("bg-red-500");

        assert_eq!(
            generator.generate(),
            String::from(
                r#".bg-red-500 {
  --en-bg-opacity: 1;
  background-color: rgb(239 68 68 / var(--en-bg-opacity));
}"#
            )
        );
    }

    #[test]
    fn gen_css_for_selector_with_arbitrary_property() {
        let mut generator = EncreGenerator::from_config(base_config());
        generator.add_selector("hover:[mask-type:luminance]");

        assert_eq!(
            generator.generate(),
            String::from(
                r#".hover\:\[mask-type\:luminance\]:hover {
  mask-type: luminance;
}"#
            )
        );
    }

    #[test]
    fn gen_css_for_selector_with_arbitrary_variant() {
        let mut generator = EncreGenerator::from_config(base_config());
        generator.add_selector("[&_>_*]:before:content-['hello-']");
        generator.add_selector("[&:has(.active)]:bg-blue-500");
        generator.add_selector("[@supports_(display:grid)]:grid");
        generator.add_selector("[@supports_not_(display:grid)]:float-right");

        assert_eq!(
            generator.generate(),
            String::from(
                r#"@supports not (display:grid) {
  .\[\@supports_not_\(display\:grid\)\]\:float-right {
    float: right;
  }
}

@supports (display:grid) {
  .\[\@supports_\(display\:grid\)\]\:grid {
    display: grid;
  }
}

.\[\&\:has\(\.active\)\]\:bg-blue-500:has(.active) {
  --en-bg-opacity: 1;
  background-color: rgb(59 130 246 / var(--en-bg-opacity));
}

.\[\&_\>_\*\]\:before\:content-\[\'hello-\'\]::before > * {
  --en-content: 'hello-';
  content: var(--en-content);
}"#
            )
        );
    }

    #[test]
    fn gen_css_for_variant_group() {
        let mut generator = EncreGenerator::from_config(base_config());
        generator.add_selector("xl:(focus:(outline,outline-red-200),dark:(bg-black,text-white))");

        assert_eq!(
            generator.generate(),
            String::from(
                r#"@media (min-width: 1280px) {
  .xl\:\(focus\:\(outline\,outline-red-200\)\,dark\:\(bg-black\,text-white\)\):focus {
    outline-style: solid;
  }
}

@media (min-width: 1280px) {
  .xl\:\(focus\:\(outline\,outline-red-200\)\,dark\:\(bg-black\,text-white\)\):focus {
    outline-color: rgb(254 202 202);
  }
}

@media (prefers-color-scheme: dark) {
  @media (min-width: 1280px) {
    .xl\:\(focus\:\(outline\,outline-red-200\)\,dark\:\(bg-black\,text-white\)\) {
      --en-bg-opacity: 1;
      background-color: rgb(0 0 0 / var(--en-bg-opacity));
    }
  }
}

@media (prefers-color-scheme: dark) {
  @media (min-width: 1280px) {
    .xl\:\(focus\:\(outline\,outline-red-200\)\,dark\:\(bg-black\,text-white\)\) {
      --en-text-opacity: 1;
      color: rgb(255 255 255 / var(--en-text-opacity));
    }
  }
}"#
            )
        );
    }

    #[test]
    fn default_modifier_values_for_rounded() {
        let mut generator = EncreGenerator::from_config(base_config());
        generator.scan("rounded-tr rounded-tr-md rounded rounded-md rounded-t-sm rounded-bl-xl border-x border border-4 border-t-2");

        assert_eq!(
            generator.generate(),
            String::from(
                r#".rounded {
  border-radius: 0.25rem;
}

.rounded-md {
  border-radius: 0.375rem;
}

.rounded-t-sm {
  border-top-left-radius: 0.125rem;
  border-top-right-radius: 0.125rem;
}

.rounded-tr {
  border-top-right-radius: 0.25rem;
}

.rounded-tr-md {
  border-top-right-radius: 0.375rem;
}

.rounded-bl-xl {
  border-bottom-left-radius: 0.75rem;
}

.border {
  border-width: 1px;
}

.border-4 {
  border-width: 4px;
}

.border-x {
  border-left-width: 1px;
  border-right-width: 1px;
}

.border-t-2 {
  border-top-width: 2px;
}"#
            )
        );
    }

    #[test]
    fn gen_css_for_font_with_spaces() {
        let mut generator = EncreGenerator::from_config(base_config());
        generator.add_selector("font-['Times_New_Roman',Helvetica,serif]");
        generator.add_selector("font-[Roboto,'Open_Sans',sans-serif]");

        assert_eq!(
            generator.generate(),
            String::from(
                r#".font-\[\'Times_New_Roman\'\,Helvetica\,serif\] {
  font-family: 'Times New Roman',Helvetica,serif;
}

.font-\[Roboto\,\'Open_Sans\'\,sans-serif\] {
  font-family: Roboto,'Open Sans',sans-serif;
}"#
            )
        );
    }

    #[test]
    fn gen_css_for_container() {
        let mut generator = EncreGenerator::from_config(base_config());
        generator.add_selector("container");

        assert_eq!(
            generator.generate(),
            String::from(
                r#".container {
  width: 100%;
}

@media (min-width: 640px) {
  .container {
    max-width: 640px;
  }
}

@media (min-width: 768px) {
  .container {
    max-width: 768px;
  }
}

@media (min-width: 1024px) {
  .container {
    max-width: 1024px;
  }
}

@media (min-width: 1280px) {
  .container {
    max-width: 1280px;
  }
}

@media (min-width: 1536px) {
  .container {
    max-width: 1536px;
  }
}"#
            )
        );

        let mut generator = EncreGenerator::from_config(base_config());
        generator.add_selector("md:container");
        generator.add_selector("md:mx-auto");

        assert_eq!(
            generator.generate(),
            String::from(
                r#"@media (min-width: 768px) {
  .md\:container {
    width: 100%;
  }
}

@media (min-width: 768px) {
  @media (min-width: 640px) {
    .md\:container {
      max-width: 640px;
    }
  }

  @media (min-width: 768px) {
    .md\:container {
      max-width: 768px;
    }
  }

  @media (min-width: 1024px) {
    .md\:container {
      max-width: 1024px;
    }
  }

  @media (min-width: 1280px) {
    .md\:container {
      max-width: 1280px;
    }
  }

  @media (min-width: 1536px) {
    .md\:container {
      max-width: 1536px;
    }
  }
}

@media (min-width: 768px) {
  .md\:mx-auto {
    margin-left: auto;
    margin-right: auto;
  }
}"#
            )
        );
    }

    #[test]
    fn gen_css_for_selector_with_before_after_variant() {
        let mut generator = EncreGenerator::from_config(base_config());
        generator.add_selector("before:bg-red-500");
        generator.add_selector("before:content-['Hello_world!']");
        generator.add_selector("after:rounded-full");
        generator.add_selector("after:content-[counter(foo)]");

        assert_eq!(
            generator.generate(),
            String::from(
                r#".before\:bg-red-500::before {
  --en-bg-opacity: 1;
  background-color: rgb(239 68 68 / var(--en-bg-opacity));
  content: var(--en-content);
}

.before\:content-\[\'Hello_world\!\'\]::before {
  --en-content: 'Hello world!';
  content: var(--en-content);
}

.after\:rounded-full::after {
  border-radius: 9999px;
  content: var(--en-content);
}

.after\:content-\[counter\(foo\)\]::after {
  --en-content: counter(foo);
  content: var(--en-content);
}"#
            )
        );
    }

    #[test]
    fn gen_css_for_selector_with_dark_variant() {
        let mut generator = EncreGenerator::from_config(base_config());
        generator.add_selector("dark:mt-px");

        assert_eq!(
            generator.generate(),
            String::from(
                r#"@media (prefers-color-scheme: dark) {
  .dark\:mt-px {
    margin-top: 1px;
  }
}"#
            )
        );

        let mut config = base_config();
        config.theme.dark_mode = DarkMode::new_class(".dark");

        let mut generator = EncreGenerator::from_config(config);
        generator.add_selector("dark:mt-px");

        assert_eq!(
            generator.generate(),
            String::from(
                r#".dark .dark\:mt-px {
  margin-top: 1px;
}"#
            )
        );
    }

    #[test]
    fn arbitrary_values_test() {
        use std::fs;

        let file_content = fs::read_to_string("tests/fixtures/arbitrary-values.html").unwrap();
        let mut generator = EncreGenerator::from_config(base_config());
        generator.scan(&file_content);
        generator.generate();
    }
}
