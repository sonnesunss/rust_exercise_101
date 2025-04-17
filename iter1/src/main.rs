// 迭代器
fn main() {
    let v1 = vec![1, 2, 5, 3, 8, 5, 31, 89, 432, 32798, 3809, 937, 74];
    let _ = v1.iter().for_each(|ele| {
        println!("{}", ele);
    });

    let v2: Vec<_> = v1.iter().map(|ele| ele * 2).collect();

    println!("v1 {:?}", v1);
    println!("v2 {:?}", v2);

    println!("Hello, world!");
}
