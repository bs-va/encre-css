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
        &border::BorderColorPlugin,
        &border::BorderOpacityPlugin,
        &border::DivideColorPlugin,
        &border::DivideOpacityPlugin,
        &border::RingColorPlugin,
        &border::RingOpacityPlugin,
        &border::RingOffsetColorPlugin,
        &border::RingOffsetOpacityPlugin,
        &typography::TypographyColorPlugin,
        &typography::TypographyOpacityPlugin,
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
        &sizing::SizingWidthPlugin,
        &sizing::SizingMinWidthPlugin,
        &sizing::SizingMaxWidthPlugin,
        &sizing::SizingHeightPlugin,
        &sizing::SizingMinHeightPlugin,
        &sizing::SizingMaxHeightPlugin,
    ];
}
