use std::fs::{self, File};
use std::io::{Result, Write};
use std::path::Path;

fn main() -> Result<()> {
    if !Path::new("hello.txt").try_exists()? {
        let mut file = File::create("hello.txt")?;
        let _ = file.write(b"Hello rust")?;
    }

    // copy
    let _ = fs::copy("hello.txt", "../hello.txt")?;
    // move
    let _ = fs::copy("hello.txt", "../hello1.txt")?;
    let _ = fs::remove_file("hello.txt")?;

    // delete
    let _ = fs::remove_file("../hello1.txt")?;
    let _ = fs::remove_file("../hello.txt")?;

    print!("finished");

    Ok(())
}
