use std::thread;
use std::time::Duration;
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    let fname = "demo.md";
    let mut file = File::create(fname).await?;
    let n = file.write(b"# Title1").await?;
    println!("Wrote the first {} bytes of '# Title1'.", n);

    file.flush().await?;

    thread::sleep(Duration::from_secs(1));

    let mut file1 = File::open(fname).await?;

    let mut buffer = Vec::new();
    let r = file1.read_to_end(&mut buffer).await?;
    println!(
        "read the content of file -> {} bytes -> {}",
        r,
        String::from_utf8_lossy(&buffer)
    );

    Ok(())
}
