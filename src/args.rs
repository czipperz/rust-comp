use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "rust-comp",
    about = "A compiler for the Rust programming language focusing on correctness, compilation speed, and being embeddable."
)]
pub struct Opt {
    pub files: Vec<String>,
}

pub fn parse() -> Opt {
    Opt::from_args()
}
