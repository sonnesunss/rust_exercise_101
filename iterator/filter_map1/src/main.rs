fn main() {
    // 创建一个同时filter, map的迭代器，换句话说它最终返回的是一个迭代器
    let iter1 = (0..100).filter_map(|e| Some(e % 2 == 0));

    iter1.for_each(|e| println!("{}", e));

    let v1 = vec![
        "1", "two", "three", "4", "five", "6", "seven", "eight", "night", "10",
    ];

    let iter2 = v1.iter().filter_map(|ele| ele.parse::<i32>().ok());

    iter2.for_each(|ele| {
        println!("{}", ele);
    });
}
