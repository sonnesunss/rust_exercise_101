use std::thread;

/*
join的重要性， 让主线程等待子线程结束，如果不等待则主线程结束，子线程就会被迫终止，因为整个程序结束，os就全部回收了该程序分配的所有资源

thread::spawn函数在正常结束后会返回一个JoinHandle类型的值，使用该类型下的join方法使主线程等待线程的结束，如果创建不成功则返回Error

*/
fn main() {
    println!("main thread starting...");
    // 如果不等待，子线程是否能够正常运行、结束是不可预料的
    let _ = thread::spawn(|| {
        println!("i'm child thread");
    });
    println!("main thread finished...")
}
