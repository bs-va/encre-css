use crate::{config::Config, selector::Modifier};

use lazy_static::lazy_static;
use std::fmt;

pub mod accessibility;
pub mod alignment;
pub mod background;
pub mod border;
pub mod effect;
pub mod filter;
pub mod flexbox;
pub mod grid;
pub mod interactivity;
pub mod layout;
pub mod sizing;
pub mod spacing;
pub mod svg;
pub mod table;
pub mod transform;
pub mod transition;
pub mod typography;

pub trait Plugin: fmt::Debug {
    /// Returns the namespace containing the plugin
    ///
    /// By default, the plugin does not belong to a namespace
    fn namespace(&self) -> &str {
        ""
    }

    /// Returns whether of not the plugin can handle a specific arbitrary value
    ///
    /// Used to distinguish plugins inside the same namespace
    ///
    /// By default, arbitrary values are disallowed
    ///
    /// The `hint` argument can be ignored, for example if the namespace contains a single plugin
    fn is_matching_value(&self, _hint: &str, _val: &str) -> bool {
        false
    }

    /// Get the template for an arbitrary associated with the plugin
    ///
    /// Returns whether the function handled the modifier
    ///
    /// NOTE: This function is called after [to_css_value], so, `_` (underscores) are already converted to ` ` (spaces)
    ///
    /// [to_css_value]: crate::to_css_value
    fn css_template_value(&self, _val: &str, _css_content: &mut String) -> bool {
        false
    }

    /// Get the CSS code from a modifier
    ///
    /// Returns whether the function handled the modifier
    fn get_css_for_modifier(&self, _config: &Config, modifier: &Modifier, css_content: &mut String) -> bool;

    // TODO: fn custom_css(&self) -> String; (custom CSS added only if plugin used at least once, e.g. for animations or for filters and transforms (avoid repeat CSS_FILTER))
}

lazy_static! {
    // TODO: Better sorting (colors and lengths after all the other utilities (because they have
    // hints))
    pub static ref PLUGINS: &'static [&'static (dyn Plugin + Sync)] = &[
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
        &border::RadiusPlugin,
        &border::RadiusTopPlugin,
        &border::RadiusBottomPlugin,
        &border::RadiusLeftPlugin,
        &border::RadiusRightPlugin,
        &border::RadiusTopLeftPlugin,
        &border::RadiusTopRightPlugin,
        &border::RadiusBottomLeftPlugin,
        &border::RadiusBottomRightPlugin,
        &border::StylePlugin,
        &border::WidthPlugin,
        &border::WidthXPlugin,
        &border::WidthYPlugin,
        &border::WidthTopPlugin,
        &border::WidthBottomPlugin,
        &border::WidthLeftPlugin,
        &border::WidthRightPlugin,
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
        &spacing::PaddingPlugin,
        &spacing::PaddingXPlugin,
        &spacing::PaddingYPlugin,
        &spacing::PaddingLeftPlugin,
        &spacing::PaddingRightPlugin,
        &spacing::PaddingTopPlugin,
        &spacing::PaddingBottomPlugin,
        &spacing::MarginPlugin,
        &spacing::MarginXPlugin,
        &spacing::MarginYPlugin,
        &spacing::MarginLeftPlugin,
        &spacing::MarginRightPlugin,
        &spacing::MarginTopPlugin,
        &spacing::MarginBottomPlugin,
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
        &effect::MixBlendModePlugin,
        &effect::OpacityPlugin,
        &effect::BackgroundBlendModePlugin,
        &effect::BoxShadowPlugin,
        &effect::BoxShadowColorPlugin,
        &transition::DurationPlugin,
        &transition::DelayPlugin,
        &transition::EasePlugin,
        &transition::PropertyPlugin,
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

        // It is better to include the following plugins at the end because they match the "" namespace
        &layout::DisplayPlugin,
        &layout::PositionPlugin,
        &layout::VisibilityPlugin,
        &filter::FilterPlugin,
        &filter::BackdropFilterPlugin,
        &typography::TextTransformPlugin,
        &typography::ItalicPlugin,
        &typography::TextDecorationPlugin,
        &typography::FontVariantNumericPlugin,
        &typography::FontSmoothingPlugin,
        &typography::TextOverflowPlugin,
        &accessibility::ScreenReaderPlugin,
        &transform::TranslateRotateScaleSkewPlugin,
    ];
}
