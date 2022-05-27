use crate::selector::Selector;

use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::BTreeSet, fs, io::Read, iter, path::PathBuf};
use wax::Glob;

lazy_static! {
    static ref SPLIT_REGEX: Regex = Regex::new(r#"(?-u)[\s'"`;>=]+"#).unwrap();
}

pub struct Extractor {
    pub(crate) scanned_selectors_without_variant: BTreeSet<Selector>,
    pub(crate) scanned_selectors_with_variant: BTreeSet<Selector>,
}

impl Extractor {
    pub fn new() -> Self {
        Self {
            scanned_selectors_without_variant: BTreeSet::new(),
            scanned_selectors_with_variant: BTreeSet::new(),
        }
    }

    /// Add a new selector which will have its CSS generated
    ///
    /// This function automatically handles duplicated selectors
    pub fn add_selector(&mut self, val: &str) {
        let selector = Selector::new(val);

        if !selector.get_variants().is_empty() {
            self.scanned_selectors_with_variant.insert(selector);
        } else {
            self.scanned_selectors_without_variant.insert(selector);
        }
    }

    /// Scan the contents of a file and store all the selectors found
    pub fn scan_raw(&mut self, content: &str) {
        for val in SPLIT_REGEX.split(content) {
            self.add_selector(val);
        }
    }

    /// Scan all files given and store all the selectors found
    pub fn scan_files(&mut self, files: impl Iterator<Item = PathBuf>) {
        let mut file_contents: String = String::new();

        for file in files {
            let mut file = fs::File::open(file).unwrap();
            file_contents.clear();

            if file.read_to_string(&mut file_contents).is_ok() {
                self.scan_raw(&file_contents);
            }
            // TODO: Display a warning otherwise
        }
    }

    /// Scan all files in a path using the glob syntax
    pub fn scan_path(&mut self, glob_path: &PathBuf) {
        let (prefix, glob) = Glob::partitioned(
            glob_path
                .to_str()
                .expect("failed to convert the glob to a PathBuf"),
        )
        .unwrap();

        if prefix == *glob_path {
            self.scan_files(iter::once(glob_path.clone()));
        } else {
            self.scan_files(
                glob.walk(prefix, usize::MAX)
                    .map(|e| e.unwrap().into_path()),
            );
        }
    }
}

impl Default for Extractor {
    fn default() -> Self {
        Self::new()
    }
}
