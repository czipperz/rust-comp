use std::fs::File;
use std::io;
pub fn read_file(name: &str) -> io::Result<String> {
    use io::Read;
    let mut contents = String::new();
    File::open(name)?.read_to_string(&mut contents)?;
    Ok(contents)
}
