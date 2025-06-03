use std::{sync::Mutex, sync::Once, thread};

fn main() {
    static START: Once = Once::new();
    static mut acc: Option<Mutex<i32>> = None;

    let acc_clone1 = unsafe { acc.as_ref().clone() };
    let acc_clone2 = unsafe { acc.as_ref().clone() };

    let t1 = thread::spawn(move || {
        START.call_once(|| unsafe {
            acc = Some(Mutex::new(0));
        });

        for x in 0..10 {
            let mut acc1 = acc_clone2.unwrap().lock().unwrap();
            *acc1 += x;
        }
    });

    let t2 = thread::spawn(move || {
        START.call_once(|| unsafe { acc = Some(Mutex::new(0)) });

        for x in 0..10 {
            let mut acc1 = acc_clone1.unwrap().lock().unwrap();
            *acc1 += x;
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();
}
