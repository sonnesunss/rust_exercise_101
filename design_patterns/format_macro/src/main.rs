/*
    用format！串联字符串

可以在可变的String上使用push和push_str方法来建立字符串，或者使用其
+ 操作符.

然而使用format！macro往往更方便,特别是在有字面和非字面字符串混合的地方

*/

fn say_hello(name: &str) -> String {
    // 我们可以手动构建结果String
    let mut result = "Hello ".to_owned();
    result.push_str(name);
    result.push('!');
    result
}

fn say_hello2(name: &str) -> String {
    format!("Hello {}!", name)
}

fn main() {
    println!("手动创建字符串: {}", say_hello("sun"));
    println!("使用format！macro创建字符串: {}", say_hello2("sssss"));
}
