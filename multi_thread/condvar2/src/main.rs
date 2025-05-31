use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let pair = Arc::new((Mutex::new(true), Condvar::new()));

    let pair2 = pair.clone();

    // thread a
    let t1 = thread::spawn(move || {
        let (m_lock, cvar) = &*pair2;
        let mut started = m_lock.lock().unwrap();

        println!("继续执行，gogogogo");
        thread::sleep(Duration::from_secs(5));
        println!(
            "thread: [{:?}] changed shared state",
            thread::current().name().unwrap_or("Unamed")
        );
        *started = false;
        println!("快去通知如来佛祖");
        cvar.notify_one();
    });

    // thread b
    let t2 = thread::spawn(move || {
        let (m_lock, cvar) = &*pair;

        let _guard = cvar
            .wait_while(m_lock.lock().unwrap(), |pending1| *pending1)
            .unwrap();

        println!("通知收到了，老子现在就来收了这妖猴，{:?}", _guard);
        println!("继续执行，gogogo");
    });

    t1.join().unwrap();
    t2.join().unwrap();
}
