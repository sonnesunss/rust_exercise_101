fn main() {
    println!("Hello, world!");

    let r = calculate(Operation::Add, 1.1, 2.2);
    let r2 = calculate(Operation::Divide, 23.32, 0.0);

    println!("{:?}", r);
    println!("{:?}", r2);
}

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn calculate(op: Operation, a: f64, b: f64) -> Result<f64, String> {
    match op {
        Operation::Add => Ok(a + b),
        Operation::Subtract => Ok(a - b),
        Operation::Multiply => Ok(a * b),
        Operation::Divide => {
            if b == 0.0 {
                return Err("除数不能为零".to_string());
            } else {
                Ok(a / b)
            }
        }
    }
}
