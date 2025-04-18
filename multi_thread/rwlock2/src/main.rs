use std::sync::{Arc, RwLock};
use std::thread;

// 多个读锁，一个写锁
fn main() {
    println!("main thread started");
    let shared = Arc::new(RwLock::new(String::new()));
    let mut handles = vec![];

    for x in 0..10 {
        let shared_clone = shared.clone();
        let t_handle = thread::spawn(move || {
            println!("thread {}", x);
            if x == 3 {
                println!("acquire write lock...");
                let mut r = shared_clone.write().unwrap();
                r.push(x.to_string().parse::<char>().unwrap());
                println!("write finished");
            } else {
                let re = shared_clone.read().unwrap();
                println!("thread {} value -> {}", x, re);
            }
            println!("thread {} finished", x);
        });

        handles.push(t_handle);
    }

    handles.into_iter().for_each(|handle| {
        handle.join().unwrap();
    });

    println!("main thread finished");
}
