mod arena;
mod ast;
mod diagnostic;
mod lex;
mod opt;
mod parse;
mod pos;
mod read_file;
mod run;
mod token;

use run::Error;
use std::time;

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
    let start = time::Instant::now();

    let args = opt::parse();
    diagnostic::print_duration("Arguments", start.elapsed());

    run::run(diagnostic::Diagnostic::new(args.files), args.opt)?;

    diagnostic::print_duration("Total", start.elapsed());
    Ok(())
}

fn handle_error(e: Error) {
    match e {
        Error::Io(e) => eprintln!("{}", e),
        Error::Handled => (),
    }
}
