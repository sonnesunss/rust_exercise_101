use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // 演示死锁
    // 两个线程互相持有对方等待的锁造成死锁
    let lock_a = Arc::new(Mutex::new(Vec::<String>::new()));
    let lock_b = Arc::new(Mutex::new(Vec::<String>::new()));

    let cloned_lock_a1 = lock_a.clone();
    let cloned_lock_b1 = lock_b.clone();
    let cloned_lock_a2 = lock_a.clone();
    let cloned_lock_b2 = lock_b.clone();

    let t1 = thread::spawn(move || {
        println!("t1线程开始执行并尝试获取lock_a互斥锁");
        let _a = cloned_lock_a1.lock().unwrap();
        println!("t1线程获得lock_a互斥锁进入临界区执行");
        thread::sleep(Duration::from_secs(1));

        println!("t1线程尝试获取lock_b互斥锁");
        let _b = cloned_lock_b1.lock().unwrap();
        println!("t1线程获取lock_b互斥锁并进入临界区执行");
        thread::sleep(Duration::from_secs(1));
    });

    let t2 = thread::spawn(move || {
        println!("t2线程开始执行并尝试获取lock_b互斥锁");
        let _a = cloned_lock_b2.lock().unwrap();
        println!("t2线程获得lock_b互斥锁进入临界区执行");
        thread::sleep(Duration::from_secs(1));

        println!("t2线程尝试获取lock_a互斥锁");
        let _b = cloned_lock_a2.lock().unwrap();
        println!("t2线程获取lock_a互斥锁并进入临界区执行");
        thread::sleep(Duration::from_secs(1));
    });

    t1.join().unwrap();
    t2.join().unwrap();

    println!(
        "演示死锁，这里死锁的原因是互相等待对方释放自己需要的锁，解决方法是严格按照固定顺序获得锁"
    );
}
