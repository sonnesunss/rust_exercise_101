fn main() {
    let arr1 = [1, 10, 32, 56, 93, 50, 86];
    let arr3 = "Hello Rust String smart pointer".to_string();
    let arr4 = vec![1, 2, 3, 4, 5];

    let _: Vec<_> = arr3
        .chars()
        .map(|e| {
            println!("arr3 ele -> {}", e);
        })
        .collect();

    for x in arr1 {
        println!("arr1 ele -> {}", x * x);
    }

    for x in 0..arr4.len() {
        println!("arr4 pos, ele: {}, {}", x, arr4[x]);
    }
}
