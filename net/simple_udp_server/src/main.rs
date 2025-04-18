use std::io::Result;
use std::net::SocketAddr;
use std::net::UdpSocket;

fn main() -> Result<()> {
    println!("Hello, UDP Server!");

    let server = UdpSocket::bind("127.0.0.1:54321")?;
    let mut buffer: [u8; 100] = [0; 100];
    // udp是无连接的，因此需要知道希望与哪个SocketAddr通信
    // 一个方法是接收传递来的信息的时候记录发送者地址
    let (size_bytes, from_socket_addr) = server.recv_from(&mut buffer)?;
    let s = String::from_utf8_lossy(&buffer);

    println!("recv data is {}", s);
    println!("recv data from addr {:?}", from_socket_addr);
    println!("recv data size bytes {:?}", size_bytes);

    server.send_to(b"hello udp client", from_socket_addr)?;

    Ok(())
}
