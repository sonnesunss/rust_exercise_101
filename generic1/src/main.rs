fn min_max<T: PartialOrd + Copy>(slice: &[T]) -> Option<(T, T)> {
    // 使用迭代器，若切片为空，则返回 None
    let mut iter = slice.iter();
    // 获取第一个元素作为初始的 (min, max)
    let first_init = *iter.next()?;
    Some(iter.fold((first_init, first_init), |(min, max), &x| {
        (if x < min { x } else { min }, if x > max { x } else { max })
    }))
}

fn main() {
    let nums = [3, 5, 1, 9, 2];
    match min_max(&nums) {
        Some((min, max)) => println!("最小值: {}, 最大值: {}", min, max),
        None => println!("切片为空"),
    }
}
