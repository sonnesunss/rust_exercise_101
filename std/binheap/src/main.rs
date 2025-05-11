use std::collections::BinaryHeap;

fn main() {
    println!("Hello, world!");

    let mut bh = BinaryHeap::new();

    bh.push(1);
    bh.push(2);
    bh.push(3);
    bh.push(4);
    bh.push(5);
    bh.push(6);
    bh.push(7);
    bh.push(8);

    println!("bh is {:?}", bh);
}
