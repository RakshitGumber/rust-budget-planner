use std::fs::File;
use std::io;

fn main() -> io::Result<()> {
    let mut f = File::create("foo.txt")?;
    Ok(())
}