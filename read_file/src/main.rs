use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

fn main() -> std::io::Result<()> {
    // 创建并写入文件
    let mut file = File::create("hello.txt")?;
    file.write(b"hello rust!")?;

    // 打开并读取文件
    let mut result = String::new();
    let mut file1 = File::open("hello.txt")?;
    let usize1 = file1.read_to_string(&mut result);

    println!("{:?} size {:?}", result, usize1);

    let exists = Path::new("hello.txt").try_exists()?;

    if exists {
        println!("file exists");
    } else {
        println!("file not exists!");
    }

    Ok(())
}
