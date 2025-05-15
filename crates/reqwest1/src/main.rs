use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
struct MyIP {
    origin: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let req1 = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<MyIP>()
        .await?;

    println!("aaaaaaaaaaaaaaaaa");

    println!("{req1:#?}");
    println!("MyIP is: {}", req1.origin);

    Ok(())
}
