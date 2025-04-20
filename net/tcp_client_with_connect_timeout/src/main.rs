use std::net::SocketAddr;
use std::net::TcpStream;
use std::time::Duration;

// 连接到127.0.0.1:8080，设置5秒连接超时
fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:8080".parse::<SocketAddr>();

    match addr {
        Ok(addr) => {
            let stream = TcpStream::connect_timeout(&addr, Duration::from_secs(50))?;
            println!("{:?}", stream.peer_addr());
        }

        Err(e) => {
            eprintln!("{}", e);
        }
    }
    Ok(())
}
