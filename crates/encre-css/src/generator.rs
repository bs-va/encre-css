use crate::{
    config::Config,
    extractor::Extractor,
    plugins::*,
    preflight::ENCRE_PREFLIGHT_CSS,
    selector::{Modifier, Selector},
    utils::indent,
    variant::{init_variants, Variant},
    error::{Result, Error},
};

use lazy_static::lazy_static;
use regex::{Captures, Regex};
use smol_str::SmolStr;
use std::{
    borrow::Cow,
    collections::{BTreeMap, BTreeSet},
    fmt::Write,
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
    pub fn generate(&self) -> Result<String> {
        debug!("Start generating CSS");
        let plugins = self.build_plugins();

        // TODO: Is parallelism possible without bad sorting of selectors?
        let mut iter = self.scanned_selectors.iter();

        let mut buffer = String::with_capacity(10 * self.scanned_selectors.len()); // TODO: More accurate value
        buffer.push_str(ENCRE_PREFLIGHT_CSS); // TODO: Push and reserve at the same time

        iter.try_for_each(|selector| {
            let mut not_found = true;

            for plugin in &plugins {
                if let Some(mut modifier) = selector.modifier(
                    &self.config,
                    &plugin
                        .namespace()
                        .replace('-', &*self.config.modifier_separator),
                ) {
                    if plugin.can_handle(&self.config, &modifier) {
                        write!(buffer, "\n\n")?;

                        plugin.css_before_rule(&modifier, &mut buffer)?;

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
                        if let Modifier::Arbitrary { ref mut value, .. } = modifier {
                            // Transform the mangled CSS content of the selector into a real CSS rule
                            *value = to_css_value(value);
                        }

                        // TODO: Support the important prefix
                        plugin.handle(&self.config, &modifier, indentation + 1, &mut buffer)?;

                        // After rule
                        for i in (1..indentation + 1).rev() {
                            indent(i, &mut buffer)?;
                            writeln!(buffer, "}}")?;
                        }

                        write!(buffer, "}}")?;

                        // The plugin is found, move on to the next selector
                        not_found = false;
                        break;
                    }
                }
            }

            if not_found {
                trace!("Plugin not found for handling `{}`", selector.full);
            }

            Ok::<(), Error>(())
        })?;

        debug!("Finished generating CSS");

        Ok(buffer)
    }

    /// Return the list of plugins needed
    pub fn build_plugins(&self) -> [Box<dyn Plugin + Send + Sync>; 206] {
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
            Box::new(border::RadiusTopRightPlugin),
            Box::new(border::RadiusTopLeftPlugin),
            Box::new(border::RadiusBottomRightPlugin),
            Box::new(border::RadiusBottomLeftPlugin),
            Box::new(border::RadiusTopPlugin),
            Box::new(border::RadiusBottomPlugin),
            Box::new(border::RadiusLeftPlugin),
            Box::new(border::RadiusRightPlugin),
            Box::new(border::RadiusPlugin),
            Box::new(border::StylePlugin),
            Box::new(border::WidthTopPlugin),
            Box::new(border::WidthBottomPlugin),
            Box::new(border::WidthLeftPlugin),
            Box::new(border::WidthRightPlugin),
            Box::new(border::WidthXPlugin),
            Box::new(border::WidthYPlugin),
            Box::new(border::WidthPlugin),
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
            Box::new(transform::TranslateXPlugin),
            Box::new(transform::TranslateYPlugin),
            Box::new(transform::RotatePlugin),
            Box::new(transform::ScalePlugin),
            Box::new(transform::ScaleXPlugin),
            Box::new(transform::ScaleYPlugin),
            Box::new(transform::SkewXPlugin),
            Box::new(transform::SkewYPlugin),
            Box::new(filter::FilterPlugin),
            Box::new(filter::BlurPlugin),
            Box::new(filter::BrightnessPlugin),
            Box::new(filter::ContrastPlugin),
            Box::new(filter::DropShadowPlugin),
            Box::new(filter::GrayscalePlugin),
            Box::new(filter::HueRotatePlugin),
            Box::new(filter::InvertPlugin),
            Box::new(filter::SaturatePlugin),
            Box::new(filter::SepiaPlugin),
            Box::new(filter::BackdropFilterPlugin),
            Box::new(filter::BackdropBlurPlugin),
            Box::new(filter::BackdropBrightnessPlugin),
            Box::new(filter::BackdropContrastPlugin),
            Box::new(filter::BackdropGrayscalePlugin),
            Box::new(filter::BackdropHueRotatePlugin),
            Box::new(filter::BackdropInvertPlugin),
            Box::new(filter::BackdropOpacityPlugin),
            Box::new(filter::BackdropSaturatePlugin),
            Box::new(filter::BackdropSepiaPlugin),
            Box::new(effect::MixBlendModePlugin),
            Box::new(effect::OpacityPlugin),
            Box::new(effect::BackgroundBlendModePlugin),
            Box::new(effect::BoxShadowPlugin),
            Box::new(effect::BoxShadowColorPlugin),
            // It is better to include the following plugins at the end because they match the "" namespace
            Box::new(layout::DisplayPlugin),
            Box::new(layout::PositionPlugin),
            Box::new(layout::VisibilityPlugin),
            Box::new(typography::TextTransformPlugin),
            Box::new(typography::ItalicPlugin),
            Box::new(typography::TextDecorationPlugin),
            Box::new(typography::FontVariantNumericPlugin),
            Box::new(typography::FontSmoothingPlugin),
            Box::new(typography::TextOverflowPlugin),
            Box::new(accessibility::ScreenReaderPlugin),
        ]
    }

    /// Restore the default state of the generator (without any scanned selectors)
    /// Useful when repeatedly calling [`EncreGenerator::generate`]
    pub fn reset(&mut self) {
        self.scanned_selectors.clear();
    }
}
