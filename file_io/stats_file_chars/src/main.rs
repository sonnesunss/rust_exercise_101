use std::fs::File;
use std::io::{BufReader, Read, Result};

fn main() -> Result<()> {
    let file = File::open("nobody.txt")?;
    let mut reader = BufReader::new(file);

    let mut r = String::new();

    let _ = reader.read_to_string(&mut r);
    let char_count = r.chars().count();

    println!("char count -> {}", char_count);

    Ok(())
}
