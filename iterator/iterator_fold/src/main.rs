fn main() {
    let r = (0..100).into_iter().fold(0, |acc, ele| acc + ele);

    println!("Result: {:?}", r);
}
