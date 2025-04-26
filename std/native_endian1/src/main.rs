fn main() {
    // 判断机器大端小端
    // 1. 使用cfg!(target_endian = "") macro , 这是rust编译器在编译时就确定好的宏
    if cfg!(target_endian = "little") {
        print!("Machine is little-endian");
    } else if cfg!(target_endian = "big") {
        println!("Machine is big-endian");
    }
}
