use std::collections::BTreeSet;

pub struct Extractor {
    extract_fn: Box<dyn Fn(&str) -> BTreeSet<&str>>,
}

impl Extractor {
    pub fn from_fn<T: 'static + Fn(&str) -> BTreeSet<&str>>(extract_fn: T) -> Self {
        Self {
            extract_fn: Box::new(extract_fn),
        }
    }

    pub(crate) fn extract<'a>(&self, val: &'a str) -> BTreeSet<&'a str> {
        (self.extract_fn)(val)
    }
}

impl Default for Extractor {
    fn default() -> Self {
        Self {
            extract_fn: Box::new(|val| {
                val.split(|ch| ch == ' ' || ch == '"' || ch == '\'' || ch == '`')
                    .collect::<BTreeSet<&str>>()
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::BTreeSet;

    #[test]
    fn default_extractor_test() {
        assert_eq!(
            Extractor::default().extract("test bg-red-500 'hello'"),
            BTreeSet::from(["", "test", "bg-red-500", "hello"])
        );
    }

    #[test]
    fn custom_extractor_test() {
        let extractor =
            Extractor::from_fn(|val| val.split(|ch| ch == '|').collect::<BTreeSet<&str>>());

        assert_eq!(
            extractor.extract("test|bg-red-500|'hello'"),
            BTreeSet::from(["test", "bg-red-500", "'hello'"])
        );
    }
}
