use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    let th1 = thread::spawn(move || {
        let (mlock, cvar) = &*pair2;
        let mut started = mlock.lock().unwrap();
        *started = true;

        cvar.notify_one();
        println!("notify！");
    });

    let th2 = thread::spawn(move || {
        let (mlock, cvar) = &*pair;
        let mut started1 = mlock.lock().unwrap();

        while !*started1 {
            started1 = cvar.wait(started1).unwrap();
        }

        // 条件满足后执行的动作
        thread::sleep(Duration::from_secs(10));
        println!("I\'m coming!!!");
    });

    th1.join().unwrap();
    th2.join().unwrap();

    println!("main thread finished");
}
