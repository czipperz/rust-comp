use crate::*;
use std::io;

pub enum Error {
    Io(io::Error),
    Lex(lex::Error),
    Parse(parse::Error),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<lex::Error> for Error {
    fn from(e: lex::Error) -> Self {
        Error::Lex(e)
    }
}

impl From<parse::Error> for Error {
    fn from(e: parse::Error) -> Self {
        Error::Parse(e)
    }
}

pub fn run(opt: opt::Opt) -> Result<(), Error> {
    let mut files_contents = Vec::new();
    for i in 0..opt.files.len() {
        let file_name = &opt.files[i];
        let file_contents = read_file::read_file(file_name)?;
        files_contents.push(file_contents);
        let file_contents = &files_contents[i];

        let (tokens, eofpos) = lex::read_tokens(i, &file_contents)?;
        println!("{:?}", tokens);
        let top_levels = parse::parse(&file_contents, &tokens, eofpos)?;
        println!("{:?}", top_levels);
    }
    Ok(())
}
