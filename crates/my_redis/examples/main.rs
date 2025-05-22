use mini_redis::{Result, client};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;
    client.set("hello123", "world123".into()).await?;

    let result = client.get("hello123").await?;
    println!("got value from the server; result={:?}", result);

    Ok(())
}
