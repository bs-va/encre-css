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
    pub fn content(&self) -> &str {
        self.content
    }

    //// Check if the modifier is negative
    pub fn is_negative(&self) -> bool {
        self.is_negative
    }

    //// Check if the modifier content is equals to a string
    pub fn is(&self, val: &str) -> bool {
        self.content == val
    }

    //// Check if the modifier content is equals to at least one value in the given list
    pub fn is_one_of(&self, values: &[&str]) -> bool {
        values.contains(&self.content)
    }

    //// Check if the modifier is empty
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    /// Try to convert the modifier to an `f32`
    pub fn to_f32(&self) -> Result<f32, num::ParseFloatError> {
        if self.is_negative {
            self.content.parse::<f32>().map(|v| -v)
        } else {
            self.content.parse::<f32>()
        }
    }

    /// Try to convert the modifier to a `usize`
    pub fn to_usize(&self) -> Result<usize, num::ParseIntError> {
        self.content.parse::<usize>()
    }

    pub fn strip_prefix(&self, prefix: &str) -> Option<&str> {
        self.content.strip_prefix(prefix)
    }
}

impl<'a> fmt::Display for Modifier<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", if self.is_negative { "-" } else { "" }, self.content)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct Selector {
    full_name: String,
    variants: Vec<String>,
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

        if VARIANT_REGEX.is_match(data) {
            let mut variants = data
                .split(VARIANT_SEPARATOR)
                .map(|v| v.to_string())
                .collect::<Vec<String>>();
            let content = variants.pop().unwrap();

            // Used to be compatible with TailwindCSS
            variants.reverse();

            Self {
                full_name: original_data.to_string(),
                variants,
                content,
                is_negative,
                is_important,
            }
        } else {
            Self {
                full_name: original_data.to_string(),
                variants: vec![],
                content: data.to_string(),
                is_negative,
                is_important,
            }
        }
    }

    /// Check whether an arbitrary string belongs to a namespace
    pub fn check_namespace(&self, maybe_namespace: &str) -> bool {
        self.content.starts_with(maybe_namespace)
    }

    /// Get the modifier of the selector from the namespace of a plugin
    pub fn get_modifier(&self, namespace: &str) -> Modifier {
        if namespace.is_empty() {
            Modifier::new(&self.content, self.is_negative)
        } else {
            let modifier_start_index = if self.content.chars().nth(namespace.len()).map(|v| v == '-') == Some(true) {
                namespace.len() + 1
            } else {
                namespace.len()
            };

            Modifier::new(&self.content[modifier_start_index..], self.is_negative)
        }
    }

    pub fn get_variants(&self) -> &Vec<String> {
        &self.variants
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

    pub fn is_important(&self) -> bool {
        self.is_important
    }

    pub fn contains(&self, other: &Selector) -> bool {
        if self.variants != other.variants {
            return false;
        }

        let (longest, smallest) = if self.content.len() > other.content.len() {
            (&self.content, &other.content)
        } else {
            (&other.content, &self.content)
        };

        longest.contains(smallest)
    }

    pub fn len(&self) -> usize {
        self.content.len()
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }
}

impl fmt::Display for Selector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.full_name)
    }
}
