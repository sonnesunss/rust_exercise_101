use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::Path;

fn main() -> Result<()> {
    if Path::new("nobody.txt").try_exists()? {
        let file = File::open("nobody.txt")?;
        let reader = BufReader::new(file);

        for (index, line) in reader.lines().enumerate() {
            let line = line?;
            println!("{}: {}", index + 1, line);
        }
    } else {
        println!("file not exists");
    }
    println!("Hello, world!");

    Ok(())
}
