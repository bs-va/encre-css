use regex::Regex;
use std::fmt;

use crate::variant::VARIANT_SEPARATOR;

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct Selector {
    variant: Option<String>,
    content: String,                 // FIXME: Remove this pub!!!
    arbitrary_value: Option<String>, // FIXME: Remove this pub!!!
    is_negative: bool,
    is_important: bool,
}

impl Selector {
    pub fn new<T: Into<String>>(data: T) -> Self {
        let data = data.into();

        // Strip the important flag before the negative one
        let (data, is_important) = if let Some(data) = data.strip_prefix('!') {
            (data, true)
        } else {
            (data.as_str(), false)
        };

        let (data, is_negative) = if let Some(data) = data.strip_prefix('-') {
            (data, true)
        } else {
            (data, false)
        };

        let arbitrary_value = {
            if let Some(opening_index) = data.find('[') {
                data.find(']')
                    .map(|closing_index| data[opening_index + 1..closing_index].to_string())
            } else {
                None
            }
        };

        if Regex::new(r"^[^\[]*:").unwrap().is_match(data) {
            let mut split = data.split(VARIANT_SEPARATOR);
            let variant = split.next().unwrap();
            let content = split.next().unwrap();

            Self {
                variant: Some(variant.to_string()),
                content: content.to_string(),
                arbitrary_value,
                is_negative,
                is_important,
            }
        } else {
            Self {
                variant: None,
                content: data.to_string(), // TODO: Prevent
                arbitrary_value,
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

    pub fn get_variant(&self) -> &Option<String> {
        &self.variant
    }

    pub fn get_arbitrary_value(&self) -> &Option<String> {
        &self.arbitrary_value
    }

    pub fn is_important(&self) -> bool {
        self.is_important
    }

    pub fn contains(&self, other: &Selector) -> bool {
        if self.variant != other.variant {
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
        if let Some(ref variant) = self.variant {
            write!(
                f,
                "{}{}{}{}{}",
                if self.is_important { "!" } else { "" },
                if self.is_negative { "-" } else { "" },
                variant,
                VARIANT_SEPARATOR,
                self.content
            )
        } else {
            write!(
                f,
                "{}{}{}",
                if self.is_important { "!" } else { "" },
                if self.is_negative { "-" } else { "" },
                self.content
            )
        }
    }
}
