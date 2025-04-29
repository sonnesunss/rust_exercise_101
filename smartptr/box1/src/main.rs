fn main() {
    let box1 = Box::new(123);
    let box2 = Box::new("String".to_string());

    println!("{} - {}", box1, box2);
}
