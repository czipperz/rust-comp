#![feature(custom_attribute)]

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "rust-comp",
    about = "A compiler for the Rust programming language focusing on correctness, compilation speed, and being embeddable."
)]
pub struct Args {
    pub files: Vec<String>,
    #[structopt(flatten)]
    pub opt: Opt,
}

#[derive(StructOpt, Debug)]
pub struct Opt {}

pub fn parse() -> Args {
    Args::from_args()
}
