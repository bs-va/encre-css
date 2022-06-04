use lazy_static::lazy_static;
use regex::Regex;
use smol_str::SmolStr;
use std::{fmt, num};

use crate::variant::VARIANT_SEPARATOR;

lazy_static! {
    static ref VARIANT_REGEX: Regex = Regex::new(r"^[^\[]*:").unwrap();
    static ref ARBITRARY_VALUE_REGEX: Regex = Regex::new(r"\[([a-zA-Z0-9-_]+:)?(.+)\]$").unwrap();
}

pub struct Modifier<'a> {
    content: &'a str,
    is_negative: bool,
}

impl<'a> Modifier<'a> {
    pub fn new(content: &'a str, is_negative: bool) -> Self {
        Self {
            content,
            is_negative,
        }
    }

    /// Get the content of the modifier
    #[inline]
    pub fn content(&self) -> &str {
        self.content
    }

    //// Check if the modifier is negative
    #[inline]
    pub fn is_negative(&self) -> bool {
        self.is_negative
    }

    //// Check if the modifier content is equals to a string
    #[inline]
    pub fn is(&self, val: &str) -> bool {
        self.content == val
    }

    //// Check if the modifier content is equals to at least one value in the given list
    #[inline]
    pub fn is_one_of(&self, values: &[&str]) -> bool {
        values.contains(&self.content)
    }

    //// Check if the modifier is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    /// Try to convert the modifier to an `f32`
    #[inline]
    pub fn to_f32(&self) -> Result<f32, num::ParseFloatError> {
        if self.is_negative {
            self.content.parse::<f32>().map(|v| -v)
        } else {
            self.content.parse::<f32>()
        }
    }

    /// Try to convert the modifier to a `usize`
    #[inline]
    pub fn to_usize(&self) -> Result<usize, num::ParseIntError> {
        self.content.parse::<usize>()
    }

    #[inline]
    pub fn strip_prefix(&self, prefix: &str) -> Option<&str> {
        self.content.strip_prefix(prefix)
    }
}

impl<'a> fmt::Display for Modifier<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            if self.is_negative { "-" } else { "" },
            self.content
        )
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Selector {
    pub(crate) full_name: SmolStr,
    pub(crate) variants: Option<Vec<String>>,
    pub(crate) content: String,
    pub(crate) is_negative: bool,
    pub(crate) is_important: bool,
}

impl Selector {
    pub fn new<T: Into<SmolStr>>(data: T) -> Self {
        let mut data = data.into();
        let full_name = data.clone();

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
                VARIANT_SEPARATOR => if !in_square_bracket {
                    next_variant = true;
                    return;
                },
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

        if !variants.is_empty() {
            // Used to be compatible with TailwindCSS
            variants.reverse();

            Self {
                full_name,
                variants: Some(variants),
                content,
                is_negative,
                is_important,
            }
        } else {
            Self {
                full_name,
                variants: None,
                content: data.to_string(),
                is_negative,
                is_important,
            }
        }
    }

    /// Get the modifier of the selector from the namespace of a plugin
    pub fn get_modifier(&self, namespace: &str) -> Modifier {
        if namespace.is_empty() {
            Modifier::new(&self.content, self.is_negative)
        } else {
            let modifier_start_index =
                if self.content.chars().nth(namespace.len()).map(|v| v == '-') == Some(true) {
                    namespace.len() + 1
                } else {
                    namespace.len()
                };

            Modifier::new(&self.content[modifier_start_index..], self.is_negative)
        }
    }

    pub fn get_arbitrary_value(&self) -> Option<(&str, &str)> {
        let caps = ARBITRARY_VALUE_REGEX.captures(&self.content)?;
        Some((caps.get(1).map(|c| c.as_str()).unwrap_or("").trim_end_matches(':'), caps.get(2)?.as_str()))
    }

    /// Check whether an arbitrary string belongs to a namespace
    #[inline]
    pub fn check_namespace(&self, maybe_namespace: &str) -> bool {
        self.content.starts_with(maybe_namespace)
    }
}

impl fmt::Display for Selector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.full_name)
    }
}
