use std::{fmt, ops::Index};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Span(pub usize, pub usize, pub usize);

impl Span {
    pub fn new(start: usize, offset: usize, line: usize) -> Self {
        Self(start, offset, line)
    }

    pub fn combine(&self, rhs: &Self) -> Self {
        Self(self.0, rhs.1, self.2)
    }
}

impl Index<&Span> for str {
    type Output = str;

    fn index(&self, index: &Span) -> &Self::Output {
        &self[index.0..index.1]
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line: {}", self.2)
    }
}
