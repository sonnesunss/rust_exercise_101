use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

fn main() {
    let shared = Arc::new(RwLock::new(Vec::<i32>::new()));
    let mut handles = vec![];

    for x in 0..10 {
        let shared_clone = shared.clone();
        let t_handle = thread::spawn(move || {
            {
                let rr = shared_clone.read().unwrap();
                println!("thread-{} 此时读取到的向量内容是: {:?}", x, rr);
            }
            thread::sleep(Duration::from_millis(500));
            {
                let mut ww = shared_clone.write().unwrap();
                ww.push(x);
            }
            println!("thread-{} finished", x);
        });

        handles.push(t_handle);
    }

    handles.into_iter().for_each(|handle| {
        handle.join().unwrap();
    });

    println!("Main thread finished!");
}
