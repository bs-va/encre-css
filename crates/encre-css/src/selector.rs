use lazy_static::lazy_static;
use regex::Regex;
use std::{fmt, num};

use crate::variant::VARIANT_SEPARATOR;

lazy_static! {
    static ref VARIANT_REGEX: Regex = Regex::new(r"^[^\[]*:").unwrap();
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

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct Selector {
    full_name: String,
    variants: Option<Vec<String>>,
    content: String,
    is_negative: bool,
    is_important: bool,
}

impl Selector {
    pub fn new(original_data: &str) -> Self {
        // Strip the important flag before the negative one
        let (data, is_important) = if let Some(data) = original_data.strip_prefix('!') {
            (data, true)
        } else {
            (original_data, false)
        };

        let (data, is_negative) = if let Some(data) = data.strip_prefix('-') {
            (data, true)
        } else {
            (original_data, false)
        };

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
                full_name: original_data.to_string(),
                variants: Some(variants),
                content,
                is_negative,
                is_important,
            }
        } else {
            Self {
                full_name: original_data.to_string(),
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

    pub fn get_arbitrary_value(&self) -> Option<String> {
        if let Some(opening_index) = self.content.find('[') {
            self.content
                .find(']')
                .map(|closing_index| self.content[opening_index + 1..closing_index].to_string())
        } else {
            None
        }
    }

    /// Check whether an arbitrary string belongs to a namespace
    #[inline]
    pub fn check_namespace(&self, maybe_namespace: &str) -> bool {
        self.content.starts_with(maybe_namespace)
    }

    #[inline]
    pub fn get_variants(&self) -> Option<&Vec<String>> {
        self.variants.as_ref()
    }
    
    #[inline]
    pub fn is_important(&self) -> bool {
        self.is_important
    }

    #[inline]
    pub fn full(&self) -> &str {
        &self.full_name
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.content.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }
}

impl fmt::Display for Selector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.full_name)
    }
}
