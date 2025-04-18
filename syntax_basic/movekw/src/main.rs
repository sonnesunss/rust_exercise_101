use std::thread;

// 闭包默认是传递引用的，引用带来的最常见问题莫过于悬垂指针问题
// rust要求，引用不能超出所引用值的作用范围，但是开辟一个新线程并将主线程作用域内的值以引用的方式传递
// 给新线程时，显然这不被rust允许，move关键字的作用是将值的所有权显式的传递给新线程.
fn main() {
    let s1 = "hello from main thread".to_string();

    // 注意这里spawn的函数签名闭包必须是fn类型
    let handle = thread::spawn(move || {
        println!("{}", s1);
    });

    handle.join().unwrap();
}
