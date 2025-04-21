fn main() {
    let slice1 = 100..=150;
    let silice2 = 200..=250;

    let r = slice1
        .into_iter()
        .zip(silice2.into_iter())
        .for_each(|e| println!("{:?}", e));

    println!("{:?}", r);
}
