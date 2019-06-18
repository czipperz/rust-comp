use super::parser::Parser;
use super::tree::*;
use super::Error;
use crate::token::*;

pub fn expect_path<'a>(parser: &mut Parser) -> Result<Path, Error> {
    let mut segments = Vec::new();
    let mut separator_spans = Vec::new();
    let prefix_separator = parser.expect_token(TokenKind::ColonColon).ok();
    segments.push(parser.expect_token(TokenKind::Label)?);
    while let Ok(separator) = parser.expect_token(TokenKind::ColonColon) {
        separator_spans.push(separator);
        segments.push(parser.expect_token(TokenKind::Label)?);
    }
    Ok(Path {
        segments,
        prefix_separator,
        separator_spans,
    })
}

#[cfg(test)]
mod tests {
    use super::super::test::parse;
    use super::*;
    use crate::pos::Span;

    #[test]
    fn expect_path_prefix() {
        let (index, len, path) = parse(expect_path, "::a");
        assert_eq!(index, len);
        assert_eq!(
            path,
            Ok(Path {
                segments: vec![Span {
                    file: 0,
                    start: 2,
                    end: 3
                }],
                prefix_separator: Some(Span {
                    file: 0,
                    start: 0,
                    end: 2
                }),
                separator_spans: vec![]
            })
        );
    }

    #[test]
    fn expect_path_no_separators() {
        let (index, len, path) = parse(expect_path, "a");
        assert_eq!(index, len);
        assert_eq!(
            path,
            Ok(Path {
                segments: vec![Span {
                    file: 0,
                    start: 0,
                    end: 1
                }],
                prefix_separator: None,
                separator_spans: vec![]
            })
        );
    }

    #[test]
    fn expect_path_one_separator() {
        let (index, len, path) = parse(expect_path, "a::b");
        assert_eq!(index, len);
        assert_eq!(
            path,
            Ok(Path {
                segments: vec![
                    Span {
                        file: 0,
                        start: 0,
                        end: 1
                    },
                    Span {
                        file: 0,
                        start: 3,
                        end: 4
                    }
                ],
                prefix_separator: None,
                separator_spans: vec![Span {
                    file: 0,
                    start: 1,
                    end: 3
                }]
            })
        );
    }
}
