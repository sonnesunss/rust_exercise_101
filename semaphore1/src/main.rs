use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;
use tokio::task;

/*
    假设有 10 个任务，但希望最多只允许 3 个线程同时执行

    rust std并没有自带semaphore的实现，所以这里使用tokio异步运行时提供的semaphore
*/
#[tokio::main]
async fn main() {
    // 信号量底层实现已经考虑到了并发修改问题，因此这里无须再套一个mutex
    let sema = Arc::new(Semaphore::new(3));
    let mut handles = vec![];

    for x in 0..10 {
        let cloned_sema = sema.clone();

        let handle = task::spawn(async move {
            // 显式P操作
            let _permit = cloned_sema.acquire().await.unwrap();
            println!("线程{}正在执行", x);
            tokio::time::sleep(Duration::from_secs(1)).await;
            println!("线程{}执行完毕", x);
            // 退出自动执行V操作
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }
}
