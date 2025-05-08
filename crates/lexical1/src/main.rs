use lexical;

fn main() {
    let _a1 = lexical::to_string(5);
    let _a2 = lexical::parse::<i32, _>("123");

    println!("{:?} -- {:?}", _a1, _a2);
}
