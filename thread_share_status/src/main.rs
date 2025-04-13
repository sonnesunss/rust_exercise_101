use std::sync::{Arc, Mutex};
use std::thread;

// 多线程之间共享状态

fn main() {
    let status = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let status_clone = status.clone();
        let t = thread::spawn(move || {
            let mut num = status_clone.lock().unwrap();
            *num += 1;

            println!("{:?}", thread::current().id());
        });

        handles.push(t);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{}", status.lock().unwrap());
}
