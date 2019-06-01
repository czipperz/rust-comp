use std::fs::File;
use std::io::{BufRead, BufReader, Result};

pub fn read_file(name: &str) -> Result<Vec<String>> {
    BufReader::new(File::open(name)?).lines().collect()
}
