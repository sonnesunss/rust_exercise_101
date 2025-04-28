use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::u8;

fn main() {
    let f1 = "birds";
    let r = read_whole_file(f1);

    match r {
        Ok(r) => {
            println!("{}", String::from_utf8_lossy(&r));
        }
        Err(e) => eprintln!("{}", e),
    }
}

fn read_whole_file(name: &str) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(Path::new(name))?;
    let file_meta = file.metadata()?;

    let mut buffer = Vec::with_capacity(file_meta.len() as usize);
    let _rsize = file.read_to_end(&mut buffer);

    Ok(buffer)
}
