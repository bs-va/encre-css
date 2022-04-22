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
    fn namespace(&self) -> String;

    /// Get the CSS code from a modifier
    ///
    /// If `None` is returned, the result of the plugin will be ignored
    fn get_css_for_modifier(&self, modifier: &str) -> Option<String>; // TODO: Custom type for modifier
                                                                      // TODO: fn custom_css(&self) -> String; (custom CSS added only if plugin used at least once, e.g. for animations)

    /// Returns whether of not the plugin can handle a specific arbitrary value
    ///
    /// Used to distinguish plugins inside the same namespace
    ///
    /// By default, arbitrary values are disallowed
    ///
    /// The `hint` argument can be ignored, for example if the namespace contains a single plugin
    ///
    /// TODO: Remove this function in favor of checking get_css_for_modifier != None
    fn is_matching_value(&self, _hint: &str, _val: &str) -> bool {
        false
    }
    fn css_template_value(&self, _val: &str) -> String {
        String::new()
    }
}

lazy_static! {
    pub static ref PLUGINS: &'static [&'static (dyn Plugin + Sync)] = &[
        &background::BackgroundColorPlugin,
        &background::BackgroundAttachmentPlugin,
        &background::BackgroundClipPlugin,
        &background::BackgroundOpacityPlugin,
        &background::BackgroundImagePlugin,
        &background::BackgroundGradientFromPlugin,
        &background::BackgroundGradientViaPlugin,
        &background::BackgroundGradientToPlugin,
        &background::BackgroundPositionPlugin,
        &background::BackgroundRepeatPlugin,
        &background::BackgroundSizePlugin,
        &border::BorderColorPlugin,
        &border::BorderRadiusPlugin,
        &border::BorderOpacityPlugin,
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
        &sizing::SizingWidthPlugin,
        &sizing::SizingMinWidthPlugin,
        &sizing::SizingMaxWidthPlugin,
        &sizing::SizingHeightPlugin,
        &sizing::SizingMinHeightPlugin,
        &sizing::SizingMaxHeightPlugin,
        &spacing::SpacingPaddingPlugin,
        &spacing::SpacingPaddingXPlugin,
        &spacing::SpacingPaddingYPlugin,
        &spacing::SpacingPaddingLeftPlugin,
        &spacing::SpacingPaddingRightPlugin,
        &spacing::SpacingPaddingTopPlugin,
        &spacing::SpacingPaddingBottomPlugin,
        &spacing::SpacingMarginPlugin,
        &spacing::SpacingMarginXPlugin,
        &spacing::SpacingMarginYPlugin,
        &spacing::SpacingMarginLeftPlugin,
        &spacing::SpacingMarginRightPlugin,
        &spacing::SpacingMarginTopPlugin,
        &spacing::SpacingMarginBottomPlugin,
        &flexbox::FlexboxOrderPlugin,
        &flexbox::FlexboxDirectionPlugin,
        &flexbox::FlexboxWrapPlugin,
        &flexbox::FlexboxGrowShrinkBasisPlugin,
        &layout::LayoutInsetPlugin,
        &layout::LayoutInsetXPlugin,
        &layout::LayoutInsetYPlugin,
        &layout::LayoutTopPlugin,
        &layout::LayoutBottomPlugin,
        &layout::LayoutLeftPlugin,
        &layout::LayoutRightPlugin,
        &layout::LayoutZIndexPlugin,
        &layout::LayoutContainerPlugin,
        &layout::LayoutBoxDecorationBreakPlugin,
        &layout::LayoutBoxSizingPlugin,
        &layout::LayoutFloatPlugin,
        &layout::LayoutClearPlugin,
        &layout::LayoutIsolationPlugin,
        &layout::LayoutObjectFitPlugin,
        &layout::LayoutObjectPositionPlugin,
        &layout::LayoutOverflowPlugin,
        &layout::LayoutOverscrollPlugin,
        &alignment::AlignmentAlignItemsPlugin,
        &alignment::AlignmentJustifyContentPlugin,
        &effect::EffectMixBlendModePlugin,
        &transition::TransitionDurationPlugin,
        &transition::TransitionPropertyPlugin,
        &grid::GridColumnsPlugin,
        &grid::GridRowsPlugin,
        &grid::GridGapPlugin,

        // It is better to include the following plugins at the end because they match the "" namespace
        &layout::LayoutDisplayPlugin,
        &layout::LayoutPositionPlugin,
        &layout::LayoutVisibilityPlugin,
        &filter::FilterPlugin,
        &typography::TypographyTextTransformPlugin,
    ];
}
