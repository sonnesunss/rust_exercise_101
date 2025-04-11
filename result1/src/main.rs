fn main() {
    let r = parse_number("12321");
    println!("{:?}", r);
}

/*
编写一个函数 parse_number(s: &str) -> Result<i32, std::num::ParseIntError>，尝试将字符串转换成整数。

在 main 中调用该函数，对返回的 Result 使用 match 进行模式匹配，并在出错时输出错误信息。
*/
fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse::<i32>()
}
