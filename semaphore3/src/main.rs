use std::{sync::mpsc::channel, thread};

// 使用channel实现 两个线程交替打印10以内整数的任务，一个线程打印奇数，一个线程打印偶数
fn main() {
    let (tx_odd, rx_odd) = channel();
    let (tx_even, rx_even) = channel();

    // 通知odd先执行
    tx_odd.send(()).unwrap();

    let handle_odd = thread::spawn(move || {
        for x in (1..=9).step_by(2) {
            // 接收通知
            rx_odd.recv().unwrap();
            println!("odd -> {}", x);
            if tx_even.send(()).is_err() {
                break;
            }
        }
    });

    let handle_even = thread::spawn(move || {
        for x in (2..=10).step_by(2) {
            rx_even.recv().unwrap();
            println!("even -> {}", x);
            if tx_odd.send(()).is_err() {
                break;
            }
        }
    });

    handle_odd.join().unwrap();
    handle_even.join().unwrap();
}
