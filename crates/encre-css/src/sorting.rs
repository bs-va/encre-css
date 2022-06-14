//! Sort selectors following Tailwind rules
//!
//! See <https://tailwindcss.com/blog/automatic-class-sorting-with-prettier>
use std::cmp::Ordering;

use crate::selector::Selector;

impl PartialOrd for Selector {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(if self.variants.is_none() && other.variants.is_some() {
            Ordering::Less
        } else if self.variants.is_some() && other.variants.is_none() {
            Ordering::Greater
        } else {
            self.full.cmp(&other.full)
        })
    }
}

impl Ord for Selector {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.variants.is_none() && other.variants.is_some() {
            Ordering::Less
        } else if self.variants.is_some() && other.variants.is_none() {
            Ordering::Greater
        } else {
            self.full.cmp(&other.full)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::selector::Selector;

    use std::collections::BTreeSet;

    #[test]
    fn sorting_test() {
        let mut selectors = BTreeSet::new();
        selectors.insert(Selector::new("lg:bg-red-500"));
        selectors.insert(Selector::new("bg-red-500"));

        assert_eq!(
            selectors.iter().collect::<Vec<&Selector>>(),
            vec![
                &Selector::new("bg-red-500"),
                &Selector::new("lg:bg-red-500")
            ]
        );
    }
}
