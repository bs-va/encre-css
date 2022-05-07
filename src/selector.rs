use lazy_static::lazy_static;
use regex::Regex;
use std::fmt;

use crate::variant::VARIANT_SEPARATOR;

lazy_static! {
    static ref VARIANT_REGEX: Regex = Regex::new(r"^[^\[]*:").unwrap();
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
    pub fn new(data: &str) -> Self {
        // Strip the important flag before the negative one
        let (data, is_important) = if let Some(data) = data.strip_prefix('!') {
            (data, true)
        } else {
            (data, false)
        };

        let (data, is_negative) = if let Some(data) = data.strip_prefix('-') {
            (data, true)
        } else {
            (data, false)
        };

        if VARIANT_REGEX.is_match(data) {
            // TODO: Escape `:` inside `[]`
            let mut variants = data
                .split(VARIANT_SEPARATOR)
                .map(|v| v.to_string())
                .collect::<Vec<String>>();
            let content = variants.pop().unwrap();

            // Used to be compatible with TailwindCSS
            variants.reverse();

            Self {
                full_name: data.to_string(),
                variants,
                content,
                is_negative,
                is_important,
            }
        } else {
            Self {
                full_name: data.to_string(),
                variants: vec![],
                content: data.to_string(), // TODO: Prevent
                is_negative,
                is_important,
            }
        }
    }

    pub fn check_namespace(&self, maybe_namespace: &str) -> bool {
        self.content.starts_with(maybe_namespace)
    }

    /// Get the modifier of the selector from the namespace of a plugin
    pub fn get_modifier(&self, namespace: &str) -> String {
        if namespace.is_empty() {
            format!(
                "{}{}",
                if self.is_negative { "-" } else { "" },
                self.content
            )
        } else {
            format!(
                "{}{}",
                if self.is_negative { "-" } else { "" },
                self.content.replace(
                    &format!(
                        "{}{}",
                        namespace,
                        if self.content.contains('-') { "-" } else { "" }
                    ),
                    ""
                )
            )
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
