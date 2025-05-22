use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut file = File::open("Cargo.toml").await?;
    let mut buf = [0; 10];

    let n = file.read(&mut buf[..]).await?;

    println!("The bytes: {:?}", &buf[..n]);
    println!("{}", String::from_utf8_lossy(&buf[..n]));

    Ok(())
}
