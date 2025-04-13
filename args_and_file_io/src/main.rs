use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // step1: 收集启动参数
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("useage: {} <file path>", args[0]);
        eprintln!("cargo run -- file_path");
        std::process::exit(1);
    }

    // step2: 尝试打开文件
    let file_path = &args[1];

    let file = File::open(file_path);
    let file = match file {
        Ok(f) => f,
        Err(e) => {
            eprintln!("无法打开文件 {} 错误原因: {}", file_path, e);
            std::process::exit(1);
        }
    };

    // step3: 读取打开的文件
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        match line {
            Ok(content) => println!("{:>4}: {}", index + 1, content),
            Err(e) => {
                eprintln!("读取文件时出错: {}", e);
                std::process::exit(1);
            }
        }
    }
}
