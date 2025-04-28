use ctrlc;
use std::sync::mpsc::channel;

fn main() {
    // 创建一个通道，接受信号
    // 通道不仅可以传输数据，还可以使用其传递信号
    let (tx, rx) = channel();
    // 注册ctrlc handler
    ctrlc::set_handler(move || tx.send(()).expect("could't send msg"))
        .expect("Error setting ctrl-c handler");

    println!("Press ctrl + c keys to quit app");
    rx.recv().expect("Could't receive from channel");
    println!("got it! Exiting...");
}
