use tokio::time::{self, Duration};

#[tokio::main]
async fn main() {
    println!("aaaaaaa");
    let r = tokio::spawn(async {
        time::sleep(Duration::from_secs(10)).await;
        return 123;
    });

    println!("bbbbb");

    let a = r.await.unwrap();

    println!("{a}");
}
