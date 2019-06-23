use rust_comp_core::diagnostic::*;
use rust_comp_front::*;
use rust_comp_opt::Opt;
use std::time;

pub enum Error {
    File(String),
    Handled,
}

pub fn run(mut diagnostic: Diagnostic, _opt: Opt) -> Result<(), Error> {
    let mut lines = 0;
    let mut bytes = 0;
    for i in 0..diagnostic.files.len() {
        let file_contents = match read_file::read_file(&diagnostic.files[i]) {
            Ok(file_contents) => file_contents,
            Err(_) => return Err(Error::File(diagnostic.files.into_iter().nth(i).unwrap())),
        };
        diagnostic.add_file_contents(file_contents);
        bytes += diagnostic.files_contents[i].len();
        lines += diagnostic.files_lines[i].len();
    }
    println!("Lines: {}", lines);
    println!("Bytes: {}", bytes);

    let mut lex_total = time::Duration::default();
    let mut parse_total = time::Duration::default();
    let mut parse_to_syntax_total = time::Duration::default();
    for i in 0..diagnostic.files.len() {
        let file_contents = &diagnostic.files_contents[i];

        let start = time::Instant::now();
        let (tokens, eofpos) =
            lex::read_tokens(i, &file_contents).map_err(|e| handle_lex_error(&diagnostic, e))?;
        lex_total += start.elapsed();

        let start = time::Instant::now();
        let top_levels = parse::parse(&file_contents, &tokens, eofpos)
            .map_err(|e| handle_parse_error(&diagnostic, e))?;
        parse_total += start.elapsed();

        let start = time::Instant::now();
        let _top_levels: Vec<_> = top_levels
            .iter()
            .map(parse_to_syntax::convert_top_level)
            .collect();
        parse_to_syntax_total += start.elapsed();
    }

    print_duration("Lex", lex_total);
    print_duration("Parse", parse_total);
    print_duration("Parse to Syntax", parse_to_syntax_total);
    Ok(())
}

fn handle_lex_error(diagnostic: &Diagnostic, e: lex::Error) -> Error {
    match e {
        lex::Error::UnterminatedBlockComment(pos) => {
            diagnostic.print_pos_error(format_args!("unterminated block comment"), pos)
        }
        lex::Error::UnrecognizedControlChar(pos) => {
            diagnostic.print_pos_error(format_args!("unrecognized control character"), pos)
        }
    }
    Error::Handled
}

fn handle_parse_error(diagnostic: &Diagnostic, e: parse::Error) -> Error {
    match e {
        parse::Error::ExpectedToken(token, span) => {
            diagnostic.print_span_error(format_args!("expected {}", token), span)
        }
        parse::Error::Expected(thing, span) => {
            diagnostic.print_span_error(format_args!("expected {}", thing), span)
        }
    }
    Error::Handled
}
