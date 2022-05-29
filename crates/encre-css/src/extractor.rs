use crate::selector::Selector;

use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::BTreeSet, fs, io::Read, iter, path::Path};
use wax::Glob;

lazy_static! {
    static ref SPLIT_REGEX: Regex = Regex::new(r#"(?-u)[\s'"`;>=]+"#).unwrap();
}

pub struct Extractor;

impl Extractor {
    /// Scan the contents of a file and store all the selectors found
    pub fn scan_raw(content: &str) -> BTreeSet<Selector> {
        SPLIT_REGEX
            .split(content)
            .filter_map(|val| {
                // The shortest selector is `m-1`
                if val.len() >= 3 {
                    Some(Selector::new(val))
                } else {
                    None
                }
            })
            .collect::<BTreeSet<Selector>>()
    }

    /// Scan all files given and store all the selectors found
    pub fn scan_files<T: AsRef<Path>>(
        files: impl Iterator<Item = T>,
    ) -> BTreeSet<Selector> {
        // TODO: Error handling
        let mut file_contents: String = String::new();

        files
            .filter_map(|file_path| {
                let mut file = match fs::File::open(&file_path) {
                    Ok(f) => f,
                    Err(e) => panic!("Failed to read the file {:?}: {:?}", file_path.as_ref(), e),
                };
                file_contents.clear();

                if file.read_to_string(&mut file_contents).is_ok() {
                    Some(Self::scan_raw(&file_contents))
                } else {
                    // TODO: Display a warning otherwise
                    None
                }
            })
            .reduce(|mut selectors1, selectors2| {
                selectors1.extend(selectors2);
                selectors1
            }).unwrap_or_default()
    }

    /// Scan all files in a path using the glob syntax
    pub fn scan_path<T: AsRef<Path>>(glob_path: T) -> BTreeSet<Selector> {
        let (prefix, glob) = match Glob::new(
            glob_path
                .as_ref()
                .to_str()
                .expect("failed to convert the glob to a string"),
        ) {
            Ok(g) => g.partition(),
            Err(e) => panic!("{}", e),
        };

        if prefix == glob_path.as_ref() {
            Self::scan_files(iter::once(glob_path))
        } else {
            Self::scan_files(glob.walk(prefix).map(|e| e.unwrap().into_path()))
        }
    }
}
