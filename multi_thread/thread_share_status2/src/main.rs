use std::sync::{Arc, Mutex};
use std::thread;

// 启动五个线程，每个线程对共享变量+1, 直到加到100结束
fn main() {
    println!("info: main thread started!");

    let mut handles = vec![];
    let shared = Arc::new(Mutex::new(0));

    for _ in 0..5 {
        let arcs = shared.clone();
        let t = thread::spawn(move || {
            loop {
                let mut num = arcs.lock().unwrap();
                if *num < 100 {
                    *num += 1;
                } else {
                    break;
                }
                println!("{:?}", thread::current().id());
            }
        });

        handles.push(t);
    }

    for x in handles {
        x.join().unwrap();
    }

    println!("info: main thread finished {}", *shared.lock().unwrap());
}
