use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket_addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 7000);
    let udp_server = UdpSocket::bind(socket_addr)?;

    let mut buf = [0; 1024];

    loop {
        match udp_server.recv_from(&mut buf) {
            Ok((usize, addr)) => {
                println!(
                    "from: {} received {} bytes - {}",
                    addr,
                    usize,
                    String::from_utf8_lossy(&buf)
                );

                match udp_server.send_to(&buf[..usize], addr) {
                    Ok(us) => println!("成功回写数据到UDP客户端{}个字节数", us),
                    Err(e) => eprintln!("{e}"),
                }
            }
            Err(e) => {
                eprintln!("{e}");
            }
        }
    }
}
