use std::ops::{Deref, DerefMut, Index};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Span {
    pub start: Pos,
    pub end: Pos,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FilePos<'a> {
    pub file_name: &'a str,
    pub pos: Pos,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Pos {
    pub file: usize,
    pub index: usize,
}

impl Span {
    pub fn range(start: Pos, s: &str) -> Self {
        let mut end = start;
        for c in s.chars() {
            end.increment(c)
        }
        Span { start, end }
    }
}

impl Index<Span> for str {
    type Output = str;

    fn index(&self, span: Span) -> &str {
        debug_assert!(span.start.file == span.end.file);
        &self[span.start.index..span.end.index]
    }
}

impl<'a> Deref for FilePos<'a> {
    type Target = Pos;

    fn deref(&self) -> &Pos {
        &self.pos
    }
}

impl<'a> DerefMut for FilePos<'a> {
    fn deref_mut(&mut self) -> &mut Pos {
        &mut self.pos
    }
}

impl Pos {
    pub fn increment(&mut self, c: char) {
        self.index += c.len_utf8();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range() {
        let start = Pos { file: 0, index: 3 };
        assert_eq!(
            Span::range(start, "abc\ndef"),
            Span {
                start,
                end: Pos { file: 0, index: 10 }
            }
        );
    }

    #[test]
    fn test_increment_same_line() {
        let mut pos = Pos { file: 0, index: 0 };

        pos.increment('a');
        assert_eq!(pos.index, 1);

        pos.increment('b');
        assert_eq!(pos.index, 2);
    }

    #[test]
    fn test_increment_new_line() {
        let mut pos = Pos { file: 0, index: 0 };

        pos.increment('a');
        pos.increment('\n');
        assert_eq!(pos.index, 2);
    }

    #[test]
    fn test_increment_index() {
        let mut pos = Pos { file: 0, index: 0 };

        // It appears that rustfmt will change this to ' ' instead of
        // 'greekletter'.  This causes this test to fail instead of pass.
        pos.increment('μ');
        assert_eq!(pos.index, 2);

        pos.increment('m');
        assert_eq!(pos.index, 3);
    }
}
