use tokio::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let server = TcpListener::bind("127.0.0.1:9000").await?;

    loop {
        let (tcp_stream, addr) = server.accept().await?;
        println!("Client is incoming from Addr {:?}", addr);

        let (mut stream_read, mut stream_write) = io::split(tcp_stream);
        let mut buf = vec![0; 1024];

        match stream_read.read(&mut buf).await {
            Ok(0) => {
                println!("Client {:?} closed the connection", addr);
                break;
            }
            Ok(n) => {
                println!(
                    "Read {n} bytes from Client sending data {:?}",
                    String::from_utf8_lossy(&buf[..n])
                );
                if let Err(e) = stream_write.write_all(&buf[..n]).await {
                    eprintln!("failed to write to client {}: {}", addr, e);
                    break;
                }
            }
            Err(e) => {
                eprintln!("{e}");
                break;
            }
        }
    }

    Ok(())
}
