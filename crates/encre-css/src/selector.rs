use crate::{config::Config, variant::VARIANT_SEPARATOR};

use lazy_static::lazy_static;
use regex::Regex;
use smol_str::SmolStr;

lazy_static! {
    static ref VARIANT_REGEX: Regex =
        Regex::new(&format!(r"^[^\[]*{}", VARIANT_SEPARATOR)).unwrap();
    static ref ARBITRARY_VALUE_REGEX: Regex =
        Regex::new(&format!(r"\[([a-zA-Z0-9-_]+{})?(.+)\]$", VARIANT_SEPARATOR)).unwrap();
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Modifier {
    Basic { is_negative: bool, value: SmolStr },
    Arbitrary { value: SmolStr, hint: SmolStr },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Selector {
    pub(crate) full: SmolStr,
    pub(crate) content: String,
    pub(crate) variants: Option<Vec<String>>,
    pub(crate) is_important: bool,
    pub(crate) is_negative: bool,
}

impl Selector {
    pub fn new<T: Into<SmolStr>>(data: T) -> Self {
        let mut data = data.into();
        let full = data.clone();

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
                VARIANT_SEPARATOR => {
                    if !in_square_bracket {
                        next_variant = true;
                        return;
                    }
                }
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

        Self {
            full,
            variants: if !variants.is_empty() {
                Some(variants)
            } else {
                None
            },
            content,
            is_important,
            is_negative,
        }
    }

    pub fn modifier(&self, config: &Config, namespace: &str) -> Option<Modifier> {
        let modifier_part = self.content.strip_prefix(namespace)?;
        let modifier_part = modifier_part
            .strip_prefix(&**config.modifier_separator)
            .unwrap_or(modifier_part);

        if let Some(caps) = ARBITRARY_VALUE_REGEX.captures(modifier_part) {
            Some(Modifier::Arbitrary {
                hint: SmolStr::from(
                    caps.get(1)
                        .map(|c| c.as_str())
                        .unwrap_or("")
                        .trim_end_matches(':'),
                ),
                value: SmolStr::from(caps.get(2)?.as_str()),
            })
        } else {
            Some(Modifier::Basic {
                is_negative: self.is_negative,
                value: SmolStr::from(modifier_part),
            })
        }
    }
}
