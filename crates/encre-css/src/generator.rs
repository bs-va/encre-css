//! Define the main [`EncreGenerator`] structure used to scan content and to generate CSS styles.
use crate::{
    config::{Config, BUILTIN_VARIANTS},
    context::{ContextAfterRule, ContextBeforeRule, ContextHandle},
    error::{Error, Result},
    plugins::transition::animation,
    preflight::ENCRE_PREFLIGHT_CSS,
    selector::{Selector, VARIANT_SEPARATOR},
    utils::indent,
    variant::{init_variants, Variant},
};

use std::{
    borrow::Cow,
    collections::BTreeSet,
    fmt::Write,
    path::Path,
    sync::{atomic::Ordering, Arc},
};

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
        Self {
            config: config.into(),
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
        Selector::new(val, &self.config).map(|s| self.scanned_selectors.insert(s));
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
                .filter_map(|v| Selector::new(v.trim(), &self.config)),
        );
    }

    /// Scan the contents of a file and store all the selectors found.
    ///
    /// You can customize the extractor using the configuration field [`Config::extractor`], by
    /// default, it splits the value by spaces, double quotes, single quotes and backticks.
    ///
    /// This function automatically handles duplicated selectors and sorting.
    pub fn scan(&mut self, content: &'a str) {
        self.add_selectors(self.config.extractor.extract(content));
    }

    /// Generate the CSS styles needed based on the scanned selectors.
    ///
    /// Don't forget to scan selectors before, using:
    /// - [`add_selector`] to add a single selector to the scanned list;
    /// - [`add_selectors`] to add a list of selectors to the scanned list;
    /// - [`scan`] to scan a string (e.g. the contents of a file).
    ///
    /// # Errors
    ///
    /// Returns [`Error::Format`] if writing to the buffer failed.
    ///
    /// [`add_selector`]: EncreGenerator::add_selector
    /// [`add_selectors`]: EncreGenerator::add_selectors
    /// [`scan`]: EncreGenerator::scan
    pub fn generate(&self) -> Result<String> {
        // Make sure that animations are not defined
        animation::ANIMATIONS_ALREADY_DEFINED
            .iter()
            .for_each(|animation| animation.store(false, Ordering::Relaxed));

        let custom_variants = init_variants(&self.config);
        let get_variant = |variant| {
            BUILTIN_VARIANTS
                .iter()
                .find_map(|v| if v.0 == variant { Some(&v.1) } else { None })
                .or_else(|| custom_variants.get(&variant))
        };

        let mut buffer = String::with_capacity(10 * self.scanned_selectors.len()); // TODO: More accurate value
        buffer.push_str(ENCRE_PREFLIGHT_CSS); // TODO: Push and reserve at the same time

        // TODO: Is parallelism possible without bad sorting of selectors?
        self.scanned_selectors.iter().try_for_each(|selector| {
            write!(buffer, "\n\n")?;

            let mut indentation = 0;

            {
                let context = ContextBeforeRule {
                    config: &self.config,
                    selector,
                    buffer: &mut buffer,
                };

                selector.plugin.css_before_rule(context)?;
            }

            // Before rule
            if !selector.variants.is_empty() {
                selector
                    .variants
                    .split(VARIANT_SEPARATOR)
                    .try_for_each(|variant| {
                        if let Some(Variant::AtRule(variant)) = get_variant(Cow::from(variant)) {
                            indent(indentation, &mut buffer)?;
                            writeln!(buffer, "{} {{", variant)?;
                            indentation += 1;
                        }

                        Ok::<(), Error>(())
                    })?;
            }

            // Write the class
            indent(indentation, &mut buffer)?;
            writeln!(buffer, "{} {{", selector.get_css_class(&custom_variants))?;

            // Rule content

            // TODO: Support the important prefix
            {
                let context = ContextHandle {
                    config: &self.config,
                    modifier: &selector.modifier,
                    indentation: indentation + 1,
                    buffer: &mut buffer,
                };

                selector.plugin.handle(context)?;
            }

            // After rule
            for i in (1..=indentation).rev() {
                indent(i, &mut buffer)?;
                writeln!(buffer, "}}")?;
            }

            write!(buffer, "}}")?;

            {
                let context = ContextAfterRule {
                    config: &self.config,
                    selector,
                    buffer: &mut buffer,
                    custom_variants: &custom_variants,
                };

                selector.plugin.css_after_rule(context)?;
            }

            Ok::<(), Error>(())
        })?;

        Ok(buffer)
    }
}
