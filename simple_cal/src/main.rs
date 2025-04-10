use std::io;

fn main() {
    println!("enter a num ->");
    let num1 = loop {
        match get_num_input() {
            Ok(num) => break num,
            Err(_) => {
                println!("错误：请输入有效的数字");
                continue;
            }
        }
    };

    println!("enter another num ->");
    let num2 = loop {
        match get_num_input() {
            Ok(num) => break num,
            Err(_) => {
                println!("错误，请输入有效的数字");
                continue;
            }
        }
    };

    println!("请输入运算符(+, - * /) ->");

    let mut operator = String::new();
    io::stdin().read_line(&mut operator).expect("error");
    let operator = operator.trim();

    match operator {
        "+" => println!("{} + {} = {}", num1, num2, num1 + num2),
        "-" => println!("{} - {} = {}", num1, num2, num1 - num2),
        "*" => println!("{} * {} = {}", num1, num2, num1 * num2),
        "/" => {
            if (num2 - 0.0).abs() < 1e-10 {
                println!("错误，除数不能为0");
            } else {
                println!("{} / {} = {}", num1, num2, num1 / num2);
            }
        }
        _ => println!("错误，无效的运算符，请使用+, -, *, /"),
    }
}

fn get_num_input() -> Result<f64, std::io::Error> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;
    Ok(input
        .trim()
        .parse::<f64>()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "无法解析为数字"))?)
}
