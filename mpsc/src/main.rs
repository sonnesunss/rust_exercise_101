use std::sync::{Arc, mpsc};
use std::thread;

fn main() {
    // 创建一个channel，返回一个元组，接收者，发送者
    let (tx, rx) = mpsc::channel();
    let arc_tx = Arc::new(tx);
    let mut handles = vec![];

    for x in 0..10 {
        let arc_txc = arc_tx.clone();
        let t = thread::spawn(move || {
            arc_txc.send(format!("message {}", x)).unwrap();
        });

        handles.push(t);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    drop(arc_tx);

    for x in rx {
        println!("received: {:?}", x);
    }

    println!("end");
}
