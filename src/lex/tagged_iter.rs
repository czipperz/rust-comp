use crate::pos::*;

#[cfg(test)]
pub fn lines(contents: &str) -> Vec<String> {
    contents.lines().map(|s| s.to_string()).collect::<Vec<_>>()
}

pub struct TaggedIter<'a> {
    pub contents: &'a [String],
    pub pos: Pos,
}

impl<'a> TaggedIter<'a> {
    pub fn new(contents: &'a [String]) -> Self {
        debug_assert!({
            for line in contents {
                assert!(!line.contains('\n'))
            }
            true
        });
        TaggedIter {
            contents,
            pos: Pos::start(),
        }
    }

    pub fn peek(&self) -> Option<char> {
        if self.pos.line == self.contents.len()
            || self.pos.line == self.contents.len() - 1
                && self.pos.column == self.contents[self.pos.line].len()
        {
            None
        } else {
            Some(
                self.contents[self.pos.line][self.pos.column..]
                    .chars()
                    .next()
                    .unwrap_or('\n'),
            )
        }
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
    fn test_new() {
        let contents = lines("contents");
        let x = TaggedIter::new(&contents);
        assert_eq!(x.contents, &contents[..]);
        assert_eq!(x.pos, Pos::start());
    }

    #[test]
    fn test_peek() {
        let contents = lines("  ");
        let mut x = TaggedIter::new(&contents);
        assert_eq!(x.peek(), Some(' '));
        x.next();
        assert_eq!(x.peek(), Some(' '));
        x.next();
        assert_eq!(x.peek(), None);
    }

    #[test]
    fn test_next() {
        let contents = lines("cont");
        let mut x = TaggedIter::new(&contents);
        assert_eq!(x.next(), Some('c'));
        assert_eq!(x.next(), Some('o'));
        assert_eq!(x.next(), Some('n'));
        assert_eq!(x.next(), Some('t'));
        assert_eq!(x.next(), None);
    }

    #[test]
    fn test_next_greek_letters() {
        let contents = lines("    ");
        let mut x = TaggedIter::new(&contents);
        assert_eq!(x.next(), Some(' '));
        assert_eq!(x.next(), Some(' '));
        assert_eq!(x.next(), Some(' '));
        assert_eq!(x.next(), Some(' '));
        assert_eq!(x.next(), None);
    }

    #[test]
    fn test_next_handles_new_lines() {
        let contents = lines("a\nb");
        let mut x = TaggedIter::new(&contents);
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
}
