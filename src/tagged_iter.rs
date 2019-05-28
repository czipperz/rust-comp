use crate::pos::*;

pub struct TaggedIter {
    contents: String,
    pos: Pos,
}

impl TaggedIter {
    pub fn new(contents: String) -> Self {
        TaggedIter {
            contents,
            pos: Pos::start(),
        }
    }

    pub fn pos(&self) -> Pos {
        self.pos
    }
}

impl Iterator for TaggedIter {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        let c = self.contents[self.pos.index..].chars().next()?;
        self.pos.increment(c);
        Some(c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let x = TaggedIter::new("contents".to_string());
        assert_eq!(x.contents, "contents");
        assert_eq!(x.pos, Pos::start());
    }

    #[test]
    fn test_next() {
        let mut x = TaggedIter::new("cont".to_string());
        assert_eq!(x.next(), Some('c'));
        assert_eq!(x.next(), Some('o'));
        assert_eq!(x.next(), Some('n'));
        assert_eq!(x.next(), Some('t'));
        assert_eq!(x.next(), None);
    }

    #[test]
    fn test_next_greek_letters() {
        let mut x = TaggedIter::new("    ".to_string());
        assert_eq!(x.next(), Some(' '));
        assert_eq!(x.next(), Some(' '));
        assert_eq!(x.next(), Some(' '));
        assert_eq!(x.next(), Some(' '));
        assert_eq!(x.next(), None);
    }

    #[test]
    fn test_next_handles_new_lines() {
        let mut x = TaggedIter::new("a\nb".to_string());
        assert_eq!(x.pos.line, 0);
        assert_eq!(x.pos.column, 0);

        assert_eq!(x.next(), Some('a'));
        assert_eq!(x.pos.line, 0);
        assert_eq!(x.pos.column, 1);

        assert_eq!(x.next(), Some('\n'));
        assert_eq!(x.pos.line, 1);
        assert_eq!(x.pos.column, 0);

        assert_eq!(x.next(), Some('b'));
        assert_eq!(x.pos.line, 1);
        assert_eq!(x.pos.column, 1);

        assert_eq!(x.next(), None);
        assert_eq!(x.pos.line, 1);
        assert_eq!(x.pos.column, 1);
    }

    #[test]
    fn test_pos() {
        let x = TaggedIter::new("a\nb".to_string());
        assert_eq!(x.pos(), x.pos);
    }
}
