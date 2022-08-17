//! Spacing utility functions.
use std::borrow::Cow;

/// Returns whether the modifier is matching a builtin spacing value. The builtin spacing values are:
///
/// - `px`;
/// - Any float ([`f32`]);
/// - Any fraction (e.g `9/12`) consisting of a [`usize`], a slash and another [`usize`].
pub fn is_matching_builtin_spacing(value: &str) -> bool {
    value == "px"
        || value.parse::<f32>().is_ok()
        || value.split_once('/').map_or(false, |(a, b)| {
            a.parse::<usize>().is_ok() && b.parse::<usize>().is_ok()
        })
}

/// Get a spacing value from a modifier.
///
/// Spacing values don't follow Tailwind's philosophy of limiting possible values and are closer
/// to [Windi CSS](https://windicss.org/features/value-auto-infer.html#numbers). They are
/// however perfectly compatible with Tailwind's values.
pub fn get(value: &str, is_negative: bool) -> Option<Cow<str>> {
    if value == "px" {
        if is_negative {
            Some(Cow::from("-1px"))
        } else {
            Some(Cow::from("1px"))
        }
    } else if value == "0" {
        Some(Cow::from("0px"))
    } else if let Some((a, b)) = value.split_once('/') {
        // Fractions
        let a = a.parse::<usize>().ok()?;
        let b = b.parse::<usize>().ok()?;

        #[allow(clippy::cast_precision_loss)]
        if is_negative {
            Some(Cow::from(format!("{}%", 100. * (-(a as f32) / b as f32))))
        } else {
            Some(Cow::from(format!("{}%", 100. * (a as f32 / b as f32))))
        }
    } else {
        // Floats
        let value = if is_negative {
            -value.parse::<f32>().ok()? / 4.
        } else {
            value.parse::<f32>().ok()? / 4.
        };

        Some(Cow::from(format!("{}rem", value)))
    }
}
