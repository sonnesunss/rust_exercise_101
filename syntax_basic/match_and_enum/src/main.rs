fn main() {
    let m = Message::ChangeColor(13, 32, 123);

    f1(m);
}

#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn f1(msg: Message) {
    match msg {
        Message::ChangeColor(a, b, c) => {
            println!("{} {} {}", a, b, c);
        }
        Message::Move { x, y } => {
            println!("{} {}", x, y);
        }
        Message::Write(s) => {
            println!("{}", s);
        }
        Message::Quit => {
            println!("quit now");
        }
    }
}
