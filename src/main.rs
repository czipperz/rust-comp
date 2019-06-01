mod ast;
mod lex;
mod parse;
mod pos;
mod read_file;
mod token;

fn main() {
    let file_name = "../system-tests/ex1.rs";
    let file_contents = read_file::read_file(file_name).expect("Cannot read input file");
    let (tokens, eofpos) = lex::read_tokens(&file_contents).expect("Lexing error");
    println!("{:?}", tokens);
    let top_levels = parse::parse(&file_contents, &tokens, eofpos).expect("Parsing error");
    println!("{:?}", top_levels);
}
