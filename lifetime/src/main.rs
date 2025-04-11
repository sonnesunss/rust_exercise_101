fn main() {
    println!("Hello, world!");

    println!("{}", longest("hello", "world"));
}

// 编写一个函数 longest<'a>(s1: &'a str, s2: &'a str) -> &'a str，该函数返回两个字符串切片中较长的一个。
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.chars().count() >= s2.chars().count() {
        return s1;
    }
    s2
}
