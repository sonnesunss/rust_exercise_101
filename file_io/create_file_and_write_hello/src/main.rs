use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut handle = File::create("hello.txt")?;
    handle.write(b"Hello, RUST!")?;

    Ok(())
}
