use crate::pos::*;

pub struct TaggedIter<'a> {
    contents: &'a str,
    pos: Pos,
    offset: usize,
    next: [Option<char>; 8],
}

impl<'a> TaggedIter<'a> {
    pub fn new(file: usize, contents: &'a str) -> Self {
        let mut next = [None; 8];
        chars(&mut next, contents);
        TaggedIter {
            contents,
            pos: Pos { file, index: 0 },
            offset: 0,
            next,
        }
    }

    pub fn contents(&self) -> &'a str {
        self.contents
    }

    pub fn pos(&self) -> Pos {
        self.pos
    }

    pub fn peek(&self) -> Option<char> {
        self.next[self.offset]
    }

    pub fn peek2(&self) -> Option<char> {
        self.next[self.offset + 1]
    }

    pub fn advance(&mut self) {
        if let Some(c) = self.peek() {
            self.pos.increment(c);

            // Reading from memory is expensive.  We call peek often but rarely
            // call advance.  Thus we prestore the values here.
            if self.offset + 2 >= self.next.len() {
                chars(&mut self.next, &self.contents[self.pos.index..]);
                self.offset = 0;
            } else {
                self.offset += 1;
            }
        } else {
            panic!();
        }
    }
}

fn chars(next: &mut [Option<char>], contents: &str) {
    let mut chars = contents.chars();
    for i in 0..next.len() {
        next[i] = chars.next();
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
