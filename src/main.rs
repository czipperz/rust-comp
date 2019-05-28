mod lexer;
mod pos;
mod tagged_iter;

use std::io;
use std::fs::File;
fn read_file(name: &str) -> io::Result<String> {
    use io::Read;
    let mut contents = String::new();
    File::open(name)?.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    let file_name = "../system-tests/ex1.rs";
    let file_contents = read_file(file_name).expect("Cannot read input file");
    let tagged_iter = tagged_iter::TaggedIter::new(file_contents, file_name.to_string());
    let tokens = lexer::read_tokens(tagged_iter);
    println!("{:?}", tokens);
}
