use std::io::Read;
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket_addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 9000);
    let server = TcpListener::bind(socket_addr)?;

    loop {
        match server.accept() {
            Ok((tcp_stream, socket_addr)) => {
                thread::spawn(move || {
                    println!(
                        "I'm thread-{:?} -> new connection: {:?}",
                        thread::current().id(),
                        socket_addr
                    );
                    handle(tcp_stream);
                });
            }
            Err(e) => eprintln!("{}", e),
        }
    }
}

fn handle(mut stream: TcpStream) {
    let mut buffer: [u8; 1024] = [0; 1024];

    let _read_usize = stream.read(&mut buffer);

    if _read_usize.unwrap_or(0) > 0 {
        println!("read content -> {}", String::from_utf8_lossy(&buffer));
    }
}
