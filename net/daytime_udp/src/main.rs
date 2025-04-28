use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket_addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 13000);
    let udp_socket = UdpSocket::bind(socket_addr)?;

    let mut buf = [0; 1024];

    let (number_of_bytes, src_addr) = udp_socket.recv_from(&mut buf)?;

    let filled_buf = &mut buf[..number_of_bytes];
    println!("{:?}", src_addr);

    udp_socket.send_to(&filled_buf, src_addr)?;

    println!("{:?}", String::from_utf8_lossy(filled_buf));

    Ok(())
}
