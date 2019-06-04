mod ast;
mod lex;
mod opt;
mod parse;
mod pos;
mod read_file;
mod token;

fn main() {
    let opt = opt::parse();
    for i in 0..opt.files.len() {
        let file_name = &opt.files[i];
        let file_contents = read_file::read_file(file_name).expect("Cannot read input file");
        let (tokens, eofpos) = lex::read_tokens(i, &file_contents).expect("Lexing error");
        println!("{:?}", tokens);
        let top_levels = parse::parse(&file_contents, &tokens, eofpos).expect("Parsing error");
        println!("{:?}", top_levels);
    }
}
