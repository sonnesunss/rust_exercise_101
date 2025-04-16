use std::sync::{Arc, RwLock};
use std::thread;

// 读写锁基础练习
// 读写锁就是互斥锁的一种变体， 允许多个读锁同时进入临界区，仅允许一个写锁进入临界区写入，且当有一个写锁在临界区时
// 不允许同时出现任何的读锁进入临界区
fn main() {
    println!("main thread started");
    let shared = Arc::new(RwLock::new(String::from("hello rwlock")));
    let mut handles = vec![];

    for x in 0..=100 {
        let shared_cloned = shared.clone();

        let handle = thread::spawn(move || {
            println!("thread{} created and starting", x);
            let r = shared_cloned.read().unwrap();
            println!("{}", r);
            println!("thread{} finished", x);
        });

        handles.push(handle);
    }

    /*
    for handle in handles {
        handle.join().unwrap();
    }
    */

    handles.into_iter().for_each(|handle| {
        handle.join().unwrap();
    });

    println!("main thread finished!");
}
