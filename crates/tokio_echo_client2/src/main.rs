use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut client = TcpStream::connect("127.0.0.1:9000").await?;
    let mut buf = [0; 1024];

    client.write_all(b"Hello Tcp server!!!").await?;

    match client.read(&mut buf).await {
        Ok(0) => {
            println!("server closed this connection");
        }
        Ok(n) => {
            println!("{:?}", String::from_utf8_lossy(&buf[..n]));
        }
        Err(e) => {
            eprintln!("{e}");
        }
    }

    Ok(())
}
