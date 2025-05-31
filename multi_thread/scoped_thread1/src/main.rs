use std::thread;

fn main() {
    let v1 = vec![1, 2, 3];
    let mut a = 0;

    thread::scope(|s| {
        s.spawn(|| {
            a += v1[0] + v1[1];
        });
    });

    println!("a = {}", a);
}
