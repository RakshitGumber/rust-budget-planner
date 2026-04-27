use chrono::Local;
use std::fs::File;
use std::io::{self, Write};

fn formatted_time_entry() -> String {
    let now = Local::now();
    let time_str = now.format("%a, %b %d %Y %I:%M:%S %p\n").to_string();
    time_str
}

fn logger(filename: &str, string: &[u8]) -> io::Result<()> {
    let mut f = File::create(filename)?;
    f.write_all(string)?;
    Ok(())
}

fn log(filename: &str) -> io::Result<()> {
    let time_str = formatted_time_entry();
    logger(filename, time_str.as_bytes())?;
    Ok(())
}

fn main() {
    log("log.txt").expect("Unable to write")
}
