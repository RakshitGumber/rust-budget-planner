use std::fs::File;
use std::io::{self, Write};

fn logger(filename: &str, string: &str) -> io::Result<()>{
    let mut f = File::create(filename)?;
    f.write_all(string.as_bytes())?;
    Ok(())
}

fn main() {
    logger("log.txt", "It's working").expect("Unable to write")
}