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
}

impl<'a> Iterator for TaggedIter<'a> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.pos.increment(c);
        Some(c)
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
        x.next();
        assert_eq!(x.peek(), Some(' '));
        x.next();
        assert_eq!(x.peek(), None);
    }

    #[test]
    fn test_next() {
        let contents = "cont";
        let mut x = TaggedIter::new(0, &contents);
        assert_eq!(x.next(), Some('c'));
        assert_eq!(x.next(), Some('o'));
        assert_eq!(x.next(), Some('n'));
        assert_eq!(x.next(), Some('t'));
        assert_eq!(x.next(), None);
    }

    #[test]
    fn test_next_greek_letters() {
        let contents = "    ";
        let mut x = TaggedIter::new(0, &contents);
        assert_eq!(x.next(), Some(' '));
        assert_eq!(x.next(), Some(' '));
        assert_eq!(x.next(), Some(' '));
        assert_eq!(x.next(), Some(' '));
        assert_eq!(x.next(), None);
    }

    #[test]
    fn test_next_handles_new_lines() {
        let contents = "a\nb";
        let mut x = TaggedIter::new(0, &contents);
        assert_eq!(x.pos.index, 0);

        assert_eq!(x.next(), Some('a'));
        assert_eq!(x.pos.index, 1);

        assert_eq!(x.next(), Some('\n'));
        assert_eq!(x.pos.index, 2);

        assert_eq!(x.next(), Some('b'));
        assert_eq!(x.pos.index, 3);

        assert_eq!(x.next(), None);
        assert_eq!(x.pos.index, 3);
    }
}
