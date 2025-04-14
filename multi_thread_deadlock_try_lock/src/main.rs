use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/*
 * 使用try_lock尝试重新获取锁，如果在一定条件下仍为获取到锁则释放已获取的锁，这在不方便严格顺序获取锁时很有用.
*/
fn main() {
    let lock_a = Arc::new(Mutex::new(Vec::<String>::new()));
    let lock_b = Arc::new(Mutex::new(Vec::<String>::new()));

    let cloned_lock_a1 = lock_a.clone();
    let cloned_lock_b1 = lock_b.clone();

    let t1_handle = thread::spawn(move || {
        loop {
            println!("线程1创建成功、运行中");
            if let Ok(a_guard) = cloned_lock_a1.try_lock() {
                println!("线程1获得lock_a互斥锁，并进入临界区执行");
                thread::sleep(Duration::from_secs(1)); // 模拟繁重运算任务

                if let Ok(b_guard) = cloned_lock_b1.try_lock() {
                    println!("线程1获得lock_b互斥锁，并进入临界区执行");
                    drop(b_guard);
                    drop(a_guard);

                    break;
                } else {
                    println!("尝试获取lock_b锁失败，等待尝试");
                    drop(a_guard);
                }
            }
            println!("线程1: 获取lock_a锁失败，歇会儿再试");
            thread::sleep(Duration::from_secs(1));
        }
    });

    let cloned_lock_a2 = lock_a.clone();
    let cloned_lock_b2 = lock_b.clone();

    let t2_handle = thread::spawn(move || {
        loop {
            println!("线程2创建成功、运行中");
            if let Ok(a_guard) = cloned_lock_b2.try_lock() {
                println!("线程2获得lock_b互斥锁，并进入临界区执行");
                thread::sleep(Duration::from_secs(1)); // 模拟繁重运算任务

                if let Ok(b_guard) = cloned_lock_a2.try_lock() {
                    println!("线程2获得lock_a互斥锁，并进入临界区执行");
                    drop(b_guard);
                    drop(a_guard);

                    break;
                } else {
                    println!("尝试获取lock_a锁失败，等待尝试");
                    drop(a_guard);
                }
            }

            println!("线程2: 获取lock_b锁失败，稍后再重新试试");
            thread::sleep(Duration::from_secs(1));
        }
    });

    t1_handle.join().unwrap();
    t2_handle.join().unwrap();

    println!("main finished");
}
