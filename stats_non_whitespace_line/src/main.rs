use std::fs::File;
use std::io::{BufRead, BufReader, Result};

/*
读取并统计非空行 -- 所谓非空行就是去掉空白后长度为0的行

按行读取，然后在每一行的String类型上使用trim()方法去掉空白，然后判断长度是否为0,是则就是空白行，不是则不是空白行
*/
fn main() -> Result<()> {
    let file = File::open("nobody.txt")?;
    let reader = BufReader::new(file);
    let mut lines_count = 0;

    for x in reader.lines() {
        println!("{:?}", x);
        match x {
            Ok(x) => {
                if !x.trim().is_empty() {
                    lines_count += 1;
                }
            }
            Err(e) => eprintln!("{}", e),
        }
    }

    println!("lines count -> {}", lines_count);

    Ok(())
}
