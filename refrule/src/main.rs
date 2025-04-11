fn main() {
    let mut hello = "hello string!".to_string();
    // 同一作用域允许同时存在多个不可变引用
    let h1 = &hello;
    let h2 = &hello;
    let h3 = &hello;
    // 同一作用域不允许可变、不可变引用同时出现
    // let h4 = &mut hello;
    println!("{} {} {}", h1, h2, h3);

    // h4.push('a');
}
