// 迭代器
fn main() {
    let v1 = vec![1, 2, 5, 3, 8, 5, 31, 89, 432, 32798, 3809, 937, 74];
    // iter()获得v1的不可变引用迭代器
    let _ = v1.iter().for_each(|ele| {
        println!("{}", ele);
    });

    let v2: Vec<_> = v1.iter().map(|ele| ele * 2).collect();

    println!("v1 {:?}", v1);
    println!("v2 {:?}", v2);

    // into_iter()获得v2的所有权迭代器，迭代器可以获得v2所有元素的所有权
    // 之后再通过v2访问元素rust静态编译器不会编译通过
    let v3: Vec<_> = v2.into_iter().filter(|ele| ele % 2 == 0).collect();

    println!("v3: {:?}", v3);
    println!("Hello, world!");
}
