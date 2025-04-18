use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

fn main() -> std::io::Result<()> {
    if Path::new("hello.txt").try_exists()? {
        println!("文件存在直接追加模式下写入");
        let us1 = OpenOptions::new()
            .append(true)
            .open("hello.txt")?
            .write(b"append content\n")?;

        println!("写入字节数为 {}", us1);
    } else {
        let mut file = File::create("hello.txt")?;
        let _ = file.write(b"Hello file IO\n");

        println!("文件不存在， 创建再写入");
        let us1 = OpenOptions::new()
            .append(true)
            .open("hello.txt")?
            .write(b"append content\n")?;

        println!("写入字节数为 {}", us1);
    }

    Ok(())
}
