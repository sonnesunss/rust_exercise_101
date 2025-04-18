use std::sync::{Arc, RwLock};
use std::thread;
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

            thread::sleep(Duration::from_secs(1));

            println!("thread-{} try write", x);
            let result = shared_clone.try_write();
            match result {
                Ok(mut r) => r.push(x),
                Err(e) => eprintln!("thread-{} try write error {}", x, e),
            }

            println!("thread-{} finished", x);
        });

        handles.push(t_handle);
    }

    handles.into_iter().for_each(|handle| {
        handle.join().unwrap();
    });

    println!("{:?}", shared.read().unwrap());
    println!("main thread finished!");
}
