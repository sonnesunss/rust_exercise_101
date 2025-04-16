use std::sync::{Arc, RwLock};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

fn main() {
    let mut handles = vec![];
    let shared = Arc::new(RwLock::new(Vec::<i32>::new()));

    for x in 0..=10 {
        let shared_clone = shared.clone();

        let t_handle = thread::spawn(move || {
            println!("thread-{} try_read", x);
            let result = shared_clone.try_read();

            match result {
                Ok(r) => println!("thread-{} read -> {:?}", x, r),
                Err(e) => eprintln!("thread-{} error -> {}", x, e),
            }
        });

        handles.push(t_handle);
    }

    handles.into_iter().for_each(|handle| {
        handle.join().unwrap();
    });

    println!("main thread finished!");
}
