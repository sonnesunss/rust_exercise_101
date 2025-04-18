use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    /* 实现一个程序，多个线程往同一个 Vec<String> 中添加数据，最终主线程打印整个 Vec 内容 */
    let mut handles = vec![];
    let shared = Arc::new(Mutex::new(Vec::<String>::new()));

    for x in 0..10 {
        let arc_clone = shared.clone();
        let t_handle = thread::spawn(move || {
            let mut num = arc_clone.lock().unwrap();
            num.push(format!("message {}", x));
        });

        handles.push(t_handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{:?}", shared.clone().lock().unwrap());
}
