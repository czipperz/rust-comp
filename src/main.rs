use rust_comp_diagnostic::{print_duration, Diagnostic};
use rust_comp_opt::parse;
use rust_comp_run::{run, Error};
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
    let args = parse();
    run(Diagnostic::new(args.files), args.opt)?;
    Ok(())
}

fn handle_error(e: Error) {
    match e {
        Error::File(f) => eprintln!("Error: Could not read from {}", f),
        Error::Handled => (),
    }
}
