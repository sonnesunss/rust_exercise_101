use tokio::io::{self, AsyncReadExt};

use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut client = TcpStream::connect("127.0.0.1:9000").await?;
    let mut len_bytes = [0; 4];

    match client.read_exact(&mut len_bytes).await {
        Ok(0) => {
            println!("server disconnection");
        }
        Ok(n) => {
            // 成功读取到长度，然后再继续读取后续实际内容
            println!("读取到的长度 bytes: {}", n);
            let len1 = u32::from_be_bytes(len_bytes) as usize;
            println!("内容长度是: {}", len1);
            let mut buf = vec![0; len1];

            client.read_exact(&mut buf).await?;
            println!("content is -> {:?}", String::from_utf8_lossy(&buf));
        }
        Err(e) => {
            eprintln!("{e}");
        }
    }

    Ok(())
}
