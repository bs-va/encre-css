use regex::Regex;
use std::fmt;

use crate::prefix::PREFIX_SEPARATOR;

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct Selector {
    pub prefix: Option<String>,
    pub content: String,                 // FIXME: Remove this pub!!!
    pub arbitrary_value: Option<String>, // FIXME: Remove this pub!!!
}

impl Selector {
    pub fn new<T: Into<String>>(data: T) -> Self {
        let data = data.into();
        let arbitrary_value = {
            let modifier = if data.contains('-') {
                let mut iter = data.split('-');
                iter.next();

                if data.starts_with('-') {
                    // Negative value
                    iter.next();
                    Some(format!("-{}", iter.collect::<Vec<&str>>().join("-")))
                } else {
                    // Positive value
                    Some(iter.collect::<Vec<&str>>().join("-"))
                }
            } else {
                // No modifier
                None
            };

            if let Some(ref modifier) = modifier {
                if modifier.contains('[') && modifier.contains(']') {
                    Some(modifier[1..modifier.len() - 1].to_string())
                } else {
                    None
                }
            } else {
                None
            }
        };

        if Regex::new(r"^[^\[]*:").unwrap().is_match(&data) {
            let mut split = data.split(PREFIX_SEPARATOR);
            let prefix = split.next().unwrap();
            let content = split.next().unwrap();

            Self {
                prefix: Some(prefix.to_string()),
                content: content.to_string(),
                arbitrary_value,
            }
        } else {
            Self {
                prefix: None,
                content: data,
                arbitrary_value,
            }
        }
    }

    // TODO: Prevent allocating a String
    pub fn check_namespace(&self, maybe_namespace: &str) -> bool {
        self.content.starts_with(maybe_namespace)
    }

    /// Get the modifier of the selector from the namespace of a plugin
    pub fn get_modifier(&self, namespace: &str) -> String {
        self.content.replace(&format!("{}-", namespace), "")
    }

    /// Return the selector without its prefix
    pub fn without_prefix(mut self) -> Self {
        self.prefix = None;
        self
    }

    pub fn contains(&self, other: &Selector) -> bool {
        if self.prefix != other.prefix {
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
        if let Some(ref prefix) = self.prefix {
            write!(f, "{}{}{}", prefix, PREFIX_SEPARATOR, self.content)
        } else {
            write!(f, "{}", self.content)
        }
    }
}
