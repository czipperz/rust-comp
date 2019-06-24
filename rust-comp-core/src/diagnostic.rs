use crate::pos::*;
use std::fmt;
use std::io;

pub struct Diagnostic {
    pub files_names: Vec<String>,
    pub files_contents: Vec<String>,
    pub files_lines: Vec<Vec<usize>>,
}

impl Diagnostic {
    pub fn new(files_names: Vec<String>) -> Self {
        Diagnostic {
            files_names,
            files_contents: Vec::new(),
            files_lines: Vec::new(),
        }
    }

    pub fn print_pos_error(&self, format_args: fmt::Arguments, pos: Pos) {
        self.print_span_error(format_args, span_for_pos(pos));
    }

    pub fn print_span_error(&self, format_args: std::fmt::Arguments, span: Span) {
        // At this point we are already going to quit.  If writing to stderr
        // errors, we don't want to write anything else to it.  Thus we'll just
        // ignore it.
        let _ = self.write_span_error(io::stderr().lock(), format_args, span);
    }

    fn write_span_error(
        &self,
        mut stream: impl io::Write,
        format_args: fmt::Arguments,
        span: Span,
    ) -> io::Result<()> {
        let file_lines = &self.files_lines[span.file];
        let line = match file_lines.binary_search(&span.start) {
            Ok(line) => line,
            Err(line) => line - 1,
        };
        let line_start = file_lines[line];
        let line_end = file_lines[line + 1];
        let column = span.start - line_start;

        writeln!(
            stream,
            "{}:{}:{}:",
            self.files_names[span.file],
            line + 1,
            column + 1
        )?;
        writeln!(stream, "  >> Error: {}", format_args)?;
        writeln!(stream)?;

        let file_contents = &self.files_contents[span.file];
        // TODO: replace with log10
        let line_string = format!("{}", line + 1);
        let line_len = line_string.len();
        writeln!(
            stream,
            "{} | {}",
            line_string,
            &file_contents[line_start..line_end]
        )?;

        for _ in 0..column + line_len + 3 {
            write!(stream, " ")?;
        }
        for _ in span.start..span.end {
            write!(stream, "^")?;
        }
        writeln!(stream, " here")?;
        writeln!(stream)?;
        Ok(())
    }

    pub fn add_file_contents(&mut self, contents: String) {
        self.files_lines.push(file_lines(&contents));
        self.files_contents.push(contents);
    }
}

fn span_for_pos(pos: Pos) -> Span {
    Span {
        file: pos.file,
        start: pos.index,
        end: pos.index + 1,
    }
}

fn file_lines(s: &str) -> Vec<usize> {
    let mut result = Vec::new();
    result.push(0);
    for (i, c) in s.chars().enumerate() {
        if c == '\n' {
            result.push(i + 1);
        }
    }
    if result[result.len() - 1] != s.len() {
        result.push(s.len());
    }
    result
}

pub fn print_duration(name: &str, duration: std::time::Duration) {
    println!(
        "{}: {}.{:06}",
        name,
        duration.as_secs(),
        duration.subsec_micros()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let diagnostic = Diagnostic::new(vec!["abc".to_string()]);
        assert_eq!(diagnostic.files_names, ["abc".to_string()]);
        assert_eq!(diagnostic.files_contents.len(), 0);
        assert_eq!(diagnostic.files_lines.len(), 0);
    }

    #[test]
    fn test_add_file_contents() {
        let mut diagnostic = Diagnostic::new(vec!["file1".to_string()]);
        diagnostic.add_file_contents("oh man xx\nare they\nready for a miracle?".to_string());
        assert_eq!(
            diagnostic.files_contents,
            ["oh man xx\nare they\nready for a miracle?".to_string()]
        );
        assert_eq!(diagnostic.files_lines, [vec![0, 10, 19, 39]]);
    }

    #[test]
    fn test_file_lines_empty_file() {
        assert_eq!(file_lines(""), vec![0]);
    }

    #[test]
    fn test_file_lines_no_trailing_newline() {
        assert_eq!(file_lines("abc\ndef"), vec![0, 4, 7]);
    }

    #[test]
    fn test_file_lines_trailing_newline() {
        assert_eq!(file_lines("abc\ndef\n"), vec![0, 4, 8]);
    }

    #[test]
    fn test_write_pos_error_emulated() {
        let mut diagnostic = Diagnostic::new(vec!["file1".to_string()]);
        diagnostic.add_file_contents("oh man xx\nare they\nready for a miracle?".to_string());

        let mut buffer = Vec::new();
        diagnostic
            .write_span_error(
                &mut buffer,
                format_args!("error message"),
                span_for_pos(Pos { file: 0, index: 3 }),
            )
            .unwrap();

        let string = String::from_utf8(buffer).unwrap();
        assert_eq!(
            string,
            "file1:1:4:
  >> Error: error message

1 | oh man xx

       ^ here

"
        );
    }

    #[test]
    fn test_write_span_error() {
        let mut diagnostic = Diagnostic::new(vec!["file1".to_string()]);
        diagnostic.add_file_contents("oh man xx\nare they\nready for a miracle?".to_string());

        let mut buffer = Vec::new();
        diagnostic
            .write_span_error(
                &mut buffer,
                format_args!("error message"),
                Span {
                    file: 0,
                    start: 3,
                    end: 6,
                },
            )
            .unwrap();

        let string = String::from_utf8(buffer).unwrap();
        assert_eq!(
            string,
            "file1:1:4:
  >> Error: error message

1 | oh man xx

       ^^^ here

"
        );
    }
}
