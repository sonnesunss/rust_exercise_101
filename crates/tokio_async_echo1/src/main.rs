use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:9000").await?;

    loop {
        let (mut tcp_stream, addr) = listener.accept().await?;
        tokio::spawn(async move {
            println!("addr: {:?} incoming...", addr);

            let _n = tcp_stream
                .write_all(b"Hello Client, i\'m echo server!")
                .await;
            let mut buf = vec![0; 1028];

            loop {
                match tcp_stream.read(&mut buf).await {
                    Ok(n) if n == 0 => {
                        println!("Client {:?} disconnected.", addr);
                        break;
                    }
                    Ok(n) => {
                        let received_data = &buf[..n];
                        println!(
                            "received_data from {:?}: {:?}",
                            addr,
                            String::from_utf8_lossy(&received_data)
                        );
                        break;
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                        break;
                    }
                }
            }
        });
    }
}
