use rust_comp_opt::parse;
use rust_comp_run::{run, Error};

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
    run(parse())
}

fn handle_error(e: Error) {
    match e {
        Error::File(f) => eprintln!("Error: Could not read from {}", f),
        Error::Handled => (),
    }
}
