use crate::{
    config::Config,
    context::ContextCanHandle,
    plugins::*,
    variant::{Variant, BUILTIN_VARIANTS, VARIANT_SEPARATOR},
};

use std::{borrow::Cow, cmp::Ordering, collections::BTreeMap};

#[cfg(feature = "rayon")]
use rayon::prelude::*;

pub const VALID_PLUGIN_HINT: [&str; 4] = ["color", "length", "angle", "list"];

/// The list of builtin plugins
///
/// Sorted following https://github.com/avencera/rustywind/blob/master/src/defaults.rs
static BUILTIN_PLUGINS: [&'static (dyn Plugin + Send + Sync); 223] = [
    &layout::ContainerPlugin,
    &accessibility::ScreenReaderPlugin,
    &interactivity::PointerEventsPlugin,
    &layout::VisibilityPlugin,
    &layout::PositionPlugin,
    &layout::InsetPlugin,
    &layout::InsetXPlugin,
    &layout::InsetYPlugin,
    &layout::TopPlugin,
    &layout::RightPlugin,
    &layout::BottomPlugin,
    &layout::LeftPlugin,
    &layout::IsolationPlugin,
    &layout::ZIndexPlugin,
    &flexbox::OrderPlugin,
    &grid::StartEndSpanColumnPlugin,
    &grid::StartEndSpanRowPlugin,
    &layout::FloatPlugin,
    &layout::ClearPlugin,
    &spacing::MarginPlugin,
    &spacing::MarginXPlugin,
    &spacing::MarginYPlugin,
    &spacing::MarginTopPlugin,
    &spacing::MarginRightPlugin,
    &spacing::MarginBottomPlugin,
    &spacing::MarginLeftPlugin,
    &layout::BoxSizingPlugin,
    &layout::DisplayPlugin,
    &layout::AspectRatioPlugin,
    &sizing::HeightPlugin,
    &sizing::MaxHeightPlugin,
    &sizing::MinHeightPlugin,
    &sizing::WidthPlugin,
    &sizing::MinWidthPlugin,
    &sizing::MaxWidthPlugin,
    &flexbox::GrowShrinkBasisPlugin,
    &table::TableLayoutPlugin,
    &table::BorderCollapsePlugin,
    &table::BorderSpacingPlugin,
    &table::BorderSpacingXPlugin,
    &table::BorderSpacingYPlugin,
    &transform::OriginPlugin,
    &transform::TranslateXPlugin,
    &transform::TranslateYPlugin,
    &transform::RotatePlugin,
    &transform::SkewXPlugin,
    &transform::SkewYPlugin,
    &transform::ScalePlugin,
    &transform::ScaleXPlugin,
    &transform::ScaleYPlugin,
    &transform::TransformPlugin,
    &transition::AnimatePlugin,
    &interactivity::CursorPlugin,
    &interactivity::TouchActionPlugin,
    &interactivity::UserSelectPlugin,
    &interactivity::ResizePlugin,
    &interactivity::ScrollSnapTypePlugin,
    &interactivity::ScrollSnapAlignPlugin,
    &interactivity::ScrollSnapStopPlugin,
    &interactivity::ScrollMarginPlugin,
    &interactivity::ScrollMarginXPlugin,
    &interactivity::ScrollMarginYPlugin,
    &interactivity::ScrollMarginTopPlugin,
    &interactivity::ScrollMarginRightPlugin,
    &interactivity::ScrollMarginBottomPlugin,
    &interactivity::ScrollMarginLeftPlugin,
    &interactivity::ScrollPaddingPlugin,
    &interactivity::ScrollPaddingXPlugin,
    &interactivity::ScrollPaddingYPlugin,
    &interactivity::ScrollPaddingTopPlugin,
    &interactivity::ScrollPaddingRightPlugin,
    &interactivity::ScrollPaddingBottomPlugin,
    &interactivity::ScrollPaddingLeftPlugin,
    &typography::ListStylePositionPlugin,
    &typography::ListStyleTypePlugin,
    &interactivity::AppearancePlugin,
    &layout::ColumnsPlugin,
    &layout::BreakBeforePlugin,
    &layout::BreakInsidePlugin,
    &layout::BreakAfterPlugin,
    &grid::AutoColumnsPlugin,
    &grid::AutoFlowPlugin,
    &grid::AutoRowsPlugin,
    &grid::TemplateColumnsPlugin,
    &grid::TemplateRowsPlugin,
    &flexbox::DirectionPlugin,
    &flexbox::WrapPlugin,
    &alignment::PlaceContentPlugin,
    &alignment::PlaceItemsPlugin,
    &alignment::AlignContentPlugin,
    &alignment::AlignItemsPlugin,
    &alignment::JustifyContentPlugin,
    &alignment::JustifyItemsPlugin,
    &grid::GapPlugin,
    &grid::GapXPlugin,
    &grid::GapYPlugin,
    &spacing::SpaceYPlugin,
    &spacing::SpaceXPlugin,
    &border::DivideWidthXPlugin,
    &border::DivideWidthYPlugin,
    &border::DivideStylePlugin,
    &border::DivideColorPlugin,
    &border::DivideOpacityPlugin,
    &alignment::PlaceSelfPlugin,
    &alignment::AlignSelfPlugin,
    &alignment::JustifySelfPlugin,
    &layout::OverflowPlugin,
    &layout::OverscrollPlugin,
    &interactivity::ScrollBehaviorPlugin,
    &typography::TextOverflowPlugin,
    &typography::WhitespacePlugin,
    &typography::WordBreakPlugin,
    &border::RadiusPlugin,
    &border::RadiusTopPlugin,
    &border::RadiusRightPlugin,
    &border::RadiusBottomPlugin,
    &border::RadiusLeftPlugin,
    &border::RadiusTopLeftPlugin,
    &border::RadiusTopRightPlugin,
    &border::RadiusBottomRightPlugin,
    &border::RadiusBottomLeftPlugin,
    &border::WidthPlugin,
    &border::WidthXPlugin,
    &border::WidthYPlugin,
    &border::WidthTopPlugin,
    &border::WidthRightPlugin,
    &border::WidthBottomPlugin,
    &border::WidthLeftPlugin,
    &border::StylePlugin,
    &border::ColorPlugin,
    &border::ColorXPlugin,
    &border::ColorYPlugin,
    &border::ColorTopPlugin,
    &border::ColorRightPlugin,
    &border::ColorBottomPlugin,
    &border::ColorLeftPlugin,
    &border::OpacityPlugin,
    &background::ColorPlugin,
    &background::OpacityPlugin,
    &background::ImagePlugin,
    &background::GradientFromPlugin,
    &background::GradientViaPlugin,
    &background::GradientToPlugin,
    &layout::BoxDecorationBreakPlugin,
    &background::SizePlugin,
    &background::AttachmentPlugin,
    &background::ClipPlugin,
    &background::PositionPlugin,
    &background::RepeatPlugin,
    &background::OriginPlugin,
    &svg::FillPlugin,
    &svg::StrokeColorPlugin,
    &svg::StrokeWidthPlugin,
    &layout::ObjectFitPlugin,
    &layout::ObjectPositionPlugin,
    &spacing::PaddingPlugin,
    &spacing::PaddingXPlugin,
    &spacing::PaddingYPlugin,
    &spacing::PaddingTopPlugin,
    &spacing::PaddingRightPlugin,
    &spacing::PaddingBottomPlugin,
    &spacing::PaddingLeftPlugin,
    &typography::TextAlignmentPlugin,
    &typography::TextIndentPlugin,
    &typography::VerticalAlignPlugin,
    &typography::FontFamilyPlugin,
    &typography::FontSizePlugin,
    &typography::FontWeightPlugin,
    &typography::TextTransformPlugin,
    &typography::ItalicPlugin,
    &typography::FontVariantNumericPlugin,
    &typography::LeadingPlugin,
    &typography::TrackingPlugin,
    &typography::ColorPlugin,
    &typography::OpacityPlugin,
    &typography::TextDecorationPlugin,
    &typography::TextDecorationColorPlugin,
    &typography::TextDecorationStylePlugin,
    &typography::TextDecorationThicknessPlugin,
    &typography::TextDecorationUnderlineOffsetPlugin,
    &typography::FontSmoothingPlugin,
    &interactivity::CaretColorPlugin,
    &interactivity::AccentColorPlugin,
    &effect::OpacityPlugin,
    &effect::BackgroundBlendModePlugin,
    &effect::MixBlendModePlugin,
    &effect::BoxShadowPlugin,
    &effect::BoxShadowColorPlugin,
    &border::OutlineStylePlugin,
    &border::OutlineWidthPlugin,
    &border::OutlineOffsetPlugin,
    &border::OutlineColorPlugin,
    &border::RingWidthPlugin,
    &border::RingColorPlugin,
    &border::RingOpacityPlugin,
    &border::RingOffsetWidthPlugin,
    &border::RingOffsetColorPlugin,
    &filter::BlurPlugin,
    &filter::BrightnessPlugin,
    &filter::ContrastPlugin,
    &filter::DropShadowPlugin,
    &filter::GrayscalePlugin,
    &filter::HueRotatePlugin,
    &filter::InvertPlugin,
    &filter::SaturatePlugin,
    &filter::SepiaPlugin,
    &filter::FilterPlugin,
    &filter::BackdropBlurPlugin,
    &filter::BackdropBrightnessPlugin,
    &filter::BackdropContrastPlugin,
    &filter::BackdropGrayscalePlugin,
    &filter::BackdropHueRotatePlugin,
    &filter::BackdropInvertPlugin,
    &filter::BackdropOpacityPlugin,
    &filter::BackdropSaturatePlugin,
    &filter::BackdropSepiaPlugin,
    &filter::BackdropFilterPlugin,
    &transition::PropertyPlugin,
    &transition::DelayPlugin,
    &transition::DurationPlugin,
    &transition::EasePlugin,
    &interactivity::WillChangePlugin,
    &typography::ContentPlugin,
];

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Modifier<'a> {
    Basic {
        is_negative: bool,
        value: &'a str,
    },
    Arbitrary {
        prefix: &'a str,
        hint: &'a str,
        value: &'a str,
    },
}

#[derive(Clone, Debug)]
pub struct Selector<'a> {
    pub(crate) order: usize,
    pub(crate) full: &'a str,
    pub(crate) modifier: Modifier<'a>,
    pub(crate) variants: &'a str,
    pub(crate) is_important: bool,
    pub(crate) plugin: &'static (dyn Plugin + Sync + Send),
}

impl<'a> Selector<'a> {
    pub fn new(full: &'a str, config: &Config) -> Option<Self> {
        // We need to ignore all characters in arbitrary values (wrapped in `[]`) and we know that
        // nothing interesting is placed after them, so we can just split by `[` and take the first
        // value
        let variants = {
            let before_arbitrary = full.split('[').next().unwrap();
            &before_arbitrary[..before_arbitrary.rfind(VARIANT_SEPARATOR).unwrap_or(0)]
        };

        // The selector without variants is the remaining part of the list of variants
        let mut content = if variants.is_empty() {
            full
        } else {
            full.strip_prefix(variants)?.strip_prefix(':')?
        };

        // Strip the important flag before the negative one
        let mut is_important = false;
        if content.starts_with('!') {
            content = &content[1..];
            is_important = true;
        }

        let mut is_negative = false;
        if content.starts_with('-') {
            content = &content[1..];
            is_negative = true;
        }

        // Find the right plugin for handling this selector
        let find_fn = |(i, plugin): (usize, &&'static (dyn Plugin + Send + Sync))| {
            // Find the modifier
            if let Some(modifier_part) = content.strip_prefix(&plugin.namespace()) {
                let modifier_part = modifier_part.strip_prefix('-').unwrap_or(modifier_part);

                let modifier = if let Some((mut prefix, mut after)) = modifier_part.split_once('[')
                {
                    prefix = prefix.strip_suffix('-').unwrap_or(prefix);
                    after = after.strip_suffix(']')?;

                    if let Some((maybe_hint, rest)) = after.split_once(':') {
                        if VALID_PLUGIN_HINT.contains(&maybe_hint) {
                            Modifier::Arbitrary {
                                prefix,
                                hint: maybe_hint,
                                value: rest,
                            }
                        } else {
                            Modifier::Arbitrary {
                                prefix,
                                hint: "",
                                value: after,
                            }
                        }
                    } else {
                        Modifier::Arbitrary {
                            prefix,
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

                let context = ContextCanHandle {
                    config,
                    modifier: &modifier,
                };

                if plugin.can_handle(context) {
                    Some((i, *plugin, modifier))
                } else {
                    None
                }
            } else {
                None
            }
        };

        #[cfg(not(feature = "rayon"))]
        let result = BUILTIN_PLUGINS.iter().enumerate().find_map(find_fn);

        #[cfg(feature = "rayon")]
        let result = BUILTIN_PLUGINS
            .par_iter()
            .enumerate()
            .find_map_first(find_fn);

        if let Some((order, plugin, modifier)) = result {
            Some(Self {
                order,
                full,
                variants,
                modifier,
                is_important,
                plugin,
            })
        } else {
            None
        }
    }

    pub fn get_css_class(&self, custom_variants: &BTreeMap<Cow<str>, Variant>) -> String {
        let mut base_class = ".".to_string()
            + &self
                .full
                .chars()
                .enumerate()
                .map(|(i, ch)| {
                    if i == 0 {
                        if ch.is_numeric() {
                            // CSS classes must not start with a number, we need to escape it
                            "\\3".to_string() + &ch.to_string()
                        } else {
                            ch.to_string()
                        }
                    } else if !ch.is_alphanumeric() && ch != '-' && ch != '_' {
                        format!("\\{}", ch)
                    } else {
                        ch.to_string()
                    }
                })
                .collect::<String>();

        if !self.variants.is_empty() {
            self.variants
                .split(VARIANT_SEPARATOR)
                .rev()
                .for_each(|variant| {
                    if let Some(variant) = BUILTIN_VARIANTS
                        .iter()
                        .find_map(|v| if v.0 == variant { Some(&v.1) } else { None })
                        .or_else(|| custom_variants.get(&Cow::from(variant)))
                    {
                        if let Variant::WrapClass(template) = variant {
                            base_class = template.replace('&', &base_class);
                        }
                    }
                });
        }

        base_class
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
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Selector<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.variants.is_empty() && !other.variants.is_empty() {
            Ordering::Less
        } else if !self.variants.is_empty() && other.variants.is_empty() {
            Ordering::Greater
        } else {
            self.order
                .cmp(&other.order)
                .then_with(|| self.full.cmp(other.full))
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
