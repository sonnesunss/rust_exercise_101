fn main() {
    let slice1 = 0..=10;
    let r = slice1
        .into_iter() // 转换成迭代器，具备所有权
        .enumerate() // 适配器迭代器
        .for_each(|e| println!("{}: {}", e.0, e.1)); // 消费者迭代器

    println!("{:?}", r);
}
