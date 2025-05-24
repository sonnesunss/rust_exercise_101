use std::vec;

use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> io::Result<()> {
    let socket = TcpStream::connect("127.0.0.1:9000").await?;
    let (mut rd, mut wr) = io::split(socket);

    // 创建异步任务，向后台写入数据
    tokio::spawn(async move {
        wr.write(b"hello\r\n").await?;
        wr.write(b"world\r\n").await?;
        Ok::<(), io::Error>(())
    });

    let mut buf = vec![0; 100];

    loop {
        let n = rd.read(&mut buf).await?;

        if n == 0 {
            break;
        }

        println!("got {:?}", &buf[..n]);
        println!("got string msg {:?}", String::from_utf8_lossy(&buf[..n]));
    }

    Ok(())
}
