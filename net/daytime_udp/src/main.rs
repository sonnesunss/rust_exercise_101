use chrono::Utc;
use ctrlc;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};
use std::sync::mpsc::{TryRecvError, channel};
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = channel();
    ctrlc::set_handler(move || tx.send(()).expect("Could't send msg to channel"))
        .expect("could't set handler");

    let socket_addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 13000);
    let udp_socket = UdpSocket::bind(socket_addr)?;

    let mut buf = [0; 1024];

    let _ = udp_socket.set_nonblocking(true);

    loop {
        match rx.try_recv() {
            Ok(_) => break,
            Err(TryRecvError::Disconnected) => break,
            Err(TryRecvError::Empty) => {}
        }
        match udp_socket.recv_from(&mut buf) {
            Ok(r) => {
                let c = udp_socket.try_clone()?;
                thread::spawn(move || {
                    println!("{:?}", thread::current().id());
                    t_handle(r, &c);
                });
            }
            Err(ref e)
                if e.kind() == std::io::ErrorKind::WouldBlock
                    || e.kind() == std::io::ErrorKind::ResourceBusy => {}
            Err(e) => eprintln!("{}", e),
        }
    }

    Ok(())
}

#[allow(dead_code)]
fn t_handle(tu: (usize, SocketAddr), us: &UdpSocket) {
    let utc_now = Utc::now();

    let _size = us.send_to(utc_now.to_rfc2822().as_bytes(), tu.1);
}
