fn main() {
    let num = Some(12321);
    describe_option(num);
}

/*
编写一个函数 describe_option(opt: Option<i32>)，使用 match 根据输入：

如果是 Some(num)，打印 "有值: num"

如果是 None，打印 "没有值"
*/
fn describe_option(opt: Option<i32>) {
    match opt {
        None => println!("没有值"),
        Some(num) => println!("有值{}", num),
    }
}
