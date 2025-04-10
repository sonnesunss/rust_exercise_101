fn main() {
    let s1 = "Hello Rust!";

    println!("original string {}, reverse {}", s1, rev_string(s1));
}

fn rev_string(s: &str) -> String {
    s.chars().rev().collect()
}
