use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let server = TcpListener::bind("127.0.0.1:9000").await?;

    loop {
        let (tcp_stream, addr) = server.accept().await?;
        let (mut tsr, mut tsw) = io::split(tcp_stream);

        tokio::spawn(async move {
            match io::copy(&mut tsr, &mut tsw).await {
                Ok(n) => {
                    println!("Echoed {} bytes for client {}", n, addr);

                    if let Err(e) = tsw.flush().await {
                        eprintln!("Failed to flush for clientt {}: {}", addr, e);
                    }
                }
                Err(e) => {
                    eprintln!("Error for client {}: {}", addr, e);
                }
            }

            if let Err(e) = tsw.shutdown().await {
                eprintln!("Failed to shutdown for client {}: {}", addr, e);
            }

            println!("Client {} connection closed.", addr);
        });
    }
}
