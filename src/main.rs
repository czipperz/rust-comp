mod diagnostic;
mod run;

use crate::run::Error;
use rust_comp_frontend::*;
use rust_comp_opt::parse;
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

    let args = parse();
    diagnostic::print_duration("Arguments", start.elapsed());

    run::run(diagnostic::Diagnostic::new(args.files), args.opt)?;

    diagnostic::print_duration("Total", start.elapsed());
    Ok(())
}

fn handle_error(e: Error) {
    match e {
        Error::File(f) => eprintln!("Error: Could not read from {}", f),
        Error::Handled => (),
    }
}
