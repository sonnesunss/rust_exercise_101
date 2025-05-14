fn main() {
    // Dereferencing raw ptrs
    let x = 10;
    let r1 = &x as *const i32; // Immutable raw pointer

    unsafe {
        println!("r1 {:?} ptr to: {}", r1, *r1);
    }
}
