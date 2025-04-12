fn main() {
    let mut acc = 0;
    let v1 = vec![1, 3, 4, 78, 31, 66, 93];

    // 过滤出所有偶数
    let v2: Vec<i32> = v1.iter().filter(|ele| *ele % 2 == 0).copied().collect();
    // 所有元素的平方和
    let v3: Vec<i32> = v1.iter().map(|ele| *ele * 2).collect();
    // 所有偶数的和
    // acc = v2.iter().fold(0, |acc, ele| acc + ele);
    acc = v1
        .iter()
        .filter(|ele| *ele % 2 == 0)
        .copied()
        .fold(0, |acc, ele| acc + ele);

    println!("v1{:?}中的偶数集v2{:?}", v1, v2);
    println!("v1{:?}平方v3{:?}", v1, v3);
    println!("v1{:?}内所有偶数元素的和{:?}", v1, acc);
}

/*
创建一个 Vec<i32> 并填充一些整数。

使用迭代器方法（如 filter、map、fold 等）完成以下功能：

过滤出偶数

计算所有偶数的和，并打印结果
*/
