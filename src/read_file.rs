use std::fs::File;
use std::io::{Read, BufReader, Result};

pub fn read_file(name: &str) -> Result<String> {
    let mut contents = String::new();
    BufReader::new(File::open(name)?).read_to_string(&mut contents)?;
    Ok(contents)
}
