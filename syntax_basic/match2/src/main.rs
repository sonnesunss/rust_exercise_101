fn main() {
    println!("Hello, world!");

    let r = f2(2);

    println!("{:?}", r);

    let d1 = "123218757321321";

    f1(d1);

    let d2 = String::from("23132132135");
    f3(d2);
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
enum Token {
    One,
    Two,
    Three,
    Four,
    Five,
    Error(String),
}

#[allow(dead_code)]
fn f2(a1: i32) -> Option<Token> {
    match a1 {
        1 => Some(Token::One),
        2 => Some(Token::Two),
        3 => Some(Token::Three),
        4 => Some(Token::Four),
        5 => Some(Token::Five),
        _ => Some(Token::Error("Unexpected symbol".to_string())),
    }
}

#[allow(dead_code)]
fn f1(a1: &str) {
    let mut iter = a1.chars().peekable();

    println!("---------------------");
    while let Some(b) = iter.peek() {
        println!("{:?}", b);
        iter.next();
    }
}

#[allow(dead_code)]
fn f3(a1: String) {
    let mut iter = a1.chars().peekable();

    println!("--------------------");
    while let Some(b) = iter.peek() {
        println!("{:?}", b);
        iter.next();
    }
}
