use crate::pos::*;

pub struct TaggedIter<'a> {
    pub contents: &'a str,
    pub pos: Pos,
}

impl<'a> TaggedIter<'a> {
    pub fn new(file: usize, contents: &'a str) -> Self {
        TaggedIter {
            contents,
            pos: Pos { file, index: 0 },
        }
    }

    pub fn peek(&self) -> Option<char> {
        self.contents[self.pos.index..].chars().next()
    }

    pub fn advance(&mut self) {
        if let Some(c) = self.peek() {
            self.pos.increment(c);
        } else {
            panic!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_file_1() {
        let contents = "contents";
        let x = TaggedIter::new(1, &contents);
        assert_eq!(x.contents, &contents[..]);
        assert_eq!(x.pos, Pos { file: 1, index: 0 });
    }

    #[test]
    fn test_peek() {
        let contents = "  ";
        let mut x = TaggedIter::new(0, &contents);
        assert_eq!(x.peek(), Some(' '));
        x.advance();
        assert_eq!(x.peek(), Some(' '));
        x.advance();
        assert_eq!(x.peek(), None);
    }

    #[test]
    fn test_next_greek_letters() {
        let contents = "αβγδ";
        let mut x = TaggedIter::new(0, &contents);
        assert_eq!(x.peek(), Some('α'));
        x.advance();
        assert_eq!(x.peek(), Some('β'));
        x.advance();
        assert_eq!(x.peek(), Some('γ'));
        x.advance();
        assert_eq!(x.peek(), Some('δ'));
        x.advance();
        assert_eq!(x.peek(), None);
    }
}
