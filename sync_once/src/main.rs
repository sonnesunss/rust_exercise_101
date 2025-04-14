use std::thread;
use std::sync::Once;

/*
 * 使用Once同步原语创建一个多线程环境下只在线程内执行一次的逻辑，
 * 例如初始化，无论多少个线程执行此段代码，都会保证只执行一次
 */
static INIT:Once = Once::new();

fn main() {
    let mut  handles = vec![];

    for x in 0..5 {
        let handle = thread::spawn(move ||{
            INIT.call_once(||{
                init();
            });
            
            println!("{}", x);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("main thread");
}

fn init() {
    println!("INIT ONCE");
}
