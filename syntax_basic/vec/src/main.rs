fn main() {
    let mut v1 = vec![1];

    for x in 61..66 {
        v1.push(x);
    }

    println!("sum v1 {}", sum_vec(v1));
}

fn sum_vec(v: Vec<i32>) -> i32 {
    v.iter().map(|&x| x).sum()
}
