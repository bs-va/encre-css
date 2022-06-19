use crate::{config::Config, plugins::*, variant::VARIANT_SEPARATOR};

use derivative::Derivative;
use lazy_static::lazy_static;
use regex::Regex;
use smol_str::SmolStr;

#[cfg(not(target_arch = "wasm32"))]
use rayon::prelude::*;

lazy_static! {
    static ref VARIANT_REGEX: Regex =
        Regex::new(&format!(r"^[^\[]*{}", VARIANT_SEPARATOR)).unwrap();
    static ref ARBITRARY_VALUE_REGEX: Regex =
        Regex::new(&format!(r"\[([a-zA-Z0-9-_]+{})?(.+)\]$", VARIANT_SEPARATOR)).unwrap();

    /// The list of builtin plugins
    // TODO: Better sorting (colors and lengths after all the other utilities (because they have
    // hints))
    static ref BUILTIN_PLUGINS: [&'static (dyn Plugin + Send + Sync); 206] = [
        &background::ColorPlugin,
        &background::AttachmentPlugin,
        &background::ClipPlugin,
        &background::OpacityPlugin,
        &background::ImagePlugin,
        &background::GradientFromPlugin,
        &background::GradientViaPlugin,
        &background::GradientToPlugin,
        &background::PositionPlugin,
        &background::RepeatPlugin,
        &background::SizePlugin,
        &border::ColorPlugin,
        &border::RadiusTopRightPlugin,
        &border::RadiusTopLeftPlugin,
        &border::RadiusBottomRightPlugin,
        &border::RadiusBottomLeftPlugin,
        &border::RadiusTopPlugin,
        &border::RadiusBottomPlugin,
        &border::RadiusLeftPlugin,
        &border::RadiusRightPlugin,
        &border::RadiusPlugin,
        &border::StylePlugin,
        &border::WidthTopPlugin,
        &border::WidthBottomPlugin,
        &border::WidthLeftPlugin,
        &border::WidthRightPlugin,
        &border::WidthXPlugin,
        &border::WidthYPlugin,
        &border::WidthPlugin,
        &border::OpacityPlugin,
        &border::DivideColorPlugin,
        &border::DivideWidthXPlugin,
        &border::DivideWidthYPlugin,
        &border::DivideStylePlugin,
        &border::DivideOpacityPlugin,
        &border::RingOffsetColorPlugin,
        &border::RingOffsetWidthPlugin,
        &border::RingColorPlugin,
        &border::RingWidthPlugin,
        &border::RingOpacityPlugin,
        &border::OutlineColorPlugin,
        &border::OutlineWidthPlugin,
        &border::OutlineStylePlugin,
        &border::OutlineOffsetPlugin,
        &typography::ColorPlugin,
        &typography::OpacityPlugin,
        &typography::FontFamilyPlugin,
        &typography::FontSizePlugin,
        &typography::FontWeightPlugin,
        &typography::TextAlignmentPlugin,
        &typography::TrackingPlugin,
        &typography::LeadingPlugin,
        &typography::TextDecorationColorPlugin,
        &typography::TextDecorationStylePlugin,
        &typography::TextDecorationThicknessPlugin,
        &typography::TextDecorationOffsetPlugin,
        &typography::ContentPlugin,
        &typography::ListStyleTypePlugin,
        &typography::ListStylePositionPlugin,
        &typography::VerticalAlignPlugin,
        &typography::WhitespacePlugin,
        &typography::WordBreakPlugin,
        &sizing::WidthPlugin,
        &sizing::MinWidthPlugin,
        &sizing::MaxWidthPlugin,
        &sizing::HeightPlugin,
        &sizing::MinHeightPlugin,
        &sizing::MaxHeightPlugin,
        &spacing::PaddingLeftPlugin,
        &spacing::PaddingRightPlugin,
        &spacing::PaddingTopPlugin,
        &spacing::PaddingBottomPlugin,
        &spacing::PaddingXPlugin,
        &spacing::PaddingYPlugin,
        &spacing::PaddingPlugin,
        &spacing::MarginLeftPlugin,
        &spacing::MarginRightPlugin,
        &spacing::MarginTopPlugin,
        &spacing::MarginBottomPlugin,
        &spacing::MarginXPlugin,
        &spacing::MarginYPlugin,
        &spacing::MarginPlugin,
        &spacing::SpaceXPlugin,
        &spacing::SpaceYPlugin,
        &flexbox::OrderPlugin,
        &flexbox::DirectionPlugin,
        &flexbox::WrapPlugin,
        &flexbox::GrowShrinkBasisPlugin,
        &layout::InsetPlugin,
        &layout::InsetXPlugin,
        &layout::InsetYPlugin,
        &layout::TopPlugin,
        &layout::BottomPlugin,
        &layout::LeftPlugin,
        &layout::RightPlugin,
        &layout::ZIndexPlugin,
        &layout::ContainerPlugin,
        &layout::BoxDecorationBreakPlugin,
        &layout::BoxSizingPlugin,
        &layout::FloatPlugin,
        &layout::ClearPlugin,
        &layout::IsolationPlugin,
        &layout::ObjectFitPlugin,
        &layout::ObjectPositionPlugin,
        &layout::OverflowPlugin,
        &layout::OverscrollPlugin,
        &alignment::AlignContentPlugin,
        &alignment::AlignItemsPlugin,
        &alignment::AlignSelfPlugin,
        &alignment::JustifyContentPlugin,
        &alignment::JustifyItemsPlugin,
        &alignment::JustifySelfPlugin,
        &alignment::PlaceContentPlugin,
        &alignment::PlaceItemsPlugin,
        &alignment::PlaceSelfPlugin,
        &transition::DurationPlugin,
        &transition::DelayPlugin,
        &transition::EasePlugin,
        &transition::PropertyPlugin,
        &transition::AnimatePlugin,
        &grid::ColumnsPlugin,
        &grid::RowsPlugin,
        &grid::GapPlugin,
        &grid::GapXPlugin,
        &grid::GapYPlugin,
        &grid::StartEndSpanColumnPlugin,
        &grid::StartEndSpanRowPlugin,
        &grid::AutoFlowPlugin,
        &grid::AutoColumnsPlugin,
        &grid::AutoRowsPlugin,
        &interactivity::AccentColorPlugin,
        &interactivity::AppearancePlugin,
        &interactivity::CursorPlugin,
        &interactivity::CaretColorPlugin,
        &interactivity::PointerEventsPlugin,
        &interactivity::ResizePlugin,
        &interactivity::ScrollBehaviorPlugin,
        &interactivity::TouchActionPlugin,
        &interactivity::UserSelectPlugin,
        &interactivity::WillChangePlugin,
        &interactivity::ScrollSnapAlignPlugin,
        &interactivity::ScrollSnapStopPlugin,
        &interactivity::ScrollSnapTypePlugin,
        &interactivity::ScrollPaddingPlugin,
        &interactivity::ScrollPaddingXPlugin,
        &interactivity::ScrollPaddingYPlugin,
        &interactivity::ScrollPaddingLeftPlugin,
        &interactivity::ScrollPaddingRightPlugin,
        &interactivity::ScrollPaddingTopPlugin,
        &interactivity::ScrollPaddingBottomPlugin,
        &interactivity::ScrollMarginPlugin,
        &interactivity::ScrollMarginXPlugin,
        &interactivity::ScrollMarginYPlugin,
        &interactivity::ScrollMarginLeftPlugin,
        &interactivity::ScrollMarginRightPlugin,
        &interactivity::ScrollMarginTopPlugin,
        &interactivity::ScrollMarginBottomPlugin,
        &svg::FillPlugin,
        &svg::StrokeColorPlugin,
        &svg::StrokeWidthPlugin,
        &table::BorderCollapsePlugin,
        &table::TableLayoutPlugin,
        &transform::OriginPlugin,
        &transform::TranslateXPlugin,
        &transform::TranslateYPlugin,
        &transform::RotatePlugin,
        &transform::ScalePlugin,
        &transform::ScaleXPlugin,
        &transform::ScaleYPlugin,
        &transform::SkewXPlugin,
        &transform::SkewYPlugin,
        &filter::FilterPlugin,
        &filter::BlurPlugin,
        &filter::BrightnessPlugin,
        &filter::ContrastPlugin,
        &filter::DropShadowPlugin,
        &filter::GrayscalePlugin,
        &filter::HueRotatePlugin,
        &filter::InvertPlugin,
        &filter::SaturatePlugin,
        &filter::SepiaPlugin,
        &filter::BackdropFilterPlugin,
        &filter::BackdropBlurPlugin,
        &filter::BackdropBrightnessPlugin,
        &filter::BackdropContrastPlugin,
        &filter::BackdropGrayscalePlugin,
        &filter::BackdropHueRotatePlugin,
        &filter::BackdropInvertPlugin,
        &filter::BackdropOpacityPlugin,
        &filter::BackdropSaturatePlugin,
        &filter::BackdropSepiaPlugin,
        &effect::MixBlendModePlugin,
        &effect::OpacityPlugin,
        &effect::BackgroundBlendModePlugin,
        &effect::BoxShadowPlugin,
        &effect::BoxShadowColorPlugin,

        // It is better to include the following plugins at the end because they match the "" namespace
        &layout::DisplayPlugin,
        &layout::PositionPlugin,
        &layout::VisibilityPlugin,
        &typography::TextTransformPlugin,
        &typography::ItalicPlugin,
        &typography::TextDecorationPlugin,
        &typography::FontVariantNumericPlugin,
        &typography::FontSmoothingPlugin,
        &typography::TextOverflowPlugin,
        &accessibility::ScreenReaderPlugin,
    ];
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Modifier {
    Basic { is_negative: bool, value: SmolStr },
    Arbitrary { value: SmolStr, hint: SmolStr },
}

#[derive(Clone, Derivative)]
#[derivative(Debug, PartialEq, Eq)]
pub struct Selector {
    pub(crate) full: SmolStr,
    pub(crate) content: String,
    pub(crate) modifier: Modifier,
    pub(crate) variants: Option<Vec<String>>,
    pub(crate) is_important: bool,
    pub(crate) is_negative: bool,

    #[derivative(Debug = "ignore")]
    #[derivative(PartialEq = "ignore")]
    pub(crate) plugin: &'static (dyn Plugin + Sync + Send),
}

impl Selector {
    pub fn new<T: Into<SmolStr>>(data: T, config: &Config) -> Option<Self> {
        let mut data = data.into();
        let full = data.clone();

        // Strip the important flag before the negative one
        let mut is_important = false;
        if let Some(new_data) = data.strip_prefix('!') {
            data = SmolStr::new(new_data);
            is_important = true;
        }

        let mut is_negative = false;
        if let Some(new_data) = data.strip_prefix('-') {
            data = SmolStr::new(new_data);
            is_negative = true;
        }

        let mut variants = vec![];
        let mut next_variant = true;
        let mut in_square_bracket = false;

        data.chars().for_each(|ch| {
            match ch {
                '[' => in_square_bracket = true,
                ']' => in_square_bracket = false,
                VARIANT_SEPARATOR => {
                    if !in_square_bracket {
                        next_variant = true;
                        return;
                    }
                }
                _ => (),
            }

            if next_variant {
                variants.push(ch.to_string());
                next_variant = false;
            } else {
                // We can safely unwrap because `next_variant` is `true` by default, so the `Vec`
                // is bound to contain at least one element
                variants.last_mut().unwrap().push(ch);
            }
        });

        // The selector without variants is the remaining part of the list of variants
        let content = variants.pop().unwrap();

        // Find the right plugin for handling this selector
        let find_fn = |plugin: &&'static (dyn Plugin + Send + Sync)| {
            // Find the modifier
            if let Some(modifier_part) =
                content.strip_prefix(&plugin.namespace().replace('-', &*config.modifier_separator))
            {
                let modifier_part = modifier_part
                    .strip_prefix(&**config.modifier_separator)
                    .unwrap_or(modifier_part);

                let modifier = if let Some(caps) = ARBITRARY_VALUE_REGEX.captures(modifier_part) {
                    Modifier::Arbitrary {
                        hint: SmolStr::from(
                            caps.get(1)
                                .map(|c| c.as_str())
                                .unwrap_or("")
                                .trim_end_matches(':'),
                        ),
                        value: SmolStr::from(caps.get(2)?.as_str()),
                    }
                } else {
                    Modifier::Basic {
                        is_negative,
                        value: SmolStr::from(modifier_part),
                    }
                };

                if plugin.can_handle(config, &modifier) {
                    Some((*plugin, modifier))
                } else {
                    None
                }
            } else {
                None
            }
        };

        #[cfg(target_arch = "wasm32")]
        let result = BUILTIN_PLUGINS.iter().find_map(find_fn);

        #[cfg(not(target_arch = "wasm32"))]
        let result = BUILTIN_PLUGINS.par_iter().find_map_first(find_fn);

        if let Some(result) = result {
            Some(Self {
                full,
                variants: if !variants.is_empty() {
                    Some(variants)
                } else {
                    None
                },
                content,
                modifier: result.1,
                is_important,
                is_negative,
                plugin: result.0,
            })
        } else {
            trace!("Plugin not found for handling `{}`", full);
            None
        }
    }
}
