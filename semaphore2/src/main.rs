use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::task;

/*
两个线程交替执行，线程a打印10以内的奇数，线程b打印10以内的偶数
*/
#[tokio::main]
async fn main() {
    let odd_sema = Arc::new(Semaphore::new(1));
    let even_sema = Arc::new(Semaphore::new(0));

    let odd_sema_cloned1 = odd_sema.clone();
    let even_sema_cloned1 = even_sema.clone();

    let handle1 = task::spawn(async move {
        for x in (2..=10).step_by(2) {
            let _permit = even_sema_cloned1.acquire().await.unwrap();
            println!("even {}", x);
            drop(_permit);
            odd_sema_cloned1.add_permits(1);
            tokio::task::yield_now().await;
        }
    });

    let handle2 = task::spawn(async move {
        for x in (1..=10).step_by(2) {
            let _permit = odd_sema.acquire().await.unwrap();
            println!("odd {}", x);
            drop(_permit);
            even_sema.add_permits(1);
            tokio::task::yield_now().await;
        }
    });

    tokio::try_join!(handle1, handle2).unwrap();
    println!("main thread finished");
}
