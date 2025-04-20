fn main() {
    let r1 = 0..10;
    let r2 = 0..=10;

    println!("半开区间");
    r1.into_iter().for_each(|e| println!("{}", e));
    println!("闭合区间");
    r2.into_iter().for_each(|e| println!("{}", e));
}
