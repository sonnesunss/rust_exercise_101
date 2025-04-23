use std::io::Read;
use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::net::SocketAddrV4;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listen_addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 9000);
    let server = TcpListener::bind(listen_addr)?;

    loop {
        let stream = server.accept()?;
        thread::spawn(|| {
            handle(stream);
            println!("{:?}", thread::current().id());
        });
    }
}

fn handle(mut tcp_accept_result: (TcpStream, SocketAddr)) {
    let mut buffer: [u8; 1024] = [0; 1024];

    println!("new connection {:?}", tcp_accept_result.1);

    let _rsize = tcp_accept_result.0.read(&mut buffer);

    if _rsize.unwrap_or(0) > 0 {
        println!("{}", String::from_utf8_lossy(&buffer));
    }
}
