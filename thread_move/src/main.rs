use std::thread;

fn main() {
    let a = "Hello another thread".to_string();

    let t1_handler = thread::spawn(move || {
        println!("{}", a);
    })
    .join()
    .unwrap();

    println!("main thread finnished");
}
