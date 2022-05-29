use crate::{
    config::Config,
    extractor::Extractor,
    plugins::*,
    preflight::ENCRE_PREFLIGHT_CSS,
    selector::Selector,
    variant::{init_variants, Variant},
};

use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::{
    borrow::Cow,
    collections::{BTreeMap, BTreeSet},
    path::Path,
    sync::Arc,
};

#[cfg(not(target_arch = "wasm32"))]
use rayon::prelude::*;

lazy_static! {
    static ref URL_REGEX: Regex = Regex::new(r"url\((.+)\)").unwrap();
    static ref URL_REGEX_STRICT: Regex = Regex::new(r"^url\((.+)\)$").unwrap();
    static ref CALC_REGEX: Regex = Regex::new(r"calc\((.+)\)").unwrap();
}

pub const VALID_PLUGIN_HINT: [&str; 4] = ["color", "length", "angle", "list"];
const WILL_BE_REPLACED_BY_CSS_SELECTOR: &str = "WILL_BE_REPLACED_BY_CSS_SELECTOR";
const WILL_BE_REPLACED_BY_UNDERSCORE: &str = "WILL-BE-REPLACED-BY-UNDERSCORE";

pub fn indent(val: Cow<str>) -> String {
    val.replace('\n', "\n  ")
}

pub fn find_arbitrary_value_hint(selector: Option<&String>) -> Option<(&str, &str)> {
    if let Some(arbitrary_value) = selector {
        let mut split = arbitrary_value.split(':');
        let maybe_hint = split.next().unwrap();

        if maybe_hint == arbitrary_value.as_str() {
            // No plugin hint
            Some(("", arbitrary_value))
        } else {
            let val = split.next();

            if let Some(val) = val {
                if VALID_PLUGIN_HINT.contains(&maybe_hint) {
                    // Valid! Return (hint, stripped arbitrary value)
                    Some((maybe_hint, val))
                } else {
                    // Unknown plugin hint (like `bg-[sth:#333]`)
                    // TODO: Display a warning
                    Some(("", val))
                }
            } else {
                // Malformed arbitrary value (like just `bg-[color:]`)
                // TODO: Display a warning
                Some(("", maybe_hint))
            }
        }
    } else {
        None
    }
}

/// Convert an arbitrary value into a CSS value
///
///  -  `_` (underscores) are converted to ` ` (spaces) (not in `url`s)
pub fn to_css_value(val: &str) -> Cow<str> {
    // Don't replace `_` if it is a URL
    let val = if val.contains("url") {
        // If the value contains an url, it won't contain a calculation, so we can safely return here
        URL_REGEX.replace(val, |caps: &Captures| {
            format!(
                "url({})",
                caps[1].replace('_', WILL_BE_REPLACED_BY_UNDERSCORE)
            )
        })
    } else {
        Cow::from(val)
    };

    // Don't replace `_` if prefixed by a `\`
    let val = val
        .replace("\\_", WILL_BE_REPLACED_BY_UNDERSCORE)
        .replace('_', " ")
        .replace(WILL_BE_REPLACED_BY_UNDERSCORE, "_");

    if val.contains("calc") {
        Cow::from(
            CALC_REGEX
                .replace(&val, |caps: &Captures| {
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
        Cow::from(val)
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
        #[cfg(target_arch = "wasm32")]
        let iter = config.input.iter();

        #[cfg(not(target_arch = "wasm32"))]
        let iter = config.input.par_iter();

        let scanned_selectors = iter
            .map(Extractor::scan_path)
            .reduce_with(|mut selectors1, selectors2| {
                selectors1.extend(selectors2);
                selectors1
            })
            .unwrap_or_default();

        Self {
            variants: init_variants(&config),
            config: Arc::new(config),
            scanned_selectors,
        }
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
        self.scanned_selectors.insert(Selector::new(val));
    }

    /// Add a list of new selectors which will have their CSS generated
    ///
    /// This function automatically handles duplicated selectors
    pub fn add_selectors(&mut self, val: BTreeSet<Selector>) {
        self.scanned_selectors.extend(val);
    }

    /// Scan the contents of a file and store all the selectors found
    pub fn scan_raw(&mut self, content: &str) {
        self.scanned_selectors.extend(Extractor::scan_raw(content));
    }

    /// Scan all files given and store all the selectors found
    pub fn scan_files<T: AsRef<Path>>(&mut self, files: impl Iterator<Item = T>) {
        self.scanned_selectors.extend(Extractor::scan_files(files));
    }

    /// Scan all files in a path using the glob syntax
    pub fn scan_path<T: AsRef<Path>>(&mut self, glob_path: T) {
        self.scanned_selectors
            .extend(Extractor::scan_path(glob_path));
    }

    /// Generate the CSS styles needed based on the scanned selectors
    ///
    /// NOTE: Don't forget to scan selectors using either [scan_files] or [scan_raw] or by
    /// adding individual selectors using [add_selector]
    ///
    /// [scan_files]: EncreGenerator::scan_files
    /// [scan_raw]: EncreGenerator::scan_raw
    /// [add_selector]: EncreGenerator::add_selector
    pub fn generate(&self) -> String {
        let plugins = self.build_plugins();

        #[cfg(target_arch = "wasm32")]
        let iter = self.scanned_selectors.iter();

        #[cfg(not(target_arch = "wasm32"))]
        let iter = self.scanned_selectors.par_iter();

        let result = iter
            .filter_map(|selector| {
                let arbitrary_value = selector.get_arbitrary_value();
                self.get_css(selector, arbitrary_value, &plugins)
            })
            .collect::<String>();

        format!("{}{}", ENCRE_PREFLIGHT_CSS, result.trim_end_matches('\n'))
    }

    /// Find the matching plugin for a selector and returns the CSS generated (if the resulting CSS is valid,
    /// the plugin matches)
    pub fn get_css(
        &self,
        selector: &Selector,
        arbitrary_value: Option<(&str, &str)>,
        plugins: &[Box<dyn Plugin + Sync>; 181],
    ) -> Option<String> {
        plugins.iter().find_map(|plugin| {
            if selector.check_namespace(plugin.namespace()) {
                let mut css_content = String::new();
                let mut custom_css = String::new();

                if plugin.get_css_for_modifier(
                    &self.config,
                    &selector.get_modifier(plugin.namespace()),
                    &mut css_content,
                    &mut custom_css,
                ) {
                    let result = format!("{}\n\n", self.gen_css_rule(selector, &css_content));

                    return Some(if custom_css.is_empty() {
                        result
                    } else {
                        format!("{}{}", custom_css, result)
                    });
                } else if let Some(arbitrary_value) = arbitrary_value {
                    if plugin.is_matching_value(arbitrary_value.0, arbitrary_value.1)
                        && plugin
                            .css_template_value(&to_css_value(arbitrary_value.1), &mut css_content)
                    {
                        return Some(format!(
                            "{}\n\n",
                            self.gen_css_rule(selector, &css_content).as_str()
                        ));
                    }
                }
            }
            None
        })
    }

    /// Generate a complete CSS rule (with a class selector, a rule content and, if requested, some
    /// pseudo-elements or `@media` queries)
    pub fn gen_css_rule(&self, selector: &Selector, css_content: &str) -> String {
        let mut css_selector = selector
            .full_name
            .chars()
            .enumerate()
            .map(|(i, ch)| {
                if i == 0 {
                    // A CSS class must start with a `.`
                    let mut result = ".".to_string();

                    if ch.is_numeric() {
                        // CSS classes must not start with a number, we need to escape it
                        result.push_str("\\3");
                        result.push(ch);
                        result
                    } else {
                        result.push(ch);
                        result
                    }
                } else if !ch.is_alphanumeric() && ch != '-' && ch != '_' {
                    format!("\\{}", ch)
                } else {
                    ch.to_string()
                }
            })
            .collect::<String>();

        let css_content = if selector.is_important {
            Cow::from(css_content.replace(';', " !important;"))
        } else {
            Cow::from(css_content)
        };

        let variants = selector.variants.as_ref();
        if let Some(variants) = variants {
            let rule = variants.iter().fold(
                format!(
                    "{} {{\n  {}\n}}",
                    WILL_BE_REPLACED_BY_CSS_SELECTOR,
                    indent(css_content),
                ),
                |acc, variant| {
                    let right_variant = if let Some(result) = self.variants.get(variant.as_str()) {
                        result
                    } else {
                        println!("Unknown variant: {}", variant);
                        return acc;
                    };

                    match right_variant {
                        Variant::PseudoClass(name) => {
                            css_selector.push_str(&format!(":{}", name));
                            acc
                        }
                        Variant::PseudoElement(name) => {
                            css_selector.push_str(&format!("::{}", name));
                            acc
                        }
                        Variant::WrapSelector(template) => {
                            css_selector = template.replace('&', &css_selector);
                            acc
                        }
                        Variant::AtRule(at_rule) => {
                            format!("{} {{\n  {}\n}}", at_rule, indent(Cow::from(acc)),)
                        }
                    }
                },
            );

            rule.replace(WILL_BE_REPLACED_BY_CSS_SELECTOR, &css_selector)
        } else {
            format!("{} {{\n  {}\n}}", css_selector, indent(css_content))
        }
    }

    /// Return the list of plugins needed
    pub fn build_plugins(&self) -> [Box<dyn Plugin + Sync>; 181] {
        // TODO: Better sorting (colors and lengths after all the other utilities (because they have
        // hints))
        [
            Box::new(background::ColorPlugin),
            Box::new(background::AttachmentPlugin),
            Box::new(background::ClipPlugin),
            Box::new(background::OpacityPlugin),
            Box::new(background::ImagePlugin),
            Box::new(background::GradientFromPlugin),
            Box::new(background::GradientViaPlugin),
            Box::new(background::GradientToPlugin),
            Box::new(background::PositionPlugin),
            Box::new(background::RepeatPlugin),
            Box::new(background::SizePlugin),
            Box::new(border::ColorPlugin),
            Box::new(border::RadiusPlugin),
            Box::new(border::RadiusTopPlugin),
            Box::new(border::RadiusBottomPlugin),
            Box::new(border::RadiusLeftPlugin),
            Box::new(border::RadiusRightPlugin),
            Box::new(border::RadiusTopLeftPlugin),
            Box::new(border::RadiusTopRightPlugin),
            Box::new(border::RadiusBottomLeftPlugin),
            Box::new(border::RadiusBottomRightPlugin),
            Box::new(border::StylePlugin),
            Box::new(border::WidthPlugin),
            Box::new(border::WidthXPlugin),
            Box::new(border::WidthYPlugin),
            Box::new(border::WidthTopPlugin),
            Box::new(border::WidthBottomPlugin),
            Box::new(border::WidthLeftPlugin),
            Box::new(border::WidthRightPlugin),
            Box::new(border::OpacityPlugin),
            Box::new(border::DivideColorPlugin),
            Box::new(border::DivideWidthXPlugin),
            Box::new(border::DivideWidthYPlugin),
            Box::new(border::DivideStylePlugin),
            Box::new(border::DivideOpacityPlugin),
            Box::new(border::RingOffsetColorPlugin),
            Box::new(border::RingOffsetWidthPlugin),
            Box::new(border::RingColorPlugin),
            Box::new(border::RingWidthPlugin),
            Box::new(border::RingOpacityPlugin),
            Box::new(border::OutlineColorPlugin),
            Box::new(border::OutlineWidthPlugin),
            Box::new(border::OutlineStylePlugin),
            Box::new(border::OutlineOffsetPlugin),
            Box::new(typography::ColorPlugin),
            Box::new(typography::OpacityPlugin),
            Box::new(typography::FontFamilyPlugin),
            Box::new(typography::FontSizePlugin),
            Box::new(typography::FontWeightPlugin),
            Box::new(typography::TextAlignmentPlugin),
            Box::new(typography::TrackingPlugin),
            Box::new(typography::LeadingPlugin),
            Box::new(typography::TextDecorationColorPlugin),
            Box::new(typography::TextDecorationStylePlugin),
            Box::new(typography::TextDecorationThicknessPlugin),
            Box::new(typography::TextDecorationOffsetPlugin),
            Box::new(typography::ContentPlugin),
            Box::new(typography::ListStyleTypePlugin),
            Box::new(typography::ListStylePositionPlugin),
            Box::new(typography::VerticalAlignPlugin),
            Box::new(typography::WhitespacePlugin),
            Box::new(typography::WordBreakPlugin),
            Box::new(sizing::WidthPlugin),
            Box::new(sizing::MinWidthPlugin),
            Box::new(sizing::MaxWidthPlugin),
            Box::new(sizing::HeightPlugin),
            Box::new(sizing::MinHeightPlugin),
            Box::new(sizing::MaxHeightPlugin),
            Box::new(spacing::PaddingLeftPlugin),
            Box::new(spacing::PaddingRightPlugin),
            Box::new(spacing::PaddingTopPlugin),
            Box::new(spacing::PaddingBottomPlugin),
            Box::new(spacing::PaddingXPlugin),
            Box::new(spacing::PaddingYPlugin),
            Box::new(spacing::PaddingPlugin),
            Box::new(spacing::MarginLeftPlugin),
            Box::new(spacing::MarginRightPlugin),
            Box::new(spacing::MarginTopPlugin),
            Box::new(spacing::MarginBottomPlugin),
            Box::new(spacing::MarginXPlugin),
            Box::new(spacing::MarginYPlugin),
            Box::new(spacing::MarginPlugin),
            Box::new(spacing::SpaceXPlugin),
            Box::new(spacing::SpaceYPlugin),
            Box::new(flexbox::OrderPlugin),
            Box::new(flexbox::DirectionPlugin),
            Box::new(flexbox::WrapPlugin),
            Box::new(flexbox::GrowShrinkBasisPlugin),
            Box::new(layout::InsetPlugin),
            Box::new(layout::InsetXPlugin),
            Box::new(layout::InsetYPlugin),
            Box::new(layout::TopPlugin),
            Box::new(layout::BottomPlugin),
            Box::new(layout::LeftPlugin),
            Box::new(layout::RightPlugin),
            Box::new(layout::ZIndexPlugin),
            Box::new(layout::ContainerPlugin),
            Box::new(layout::BoxDecorationBreakPlugin),
            Box::new(layout::BoxSizingPlugin),
            Box::new(layout::FloatPlugin),
            Box::new(layout::ClearPlugin),
            Box::new(layout::IsolationPlugin),
            Box::new(layout::ObjectFitPlugin),
            Box::new(layout::ObjectPositionPlugin),
            Box::new(layout::OverflowPlugin),
            Box::new(layout::OverscrollPlugin),
            Box::new(alignment::AlignContentPlugin),
            Box::new(alignment::AlignItemsPlugin),
            Box::new(alignment::AlignSelfPlugin),
            Box::new(alignment::JustifyContentPlugin),
            Box::new(alignment::JustifyItemsPlugin),
            Box::new(alignment::JustifySelfPlugin),
            Box::new(alignment::PlaceContentPlugin),
            Box::new(alignment::PlaceItemsPlugin),
            Box::new(alignment::PlaceSelfPlugin),
            Box::new(effect::MixBlendModePlugin),
            Box::new(effect::OpacityPlugin),
            Box::new(effect::BackgroundBlendModePlugin),
            Box::new(effect::BoxShadowPlugin),
            Box::new(effect::BoxShadowColorPlugin),
            Box::new(transition::DurationPlugin),
            Box::new(transition::DelayPlugin),
            Box::new(transition::EasePlugin),
            Box::new(transition::PropertyPlugin),
            Box::new(transition::AnimatePlugin::new()),
            Box::new(grid::ColumnsPlugin),
            Box::new(grid::RowsPlugin),
            Box::new(grid::GapPlugin),
            Box::new(grid::GapXPlugin),
            Box::new(grid::GapYPlugin),
            Box::new(grid::StartEndSpanColumnPlugin),
            Box::new(grid::StartEndSpanRowPlugin),
            Box::new(grid::AutoFlowPlugin),
            Box::new(grid::AutoColumnsPlugin),
            Box::new(grid::AutoRowsPlugin),
            Box::new(interactivity::AccentColorPlugin),
            Box::new(interactivity::AppearancePlugin),
            Box::new(interactivity::CursorPlugin),
            Box::new(interactivity::CaretColorPlugin),
            Box::new(interactivity::PointerEventsPlugin),
            Box::new(interactivity::ResizePlugin),
            Box::new(interactivity::ScrollBehaviorPlugin),
            Box::new(interactivity::TouchActionPlugin),
            Box::new(interactivity::UserSelectPlugin),
            Box::new(interactivity::WillChangePlugin),
            Box::new(interactivity::ScrollSnapAlignPlugin),
            Box::new(interactivity::ScrollSnapStopPlugin),
            Box::new(interactivity::ScrollSnapTypePlugin),
            Box::new(interactivity::ScrollPaddingPlugin),
            Box::new(interactivity::ScrollPaddingXPlugin),
            Box::new(interactivity::ScrollPaddingYPlugin),
            Box::new(interactivity::ScrollPaddingLeftPlugin),
            Box::new(interactivity::ScrollPaddingRightPlugin),
            Box::new(interactivity::ScrollPaddingTopPlugin),
            Box::new(interactivity::ScrollPaddingBottomPlugin),
            Box::new(interactivity::ScrollMarginPlugin),
            Box::new(interactivity::ScrollMarginXPlugin),
            Box::new(interactivity::ScrollMarginYPlugin),
            Box::new(interactivity::ScrollMarginLeftPlugin),
            Box::new(interactivity::ScrollMarginRightPlugin),
            Box::new(interactivity::ScrollMarginTopPlugin),
            Box::new(interactivity::ScrollMarginBottomPlugin),
            Box::new(svg::FillPlugin),
            Box::new(svg::StrokeColorPlugin),
            Box::new(svg::StrokeWidthPlugin),
            Box::new(table::BorderCollapsePlugin),
            Box::new(table::TableLayoutPlugin),
            Box::new(transform::OriginPlugin),
            // It is better to include the following plugins at the end because they match the "" namespace
            Box::new(layout::DisplayPlugin),
            Box::new(layout::PositionPlugin),
            Box::new(layout::VisibilityPlugin),
            Box::new(filter::FilterPlugin),
            Box::new(filter::BackdropFilterPlugin),
            Box::new(typography::TextTransformPlugin),
            Box::new(typography::ItalicPlugin),
            Box::new(typography::TextDecorationPlugin),
            Box::new(typography::FontVariantNumericPlugin),
            Box::new(typography::FontSmoothingPlugin),
            Box::new(typography::TextOverflowPlugin),
            Box::new(accessibility::ScreenReaderPlugin),
            Box::new(transform::TranslateRotateScaleSkewPlugin),
        ]
    }

    /// Restore the default state of the generator (without any scanned selectors)
    /// Useful when repeatedly calling [`EncreGenerator::generate`]
    pub fn reset(&mut self) {
        self.scanned_selectors.clear();
    }
}
