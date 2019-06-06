use crate::pos::*;

pub struct Diagnostic {
    pub files: Vec<String>,
    pub files_contents: Vec<String>,
    pub files_lines: Vec<Vec<usize>>,
}

impl Diagnostic {
    pub fn new(files: Vec<String>) -> Self {
        Diagnostic {
            files,
            files_contents: Vec::new(),
            files_lines: Vec::new(),
        }
    }

    pub fn handle_pos_error(&self, format_args: std::fmt::Arguments, pos: Pos) {
        let file_lines = &self.files_lines[pos.file];
        let line = match file_lines.binary_search(&pos.index) {
            Ok(line) => line,
            Err(line) => line - 1,
        };
        let line_start = file_lines[line];
        let line_end = file_lines[line + 1];
        let column = pos.index - line_start;

        eprintln!("{}:{}:{}:", self.files[pos.file], line + 1, column + 1);
        eprintln!("  >> Error: {}", format_args);
        eprintln!();
        let file_contents = &self.files_contents[pos.file];
        // TODO: replace with log10
        let line_string = format!("{}", line + 1);
        let line_len = line_string.len();
        eprintln!("{} | {}", line_string, &file_contents[line_start..line_end]);

        for _ in 0..column + line_len + 3 {
            eprint!(" ");
        }
        eprintln!("^ here");
        eprintln!();
    }

    pub fn handle_span_error(&self, format_args: std::fmt::Arguments, span: Span) {
        self.handle_pos_error(
            format_args,
            Pos {
                file: span.file,
                index: span.start,
            },
        )
    }
}
