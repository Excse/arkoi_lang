#[cfg(feature = "serialize")]
use serde::Serialize;

use std::ops::Range;

use crate::file::FileID;

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Clone, Default, Copy, PartialEq)]
pub struct LabelSpan {
    pub span: Span,
    pub file_id: FileID,
}

impl LabelSpan {
    pub fn new(span: impl Into<Span>, file_id: FileID) -> Self {
        LabelSpan {
            span: span.into(),
            file_id,
        }
    }

    pub fn combine(&self, other: &LabelSpan) -> Self {
        let combined = self.span.combine(&other.span);

        LabelSpan::new(combined, self.file_id)
    }
}

#[cfg_attr(feature = "serialize", derive(Serialize))]
#[derive(Debug, Default, Eq, PartialEq, Clone, Copy)]
pub struct Span {
    pub(crate) start: usize,
    pub(crate) end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        assert!(end >= start);

        Span { start, end }
    }

    pub fn single(index: usize) -> Self {
        Span::new(index, index)
    }

    pub fn is_inside(&self, index: usize) -> bool {
        index >= self.start && index <= self.end
    }

    pub fn intersect(&self, other: &Span) -> bool {
        self.end >= other.start && other.end >= self.start
    }

    pub fn combine(&self, other: &Span) -> Self {
        let start = std::cmp::min(self.start, other.start);
        let end = std::cmp::max(self.end, other.end);

        Span::new(start, end)
    }
}

impl From<Range<usize>> for Span {
    fn from(value: Range<usize>) -> Self {
        Span::new(value.start, value.end)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn intersect() {
        let first = Span::new(0, 10);
        let second = Span::new(11, 20);
        assert!(!first.intersect(&second));
        assert!(!second.intersect(&first));

        let first = Span::new(0, 10);
        let second = Span::new(6, 20);
        assert!(first.intersect(&second));
        assert!(second.intersect(&first));
    }

    #[test]
    fn combine() {
        let first = Span::new(0, 10);
        let second = Span::new(6, 20);
        assert_eq!(first.combine(&second), Span::new(0, 20));
        assert_eq!(second.combine(&first), Span::new(0, 20));
    }
}
