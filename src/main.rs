mod ast;
mod lex;
mod parse;
mod pos;
mod token;

use std::fs::File;
use std::io;
fn read_file(name: &str) -> io::Result<String> {
    use io::Read;
    let mut contents = String::new();
    File::open(name)?.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    let file_name = "../system-tests/ex1.rs";
    let file_contents = read_file(file_name).expect("Cannot read input file");
    let (tokens, eofpos) = lex::read_tokens(&file_contents).expect("Lexing error");
    println!("{:?}", tokens);
    let top_levels = parse::parse(&file_contents, &tokens, eofpos).expect("Parsing error");
    println!("{:?}", top_levels);
}
