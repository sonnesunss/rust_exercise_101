use crossbeam::channel::bounded;
use std::thread;

fn main() {
    let (tx, rx) = bounded(5);

    let _join = thread::spawn(move || {
        let r = rx.clone().recv();
        println!("{:?}", r);
    });

    for x in 0..100 {
        tx.send(x).unwrap();
    }
}
