//! Define the main [`generate`] function used to scan content and to generate CSS styles.
use crate::{
    config::{Config, MaxShortcutDepth},
    preflight::Preflight,
    selector::{parse, Modifier, Selector, Variant, VariantType},
    utils::buffer::Buffer,
};

use std::{borrow::Cow, collections::BTreeSet, fmt::Write};

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

    /// The buffer containing the whole generated CSS.
    pub buffer: &'d mut Buffer,

    // Private fields used in `generate_class` and `generate_at_rules`
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
pub fn generate_at_rules<T: FnOnce(&mut ContextHandle)>(
    context: &mut ContextHandle,
    rule_content_fn: T,
) {
    let ContextHandle {
        buffer, selector, ..
    } = context;

    if !selector.variants.is_empty() {
        selector.variants.iter().for_each(|variant| match variant {
            Variant::Builtin(_, VariantType::AtRule(variant)) => {
                buffer.line(format_args!("{variant} {{"));
                buffer.indent();
            }
            Variant::Arbitrary(variant) if variant.starts_with('@') => {
                buffer.line(format_args!("{variant} {{"));
                buffer.indent();
            }
            _ => (),
        });
    }

    rule_content_fn(context);

    let ContextHandle { buffer, .. } = context;
    while !buffer.is_unindented() {
        buffer.unindent();

        if buffer.is_unindented() {
            buffer.raw("}");
        } else {
            buffer.line("}");
        }
    }
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
pub fn generate_class<T: FnOnce(&mut ContextHandle)>(
    context: &mut ContextHandle,
    rule_content_fn: T,
    custom_after_class: &str,
) {
    let ContextHandle {
        buffer, selector, ..
    } = context;

    // Write the class
    let mut base_class = String::with_capacity(1 + selector.full.len());
    base_class.push('.');

    selector.full.chars().enumerate().for_each(|(i, ch)| {
        if !ch.is_alphanumeric() && ch != '-' && ch != '_' {
            base_class.push('\\');
            base_class.push(ch);
        } else if i == 0 && ch.is_numeric() {
            // CSS classes must not start with a number, we need to escape it
            base_class.push_str("\\3");
            base_class.push(ch);
        } else {
            base_class.push(ch);
        }
    });

    if !selector.variants.is_empty() {
        selector
            .variants
            .iter()
            .rev()
            .for_each(|variant| match variant {
                Variant::Builtin(_, variant) => match variant {
                    VariantType::PseudoElement(element) => {
                        write!(base_class, "::{element}").expect("writing to a String can't fail");
                    }
                    VariantType::PseudoClass(class) => {
                        write!(base_class, ":{class}").expect("writing to a String can't fail");
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
    buffer.line(format_args!("{base_class}{custom_after_class} {{"));

    // Store the index of the start of the class content (useful when the `important` flag is present)
    let content_start = buffer.len();

    // Rule content
    buffer.indent();
    rule_content_fn(context);

    let ContextHandle {
        buffer, selector, ..
    } = context;

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
        buffer.line("content: var(--en-content);");
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

    buffer.unindent();
    if buffer.is_unindented() {
        buffer.raw("}");
    } else {
        buffer.line("}");
    }
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
pub fn generate_wrapper<T: FnOnce(&mut ContextHandle)>(
    context: &mut ContextHandle,
    rule_content_fn: T,
) {
    generate_at_rules(context, |context| {
        generate_class(context, rule_content_fn, "");
    });
}

fn resolve_selector<'a>(
    selector: &'a str,
    full_class: Option<&'a str>,
    selectors: &mut BTreeSet<Selector<'a>>,
    config: &'a Config,
    config_derived_variants: &[(Cow<'static, str>, VariantType)],
    depth: MaxShortcutDepth,
) {
    if depth.get() == 0 {
        return;
    }

    if let Some(expanded) = config.shortcuts.get(selector) {
        expanded.split(' ').for_each(|shortcut_target| {
            resolve_selector(
                shortcut_target,
                full_class.or(Some(selector)),
                selectors,
                config,
                config_derived_variants,
                MaxShortcutDepth::new(depth.get() - 1),
            );
        });
    } else {
        selectors.extend(
            parse(selector, None, full_class, config, config_derived_variants)
                .into_iter()
                .filter_map(Result::ok),
        );
    }
}

/// Generate the CSS styles needed based on the given sources.
///
/// Each source will be scanned in order to extract atomic classes, then CSS will be generated for
/// each class found.
///
/// By default, it splits the source by spaces, double quotes, single quotes, backticks and new
/// lines, while ignoring the content inside arbitrary values/variants and variant groups.
///
/// This function also removes duplicated selectors and sorts the generated CSS classes based on
/// the order in which they were defined to avoid conflicts.
pub fn generate<'a>(sources: impl IntoIterator<Item = &'a str>, config: &Config) -> String {
    let config_derived_variants = config.get_derived_variants();
    let mut selectors = BTreeSet::new();

    // Add selectors from the safelist
    for safe_selector in config.safelist.iter() {
        if let Some(expanded) = config.shortcuts.get(&**safe_selector) {
            expanded.split(' ').for_each(|shortcut_target| {
                selectors.extend(
                    parse(
                        shortcut_target,
                        None,
                        Some(safe_selector),
                        config,
                        &config_derived_variants,
                    )
                    .into_iter()
                    .filter_map(Result::ok),
                );
            });
        } else {
            selectors.extend(
                parse(safe_selector, None, None, config, &config_derived_variants)
                    .into_iter()
                    .filter_map(Result::ok),
            );
        }
    }

    for source in sources {
        let new_selectors = config.scanner.scan(source);

        for selector in new_selectors {
            resolve_selector(
                selector,
                None,
                &mut selectors,
                config,
                &config_derived_variants,
                config.max_shortcut_depth,
            );
        }
    }

    let preflight = config.preflight.build();
    let mut buffer = Buffer::with_capacity(10 * selectors.len()); // TODO: More accurate value
    buffer.raw(&preflight);

    for selector in selectors {
        if buffer.len() != preflight.len() || config.preflight != Preflight::None {
            buffer.raw("\n\n");
        }

        let mut context = ContextHandle {
            config,
            modifier: &selector.modifier,
            buffer: &mut buffer,
            selector: &selector,
        };

        if selector.plugin.needs_wrapping() {
            generate_wrapper(&mut context, |context| selector.plugin.handle(context));
        } else {
            selector.plugin.handle(&mut context);
        }
    }

    buffer.into_inner()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::DarkMode, utils::testing::base_config};

    use pretty_assertions::assert_eq;

    #[test]
    fn divide_and_space_between_special_class() {
        let generated = generate(
            [
                "hover:space-x-1",
                "space-x-2",
                "[&:has(.class)_>_*]:space-y-3",
                "divide-red-100",
                "divide-dashed",
                "divide-x-[11px]",
                "xl:[&_>_*]:divide-y-2",
            ],
            &base_config(),
        );

        assert_eq!(
            generated,
            String::from(
                r".space-x-2 > :not([hidden]) ~ :not([hidden]) {
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
}"
            )
        );
    }

    #[test]
    fn negative_values() {
        let generated = generate(
            [
                "-top-2",
                "-z-2",
                "-order-2",
                "-mb8",
                "-translate-x-52",
                "-rotate-90",
                "-skew-x-2",
                "-scale-50",
                "-scroll-mt-2",
                "-space-x-2",
                "-indent-2",
                "-hue-rotate-60",
                "hover:-hue-rotate-60",
                "-backdrop-hue-rotate-90",
            ],
            &base_config(),
        );

        assert_eq!(
            generated,
            String::from(
                r".-top-2 {
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
}"
            )
        );
    }

    #[test]
    fn gen_css_for_simple_selector() {
        let generated = generate(["text-current"], &base_config());

        assert_eq!(
            generated,
            String::from(
                ".text-current {
  color: currentColor;
}"
            )
        );
    }

    #[test]
    fn gen_css_with_important_flag() {
        let generated = generate(
            [
                "!w-full",
                "!-mb-8",
                "!shadow",
                "!-hue-rotate-60",
                "focus:!w-2",
                "focus:!-mb-2",
            ],
            &base_config(),
        );

        assert_eq!(
            generated,
            String::from(
                r".\!-mb-8 {
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
}",
            )
        );
    }

    #[test]
    fn gen_css_for_selector_needing_custom_css() {
        let generated = generate(["animate-pulse", "animate-pulse"], &base_config());

        assert_eq!(
            generated,
            String::from(
                "@-webkit-keyframes pulse {
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
}"
            )
        );
    }

    #[test]
    fn gen_css_for_arbitrary_value() {
        let generated = generate(
            [
                "w[12px]",
                "bg-[red]",
                "bg-[url('../img/image_with_underscores.png')]",
                "mt-[calc(100%-10px)]",
                "2xl:pb-[calc((100%/2)-10px+2rem)]",
            ],
            &base_config(),
        );

        assert_eq!(
            generated,
            String::from(
                r".mt-\[calc\(100\%-10px\)\] {
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
}"
            )
        );
    }

    #[test]
    fn gen_css_for_arbitrary_value_with_hint() {
        let generated = generate(["bg-[color:red]", "hover:bg-[color:red]"], &base_config());

        assert_eq!(
            generated,
            String::from(
                r".bg-\[color\:red\] {
  background-color: red;
}

.hover\:bg-\[color\:red\]:hover {
  background-color: red;
}"
            )
        );
    }

    #[test]
    fn gen_css_for_selector_with_simple_variant() {
        let generated = generate(["focus:w-full"], &base_config());

        assert_eq!(
            generated,
            String::from(
                r".focus\:w-full:focus {
  width: 100%;
}"
            )
        );
    }

    #[test]
    fn gen_selector_css_variants_test() {
        let generated = generate([
            "sm:hover:bg-red-400",
            "focus:hover:bg-red-600",
            "active:rtl:bg-red-800",
            "md:focus:selection:bg-blue-100",
            "rtl:active:focus:lg:underline",
            "print:ltr:xl:hover:focus:active:text-yellow-300",
            "2xl:motion-safe:landscape:focus-within:visited:first:odd:checked:open:rtl:bg-purple-100",
            "hover:file:bg-pink-600",
            "file:hover:bg-pink-600",
            "sm:before:target:content-['Hello_world!']",
            "marker:selection:hover:bg-green-200",
            "group-hover:bg-green-300",
            "group-focus:bg-green-400",
            "peer-invalid:bg-red-500",
            "peer-not-invalid:bg-green-500",
        ], &base_config());

        assert_eq!(
            generated,
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
  .sm\:before\:target\:content-\[\'Hello_world\!\'\]:target::before {
    --en-content: 'Hello world!';
    content: var(--en-content);
  }
}

@media (min-width: 640px) {
  .sm\:hover\:bg-red-400:hover {
    --en-bg-opacity: 1;
    background-color: rgb(248 113 113 / var(--en-bg-opacity));
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
        let generated = generate(["bg-red-500 bg-red-500", "bg-red-500"], &base_config());

        assert_eq!(
            generated,
            String::from(
                ".bg-red-500 {
  --en-bg-opacity: 1;
  background-color: rgb(239 68 68 / var(--en-bg-opacity));
}"
            )
        );
    }

    #[test]
    fn gen_css_for_selector_with_arbitrary_property() {
        let generated = generate(["hover:[mask-type:luminance]"], &base_config());

        assert_eq!(
            generated,
            String::from(
                r".hover\:\[mask-type\:luminance\]:hover {
  mask-type: luminance;
}"
            )
        );
    }

    #[test]
    fn gen_css_for_selector_with_arbitrary_variant() {
        let generated = generate(
            [
                "[&_>_*]:before:content-['hello-']",
                "[&:has(.active)]:bg-blue-500",
                "[@supports_(display:grid)]:grid",
                "[@supports_not_(display:grid)]:float-right",
            ],
            &base_config(),
        );

        assert_eq!(
            generated,
            String::from(
                r"@supports not (display:grid) {
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
}"
            )
        );
    }

    #[test]
    fn gen_css_for_variant_group() {
        let generated = generate(
            ["xl:(focus:(outline,outline-red-200),dark:(bg-black,text-white))"],
            &base_config(),
        );

        assert_eq!(
            generated,
            String::from(
                r"@media (min-width: 1280px) {
  .xl\:\(focus\:\(outline\,outline-red-200\)\,dark\:\(bg-black\,text-white\)\):focus {
    outline-color: rgb(254 202 202);
  }
}

@media (min-width: 1280px) {
  .xl\:\(focus\:\(outline\,outline-red-200\)\,dark\:\(bg-black\,text-white\)\):focus {
    outline-style: solid;
  }
}

@media (prefers-color-scheme: dark) {
  @media (min-width: 1280px) {
    .xl\:\(focus\:\(outline\,outline-red-200\)\,dark\:\(bg-black\,text-white\)\) {
      --en-text-opacity: 1;
      color: rgb(255 255 255 / var(--en-text-opacity));
    }
  }
}

@media (prefers-color-scheme: dark) {
  @media (min-width: 1280px) {
    .xl\:\(focus\:\(outline\,outline-red-200\)\,dark\:\(bg-black\,text-white\)\) {
      --en-bg-opacity: 1;
      background-color: rgb(0 0 0 / var(--en-bg-opacity));
    }
  }
}"
            )
        );
    }

    #[test]
    fn default_modifier_values_for_rounded() {
        let generated = generate([
            "rounded-tr rounded-tr-md rounded rounded-md rounded-t-sm rounded-bl-xl border-x border border-4 border-t-2",
        ], &base_config());

        assert_eq!(
            generated,
            String::from(
                ".rounded {
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
}"
            )
        );
    }

    #[test]
    fn gen_css_for_font_with_spaces() {
        let generated = generate(
            [
                "font-['Times_New_Roman',Helvetica,serif]",
                "font-[Roboto,'Open_Sans',sans-serif]",
            ],
            &base_config(),
        );

        assert_eq!(
            generated,
            String::from(
                r".font-\[\'Times_New_Roman\'\,Helvetica\,serif\] {
  font-family: 'Times New Roman',Helvetica,serif;
}

.font-\[Roboto\,\'Open_Sans\'\,sans-serif\] {
  font-family: Roboto,'Open Sans',sans-serif;
}"
            )
        );
    }

    #[test]
    fn gen_css_for_container() {
        let generated = generate(["container"], &base_config());

        assert_eq!(
            generated,
            String::from(
                ".container {
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
}"
            )
        );

        let generated = generate(["md:container", "md:mx-auto"], &base_config());

        assert_eq!(
            generated,
            String::from(
                r"@media (min-width: 768px) {
  .md\:mx-auto {
    margin-left: auto;
    margin-right: auto;
  }
}

@media (min-width: 768px) {
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
}"
            )
        );
    }

    #[test]
    fn gen_css_for_selector_with_before_after_variant() {
        let generated = generate(
            [
                "before:bg-red-500",
                "before:content-['Hello_world!']",
                "after:rounded-full",
                "after:content-[counter(foo)]",
            ],
            &base_config(),
        );

        assert_eq!(
            generated,
            String::from(
                r".before\:content-\[\'Hello_world\!\'\]::before {
  --en-content: 'Hello world!';
  content: var(--en-content);
}

.before\:bg-red-500::before {
  --en-bg-opacity: 1;
  background-color: rgb(239 68 68 / var(--en-bg-opacity));
  content: var(--en-content);
}

.after\:content-\[counter\(foo\)\]::after {
  --en-content: counter(foo);
  content: var(--en-content);
}

.after\:rounded-full::after {
  border-radius: 9999px;
  content: var(--en-content);
}"
            )
        );
    }

    #[test]
    fn gen_css_for_selector_with_dark_variant() {
        let generated = generate(["dark:mt-px"], &base_config());

        assert_eq!(
            generated,
            String::from(
                r"@media (prefers-color-scheme: dark) {
  .dark\:mt-px {
    margin-top: 1px;
  }
}"
            )
        );

        let mut config = base_config();
        config.theme.dark_mode = DarkMode::new_class(".dark");

        let generated = generate(["dark:mt-px"], &config);

        assert_eq!(
            generated,
            String::from(
                r".dark .dark\:mt-px {
  margin-top: 1px;
}"
            )
        );
    }

    #[test]
    fn arbitrary_values_test() {
        use std::fs;

        let file_content = fs::read_to_string("tests/fixtures/arbitrary-values.html").unwrap();
        let _generated = generate([file_content.as_str()], &base_config());
    }
}
