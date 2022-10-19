//! Defines the buffer containing the CSS generated.
use std::{
    fmt::{self, Write},
    ops::{Deref, RangeBounds},
};

/// A buffer storing the CSS generated and managing the indentation of the next lines.
#[derive(Debug)]
pub struct Buffer {
    inner: String,
    indentation: String,
}

impl Buffer {
    /// Create a new `Buffer` with a minimal capacity.
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: String::with_capacity(capacity),
            indentation: String::new(),
        }
    }

    /// Returns the length of the buffer.
    pub(crate) fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns whether the next line written will have at least one level of indentation.
    pub(crate) fn replace_range<R: RangeBounds<usize>>(&mut self, range: R, replace_with: &str) {
        self.inner.replace_range(range, replace_with);
    }

    /// Returns whether the next line written will have at least one level of indentation.
    pub fn is_unindented(&mut self) -> bool {
        self.indentation.is_empty()
    }

    /// Indent the next lines using two spaces.
    pub fn indent(&mut self) {
        let _ = write!(self.indentation, "  ");
    }

    /// Un-indent the next lines by two spaces.
    pub fn unindent(&mut self) {
        self.indentation.truncate(self.indentation.len() - 2);
    }

    /// Push a new line which will be indented, in the buffer.
    pub fn line<T: fmt::Display>(&mut self, val: T) {
        writeln!(self.inner, "{}{}", self.indentation, val)
            .expect("writing to a String can't fail");
    }

    /// Push a list of lines which will be indented, in the buffer.
    pub fn lines<T: fmt::Display>(&mut self, lines: impl IntoIterator<Item = T>) {
        lines.into_iter().for_each(|l| {
            writeln!(self.inner, "{}{}", self.indentation, l)
                .expect("writing to a String can't fail");
        });
    }

    /// Push a raw string in the buffer.
    pub fn raw(&mut self, raw: &str) {
        self.inner.push_str(raw);
    }

    /// Returns the underlying buffer.
    pub fn into_inner(self) -> String {
        self.inner
    }
}

impl Deref for Buffer {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
