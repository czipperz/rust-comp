use crate::diagnostic::*;
use crate::*;
use std::io;

pub enum Error {
    Io(io::Error),
    Handled,
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<()> for Error {
    fn from(_: ()) -> Self {
        Error::Handled
    }
}

pub fn run(mut diagnostic: Diagnostic, _opt: opt::Opt) -> Result<(), Error> {
    let start = std::time::Instant::now();
    let mut lines = 0;
    let mut bytes = 0;
    for i in 0..diagnostic.files.len() {
        let file_name = &diagnostic.files[i];
        let file_contents = read_file::read_file(file_name)?;
        diagnostic.files_contents.push(file_contents);
        let file_contents = &diagnostic.files_contents[i];
        diagnostic.files_lines.push(file_lines(&file_contents));

        lines += diagnostic.files_lines[i].len();
        bytes = file_contents.len();
    }
    println!("Lines: {}", lines);
    println!("Bytes: {}", bytes);
    print_duration("File", start.elapsed());

    let start = std::time::Instant::now();
    for i in 0..diagnostic.files.len() {
        let file_contents = &diagnostic.files_contents[i];
        let (tokens, eofpos) =
            lex::read_tokens(i, &file_contents).map_err(|e| handle_lex_error(&diagnostic, e))?;
        //println!("{:?}", tokens);

        let top_levels = parse::parse(&file_contents, &tokens, eofpos)
            .map_err(|e| handle_parse_error(&diagnostic, e))?;
        //println!("{:?}", top_levels);
    }
    print_duration("LexParse", start.elapsed());
    Ok(())
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

fn handle_lex_error(diagnostic: &Diagnostic, e: lex::Error) {
    match e {
        lex::Error::UnterminatedBlockComment(pos) => {
            diagnostic.handle_pos_error(format_args!("unterminated block comment"), pos)
        }
    }
}

fn handle_parse_error(diagnostic: &Diagnostic, e: parse::Error) {
    match e {
        parse::Error::ExpectedToken(token, span) => {
            diagnostic.handle_span_error(format_args!("expected {:?}", token), span)
        }
        parse::Error::Expected(thing, span) => {
            diagnostic.handle_span_error(format_args!("expected {}", thing), span)
        }
    }
}
