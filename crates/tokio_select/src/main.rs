use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() {
    let task1 = async {
        sleep(Duration::from_secs(1)).await;
        println!("Task 1 completed");
    };

    let task2 = async {
        sleep(Duration::from_secs(2)).await;
        println!("Task 2 completed");
    };

    let task3 = async {
        sleep(Duration::from_secs(1)).await;
        println!("Task 3 completed");
    };

    tokio::select! {
        _ = task1 => println!("Task 1 finished first"),
        _ = task2 => println!("Task 2 finished first"),
        _ = task3 => println!("Task 3 finished first"),
    }
}
