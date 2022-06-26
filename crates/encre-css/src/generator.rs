use crate::{
    config::Config,
    error::{Error, Result},
    plugins::transition,
    preflight::ENCRE_PREFLIGHT_CSS,
    selector::Selector,
    utils::indent,
    variant::{init_variants, Variant, BUILTIN_VARIANTS, VARIANT_SEPARATOR},
};

use std::{
    borrow::Cow,
    collections::BTreeSet,
    fmt::Write,
    path::Path,
    sync::{atomic::Ordering, Arc},
};

/// Main structure used to generate CSS from selectors
///
/// It is common to build this structure each time the CSS needs to be generated for the file
/// contents (due to lifetimes, the file contents must live as long as the [`EncreGenerator`]
/// structure and if you call several times the [`EncreGenerator::generate`] function, you will
/// need to clear the buffer and the scanned selectors will be in an undefined state). In this
/// case, [`EncreGenerator::from_config`] can take an [`Arc<Config>`] to avoid cloning the
/// configuration.
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
                .filter_map(|v| Selector::new(v, &self.config)),
        );
    }

    /// Scan the contents of a file and store all the selectors found.
    ///
    /// You can customize the extractor using the configuration field [Config::extractor], by
    /// default, it splits the value by spaces, double quotes, single quotes and backticks.
    ///
    /// This function automatically handles duplicated selectors and sorting.
    pub fn scan(&mut self, content: &'a str) {
        self.add_selectors(self.config.extractor.extract(content));
    }

    /// Generate the CSS styles needed based on the scanned selectors.
    ///
    /// Don't forget to scan selectors before, using:
    /// - [add_selector] to add a single selector to the scanned list;
    /// - [add_selectors] to add a list of selectors to the scanned list;
    /// - [scan] to scan a string (e.g. the contents of a file).
    ///
    /// [add_selector]: EncreGenerator::add_selector
    /// [add_selectors]: EncreGenerator::add_selectors
    /// [scan]: EncreGenerator::scan
    pub fn generate(&self) -> Result<String> {
        // Make sure that animations are not defined
        transition::ANIMATIONS_ALREADY_DEFINED
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

            selector
                .plugin
                .css_before_rule(&self.config, &selector.modifier, &mut buffer)?;

            let mut indentation = 0;

            // Before rule
            if !selector.variants.is_empty() {
                selector
                    .variants
                    .split(VARIANT_SEPARATOR)
                    .try_for_each(|variant| {
                        if let Some(Variant::BeforeRule(variant)) = get_variant(Cow::from(variant))
                        {
                            indent(indentation, &mut buffer)?;
                            writeln!(buffer, "{} {{", variant)?;
                            indentation += 1;
                        }

                        Ok::<(), Error>(())
                    })?;
            }

            // Before class
            indent(indentation, &mut buffer)?;
            if !selector.variants.is_empty() {
                // Variants are reversed to be compatible with TailwindCSS
                selector
                    .variants
                    .split(VARIANT_SEPARATOR)
                    .rev()
                    .try_for_each(|variant| {
                        if let Some(Variant::BeforeClass(variant)) = get_variant(Cow::from(variant))
                        {
                            write!(buffer, "{}", variant)?;
                        }

                        Ok::<(), Error>(())
                    })?;
            }

            // Class
            write!(buffer, ".")?;

            selector.full.chars().enumerate().try_for_each(|(i, ch)| {
                if i == 0 {
                    if ch.is_numeric() {
                        // CSS classes must not start with a number, we need to escape it
                        write!(buffer, "\\3")?;
                    }

                    write!(buffer, "{}", ch)?;
                } else if !ch.is_alphanumeric() && ch != '-' && ch != '_' {
                    write!(buffer, "\\{}", ch)?;
                } else {
                    write!(buffer, "{}", ch)?;
                }

                Ok::<(), Error>(())
            })?;

            // After class
            if !selector.variants.is_empty() {
                // Variants are reversed to be compatible with TailwindCSS
                selector
                    .variants
                    .split(VARIANT_SEPARATOR)
                    .rev()
                    .try_for_each(|variant| {
                        if let Some(Variant::AfterClass(variant)) = get_variant(Cow::from(variant))
                        {
                            write!(buffer, "{}", variant)?;
                        }

                        Ok::<(), Error>(())
                    })?;
            }

            writeln!(buffer, " {{")?;

            // Rule content

            // TODO: Support the important prefix
            selector.plugin.handle(
                &self.config,
                &selector.modifier,
                indentation + 1,
                &mut buffer,
            )?;

            // After rule
            for i in (1..indentation + 1).rev() {
                indent(i, &mut buffer)?;
                writeln!(buffer, "}}")?;
            }

            write!(buffer, "}}")?;

            selector
                .plugin
                .css_after_rule(&self.config, &selector.modifier, &mut buffer)?;

            Ok::<(), Error>(())
        })?;

        Ok(buffer)
    }
}
