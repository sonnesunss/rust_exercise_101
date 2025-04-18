use std::thread;

fn main() {
    let t1_handler = thread::spawn(|| {
        println!("Hello from thread1");
    });

    let t2_handler = thread::spawn(|| {
        println!("Hello from thread2");
    });

    t1_handler.join().unwrap();
    t2_handler.join().unwrap();

    println!("all done!")
}
