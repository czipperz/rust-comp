mod ast;
mod lex;
mod opt;
mod parse;
mod pos;
mod read_file;
mod run;
mod token;

use run::Error;

fn main() {
    std::process::exit(match run_main() {
        Ok(()) => 0,
        Err(e) => {
            handle_error(e);
            1
        }
    });
}

fn run_main() -> Result<(), Error> {
    let opt = opt::parse();
    run::run(opt)
}

fn handle_error(e: Error) {
    match e {
        Error::Io(e) => eprintln!("{}", e),
        Error::Lex(e) => eprintln!("Lexing error: {}", e),
        Error::Parse(e) => eprintln!("Parsing error: {:?}", e),
    }
}
