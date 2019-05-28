use std::ops::{Deref, DerefMut};

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
    pub line: usize,
    pub column: usize,
    pub index: usize,
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
    pub fn start() -> Self {
        Pos {
            line: 0,
            column: 0,
            index: 0,
        }
    }

    pub fn increment(&mut self, c: char) {
        if c == '\n' {
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }
        self.index += c.len_utf8();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pos_start() {
        let pos = Pos::start();
        assert_eq!(pos.line, 0);
        assert_eq!(pos.column, 0);
        assert_eq!(pos.index, 0);
    }

    #[test]
    fn test_increment_same_line() {
        let mut pos = Pos::start();

        pos.increment('a');
        assert_eq!(pos.line, 0);
        assert_eq!(pos.column, 1);

        pos.increment('b');
        assert_eq!(pos.line, 0);
        assert_eq!(pos.column, 2);
    }

    #[test]
    fn test_increment_new_line() {
        let mut pos = Pos::start();

        pos.increment('a');
        pos.increment('\n');
        assert_eq!(pos.line, 1);
        assert_eq!(pos.column, 0);
    }

    #[test]
    fn test_increment_index() {
        let mut pos = Pos::start();

        pos.increment('μ');
        assert_eq!(pos.index, 2);

        pos.increment('m');
        assert_eq!(pos.index, 3);
    }
}
