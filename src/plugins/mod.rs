use lazy_static::lazy_static;
use std::fmt;

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
    /// NOTE: This function is called after [to_css_value], so, `_` (underscores) are already converted to ` ` (spaces)
    ///
    /// [to_css_value]: crate::to_css_value
    fn css_template_value(&self, _val: &str, _css_content: &mut String) -> fmt::Result {
        Ok(())
    }

    /// Get the CSS code from a modifier
    ///
    /// If nothing is written to the `result` buffer, the result of the plugin will be ignored
    fn get_css_for_modifier(&self, modifier: &str, css_content: &mut String) -> fmt::Result; // TODO: Custom type for modifier

    // TODO: fn custom_css(&self) -> String; (custom CSS added only if plugin used at least once, e.g. for animations)
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
        &border::WidthPlugin,
        &border::WidthXPlugin,
        &border::WidthYPlugin,
        &border::WidthTopPlugin,
        &border::WidthBottomPlugin,
        &border::WidthLeftPlugin,
        &border::WidthRightPlugin,
        &border::OpacityPlugin,
        &border::DivideColorPlugin,
        &border::DivideOpacityPlugin,
        &border::RingColorPlugin,
        &border::RingOpacityPlugin,
        &border::RingOffsetColorPlugin,
        &border::RingOffsetOpacityPlugin,
        &typography::TypographyColorPlugin,
        &typography::TypographyOpacityPlugin,
        &typography::TypographyFontFamilyPlugin,
        &typography::TypographyFontSizePlugin,
        &typography::TypographyFontWeightPlugin,
        &typography::TypographyTextAlignmentPlugin,
        &typography::TypographyTrackingPlugin,
        &typography::TypographyLeadingPlugin,
        &typography::TypographyTextDecorationColorPlugin,
        &typography::TypographyTextDecorationStylePlugin,
        &typography::TypographyTextDecorationThicknessPlugin,
        &typography::TypographyTextDecorationOffsetPlugin,
        &typography::TypographyContentPlugin,
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
        &alignment::AlignItemsPlugin,
        &alignment::JustifyContentPlugin,
        &effect::MixBlendModePlugin,
        &transition::DurationPlugin,
        &transition::PropertyPlugin,
        &grid::ColumnsPlugin,
        &grid::RowsPlugin,
        &grid::GapPlugin,
        &interactivity::CursorPlugin,
        &effect::BoxShadowPlugin,
        &effect::BoxShadowColorPlugin,
        &svg::FillPlugin,
        &svg::StrokeColorPlugin,
        &svg::StrokeWidthPlugin,
        &table::BorderCollapsePlugin,
        &table::TableLayoutPlugin,

        // It is better to include the following plugins at the end because they match the "" namespace
        &layout::DisplayPlugin,
        &layout::PositionPlugin,
        &layout::VisibilityPlugin,
        &filter::FilterPlugin,
        &filter::BackdropFilterPlugin,
        &typography::TypographyTextTransformPlugin,
        &typography::TypographyItalicPlugin,
        &typography::TypographyTextDecorationPlugin,
    ];
}
