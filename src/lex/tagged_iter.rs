use crate::pos::*;

pub struct TaggedIter<'a> {
    contents: &'a str,
    pos: Pos,
    next: Option<char>,
    next2: Option<char>,
}

impl<'a> TaggedIter<'a> {
    pub fn new(file: usize, contents: &'a str) -> Self {
        let mut chars = contents.chars();
        TaggedIter {
            contents,
            pos: Pos { file, index: 0 },
            next: chars.next(),
            next2: chars.next(),
        }
    }

    pub fn contents(&self) -> &'a str {
        self.contents
    }

    pub fn pos(&self) -> Pos {
        self.pos
    }

    pub fn peek(&self) -> Option<char> {
        self.next
    }

    pub fn peek2(&self) -> Option<char> {
        self.next2
    }

    pub fn advance(&mut self) {
        if let Some(c) = self.peek() {
            self.pos.increment(c);

            // Reading from memory is expensive.  We call peek often but rarely
            // call advance.  Thus we prestore the values here.
            let mut chars = self.contents[self.pos.index..].chars();
            self.next = chars.next();
            self.next2 = chars.next();
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
        let x = TaggedIter::new(1, contents);
        assert_eq!(x.contents, contents);
        assert_eq!(x.pos, Pos { file: 1, index: 0 });
    }

    #[test]
    fn test_peek() {
        let contents = "  ";
        let mut x = TaggedIter::new(0, contents);
        assert_eq!(x.peek(), Some(' '));
        x.advance();
        assert_eq!(x.peek(), Some(' '));
        x.advance();
        assert_eq!(x.peek(), None);
    }

    #[test]
    fn test_peek2() {
        let contents = " μa";
        let mut x = TaggedIter::new(0, contents);
        assert_eq!(x.peek(), Some(' '));
        assert_eq!(x.peek2(), Some('μ'));
        x.advance();
        assert_eq!(x.peek2(), Some('a'));
        x.advance();
        assert_eq!(x.peek2(), None);
        x.advance();
        assert_eq!(x.peek2(), None);
    }

    #[test]
    fn test_next_greek_letters() {
        let contents = "αβγδ";
        let mut x = TaggedIter::new(0, contents);
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
