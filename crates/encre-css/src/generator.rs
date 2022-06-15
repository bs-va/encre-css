use crate::{
    config::Config,
    error::{Error, Result},
    plugins::transition,
    preflight::ENCRE_PREFLIGHT_CSS,
    selector::{Modifier, Selector},
    utils::indent,
    variant::{init_variants, Variant},
};

use lazy_static::lazy_static;
use regex::{Captures, Regex};
use smol_str::SmolStr;
use std::{
    borrow::Cow,
    collections::{BTreeMap, BTreeSet},
    fmt::Write,
    fs,
    io::Read,
    iter,
    path::Path,
    sync::{atomic::Ordering, Arc},
};
use wax::Glob;

lazy_static! {
    static ref URL_REGEX: Regex = Regex::new(r"url\((.+)\)").unwrap();
    static ref URL_REGEX_STRICT: Regex = Regex::new(r"^url\((.+)\)$").unwrap();
    static ref CALC_REGEX: Regex = Regex::new(r"calc\((.+)\)").unwrap();
    static ref SPLIT_REGEX: Regex = Regex::new(r#"(?-u)[\s'"`;>=]+"#).unwrap();
}

pub const VALID_PLUGIN_HINT: [&str; 4] = ["color", "length", "angle", "list"];
const WILL_BE_REPLACED_BY_UNDERSCORE: &str = "WILL-BE-REPLACED-BY-UNDERSCORE";

/// Convert an arbitrary value into a CSS value
///
///  -  `_` (underscores) are converted to ` ` (spaces) (not in `url`s)
pub fn to_css_value(value: &str) -> SmolStr {
    // Don't replace `_` if it is a URL
    let value = if value.contains("url") {
        // If the value contains an url, it won't contain a calculation, so we can safely return here
        URL_REGEX.replace(value, |caps: &Captures| {
            format!(
                "url({})",
                caps[1].replace('_', WILL_BE_REPLACED_BY_UNDERSCORE)
            )
        })
    } else {
        Cow::from(value)
    };

    // Don't replace `_` if prefixed by a `\`
    let value = value
        .replace("\\_", WILL_BE_REPLACED_BY_UNDERSCORE)
        .replace('_', " ")
        .replace(WILL_BE_REPLACED_BY_UNDERSCORE, "_");

    if value.contains("calc") {
        SmolStr::from(
            CALC_REGEX
                .replace(&value, |caps: &Captures| {
                    format!(
                        "calc({})",
                        caps[1]
                            .replace('-', " - ")
                            .replace('+', " + ")
                            .replace('/', " / ")
                            .replace('*', "*")
                    )
                })
                .to_string(),
        )
    } else {
        SmolStr::from(value)
    }
}

/// Main structure used to generate CSS from selectors
pub struct EncreGenerator {
    config: Arc<Config>,
    variants: BTreeMap<Cow<'static, str>, Variant>,
    pub(crate) scanned_selectors: BTreeSet<Selector>,
}

impl EncreGenerator {
    /// Create a new [`EncreGenerator`] by trying to read a configuration file
    ///
    /// If the file does not exist, a warning will be emitted and the default configuration will be
    /// used
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

    /// Create a new [`EncreGenerator`] using a given configuration
    ///
    /// The paths in the [`Config::input`] field of the configuration will be scanned
    pub fn from_config(config: Config) -> Self {
        let config = Arc::new(config);

        let mut result_self = Self {
            variants: init_variants(&config),
            config: Arc::clone(&config),
            scanned_selectors: BTreeSet::new(),
        };

        // Scan the files listed in the `input` configuration
        config.input.iter().for_each(|p| result_self.scan_path(p));
        result_self
    }

    /// Get the configuration
    pub fn get_config(&self) -> Arc<Config> {
        self.config.clone()
    }

    /// Set the configuration
    pub fn set_config(&mut self, config: Config) {
        self.config = Arc::new(config);
    }

    /// Add a new selector which will have its CSS generated
    ///
    /// This function automatically handles duplicated selectors
    pub fn add_selector(&mut self, val: &str) {
        Selector::new(val, &self.config).map(|s| self.scanned_selectors.insert(s));
    }

    /// Add a list of new selectors which will have their CSS generated
    pub fn add_selectors(&mut self, val: BTreeSet<Selector>) {
        self.scanned_selectors.extend(val);
    }

    /// Scan the contents of a file and store all the selectors found
    pub fn scan_raw(&mut self, content: &str) {
        self.scanned_selectors.extend(
            SPLIT_REGEX
                .split(content)
                .filter_map(|val| {
                    // The shortest selector is `m-1`
                    if val.len() >= 3 {
                        Selector::new(val, &self.config)
                    } else {
                        None
                    }
                })
                .collect::<BTreeSet<Selector>>(),
        );
    }

    /// Scan all files given and store all the selectors found
    pub fn scan_files<T: AsRef<Path>>(&mut self, files: impl Iterator<Item = T>) {
        // TODO: Error handling
        let mut file_contents: String = String::new();

        debug!("Start scanning files");
        files.for_each(|file_path| {
            let mut file = match fs::File::open(&file_path) {
                Ok(f) => f,
                Err(e) => panic!("Failed to read the file {:?}: {:?}", file_path.as_ref(), e),
            };
            file_contents.clear();

            if file.read_to_string(&mut file_contents).is_ok() {
                self.scan_raw(&file_contents)
            } else {
                // TODO: Display a warning otherwise
            }
        });
        debug!("Finished scanning files");
    }

    /// Scan all files in a path using the glob syntax
    pub fn scan_path<T: AsRef<Path>>(&mut self, glob_path: T) {
        let (prefix, glob) = match Glob::new(
            glob_path
                .as_ref()
                .to_str()
                .expect("failed to convert the glob to a string"),
        ) {
            Ok(g) => g.partition(),
            Err(e) => panic!("{}", e),
        };

        if prefix == glob_path.as_ref() {
            self.scan_files(iter::once(glob_path));
        } else {
            self.scan_files(glob.walk(prefix).map(|e| e.unwrap().into_path()));
        }
    }

    /// Generate the CSS styles needed based on the scanned selectors
    ///
    /// NOTE: Don't forget to scan selectors using either [scan_files] or [scan_raw] or by
    /// adding individual selectors using [add_selector]
    ///
    /// [scan_files]: EncreGenerator::scan_files
    /// [scan_raw]: EncreGenerator::scan_raw
    /// [add_selector]: EncreGenerator::add_selector
    pub fn generate(&self) -> Result<String> {
        debug!("Start generating CSS");
        let mut buffer = String::with_capacity(10 * self.scanned_selectors.len()); // TODO: More accurate value
        buffer.push_str(ENCRE_PREFLIGHT_CSS); // TODO: Push and reserve at the same time

        // TODO: Is parallelism possible without bad sorting of selectors?
        self.scanned_selectors.iter().try_for_each(|selector| {
            write!(buffer, "\n\n")?;

            selector
                .plugin
                .css_before_rule(&selector.modifier, &mut buffer)?;

            let mut indentation = 0;

            // Before rule
            if let Some(ref variants) = selector.variants {
                variants.iter().try_for_each(|variant| {
                    if let Some(Variant::BeforeRule(variant)) =
                        self.variants.get(&Cow::from(variant))
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
            if let Some(ref variants) = selector.variants {
                // Variants are reversed to be compatible with TailwindCSS
                variants.iter().rev().try_for_each(|variant| {
                    if let Some(Variant::BeforeClass(variant)) =
                        self.variants.get(&Cow::from(variant))
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
            if let Some(ref variants) = selector.variants {
                // Variants are reversed to be compatible with TailwindCSS
                variants.iter().rev().try_for_each(|variant| {
                    if let Some(Variant::AfterClass(variant)) =
                        self.variants.get(&Cow::from(variant))
                    {
                        write!(buffer, "{}", variant)?;
                    }

                    Ok::<(), Error>(())
                })?;
            }

            writeln!(buffer, " {{")?;

            // Rule content
            let modifier = match &selector.modifier {
                Modifier::Basic { is_negative, value } => Modifier::Basic {
                    is_negative: *is_negative,
                    value: value.clone(),
                },
                Modifier::Arbitrary { hint, value } => {
                    // Transform the mangled CSS content of the selector into a real CSS rule
                    Modifier::Arbitrary {
                        hint: hint.clone(),
                        value: to_css_value(value),
                    }
                }
            };

            // TODO: Support the important prefix
            selector
                .plugin
                .handle(&self.config, &modifier, indentation + 1, &mut buffer)?;

            // After rule
            for i in (1..indentation + 1).rev() {
                indent(i, &mut buffer)?;
                writeln!(buffer, "}}")?;
            }

            write!(buffer, "}}")?;

            Ok::<(), Error>(())
        })?;

        debug!("Finished generating CSS");

        Ok(buffer)
    }

    /// Restore the default state of the generator (without any scanned selectors)
    /// Useful when repeatedly calling [`EncreGenerator::generate`]
    pub fn reset(&mut self) {
        self.scanned_selectors.clear();

        // Make sure animations are not defined
        transition::ANIMATIONS_ALREADY_DEFINED
            .iter()
            .for_each(|animation| animation.store(false, Ordering::Relaxed));
    }
}
