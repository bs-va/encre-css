use crate::{config::Config, plugins::*, variant::VARIANT_SEPARATOR};

use std::cmp::Ordering;

#[cfg(feature = "rayon")]
use rayon::prelude::*;

pub const VALID_PLUGIN_HINT: [&str; 4] = ["color", "length", "angle", "list"];

/// The list of builtin plugins
// TODO: Better sorting (colors and lengths after all the other utilities (because they have
// hints))
static BUILTIN_PLUGINS: [&'static (dyn Plugin + Send + Sync); 206] = [
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Modifier<'a> {
    Basic { is_negative: bool, value: &'a str },
    Arbitrary { value: &'a str, hint: &'a str },
}

#[derive(Clone, Debug)]
pub struct Selector<'a> {
    pub(crate) full: &'a str,
    pub(crate) modifier: Modifier<'a>,
    pub(crate) variants: &'a str,
    pub(crate) is_important: bool,
    pub(crate) is_negative: bool,
    pub(crate) plugin: &'static (dyn Plugin + Sync + Send),
}

impl<'a> Selector<'a> {
    pub fn new(mut full: &'a str, config: &Config) -> Option<Self> {
        // Strip the important flag before the negative one
        let mut is_important = false;
        if full.starts_with('!') {
            full = &full[1..];
            is_important = true;
        }

        let mut is_negative = false;
        if full.starts_with('-') {
            full = &full[1..];
            is_negative = true;
        }

        // We need to ignore all characters in arbitrary values (wrapped in `[]`) and we know that
        // nothing interesting is placed after them, so we can just split by `[` and take the first
        // value
        let variants = {
            let before_arbitrary = full.split('[').next().unwrap();
            &before_arbitrary[..before_arbitrary.rfind(VARIANT_SEPARATOR).unwrap_or(0)]
        };

        // The selector without variants is the remaining part of the list of variants
        let content = if variants.is_empty() {
            full
        } else {
            full.strip_prefix(variants)?.strip_prefix(':')?
        };

        // Find the right plugin for handling this selector
        let find_fn = |plugin: &&'static (dyn Plugin + Send + Sync)| {
            // Find the modifier
            if let Some(modifier_part) = content.strip_prefix(&plugin.namespace()) {
                let modifier_part = modifier_part.strip_prefix('-').unwrap_or(modifier_part);

                let modifier = if let Some((_, mut after)) = modifier_part.split_once('[') {
                    after = after.strip_suffix(']')?;

                    if let Some((maybe_hint, rest)) = after.split_once(':') {
                        if VALID_PLUGIN_HINT.contains(&maybe_hint) {
                            Modifier::Arbitrary {
                                hint: maybe_hint,
                                value: rest,
                            }
                        } else {
                            Modifier::Arbitrary {
                                hint: "",
                                value: after,
                            }
                        }
                    } else {
                        Modifier::Arbitrary {
                            hint: "",
                            value: after,
                        }
                    }
                } else {
                    Modifier::Basic {
                        is_negative,
                        value: modifier_part,
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

        #[cfg(not(feature = "rayon"))]
        let result = BUILTIN_PLUGINS.iter().find_map(find_fn);

        #[cfg(feature = "rayon")]
        let result = BUILTIN_PLUGINS.par_iter().find_map_first(find_fn);

        if let Some(result) = result {
            Some(Self {
                full,
                variants,
                modifier: result.1,
                is_important,
                is_negative,
                plugin: result.0,
            })
        } else {
            None
        }
    }
}

impl<'a> PartialEq for Selector<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.full == other.full
    }
}

impl<'a> Eq for Selector<'a> {}

impl<'a> PartialOrd for Selector<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(if self.variants.is_empty() && !other.variants.is_empty() {
            Ordering::Less
        } else if !self.variants.is_empty() && other.variants.is_empty() {
            Ordering::Greater
        } else {
            self.full.cmp(other.full)
        })
    }
}

impl<'a> Ord for Selector<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.variants.is_empty() && !other.variants.is_empty() {
            Ordering::Less
        } else if !self.variants.is_empty() && other.variants.is_empty() {
            Ordering::Greater
        } else {
            self.full.cmp(other.full)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{config::Config, selector::Selector};

    use std::collections::BTreeSet;

    #[test]
    fn sorting_test() {
        let config = Config::default();
        let mut selectors = BTreeSet::new();
        selectors.insert(Selector::new("lg:bg-red-500", &config).unwrap());
        selectors.insert(Selector::new("bg-red-500", &config).unwrap());

        assert_eq!(
            selectors.iter().collect::<Vec<&Selector>>(),
            vec![
                &Selector::new("bg-red-500", &config).unwrap(),
                &Selector::new("lg:bg-red-500", &config).unwrap(),
            ]
        );
    }
}
