use chrono::Utc;
use ctrlc;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};
use std::sync::mpsc::{self, TryRecvError, channel};
use std::time::Duration;
use std::{thread, usize};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ctrlc channel
    let (tx, rx) = channel();
    ctrlc::set_handler(move || tx.send(()).expect("Could't send msg to channel"))
        .expect("could't set handler");

    // child thread channel
    let (err_tx, err_rx) = mpsc::channel();

    let socket_addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 13000);
    let udp_socket = UdpSocket::bind(socket_addr)?;

    let mut buf = [0; 1024];

    let _ = udp_socket.set_nonblocking(true);

    loop {
        // 检查Ctrlc
        match rx.try_recv() {
            Ok(_) => break,
            Err(TryRecvError::Disconnected) => break,
            Err(TryRecvError::Empty) => {}
        }

        // 检查子线程错误
        match err_rx.try_recv() {
            Ok(err_msg) => eprintln!("子线程遇到错误: {:?}", err_msg),
            Err(TryRecvError::Disconnected) => {
                eprintln!("错误通道断开了");
                break;
            }
            Err(TryRecvError::Empty) => {}
        }

        // 处理UDP包
        match udp_socket.recv_from(&mut buf) {
            Ok(r) => {
                let c = udp_socket.try_clone()?;
                let etxc = err_tx.clone();

                thread::spawn(move || {
                    println!("{:?}", thread::current().id());
                    if let Err(e) = t_handle(r, &c) {
                        let _ = etxc.send(format!("数据发送失败: {}", e));
                    }
                });
            }
            Err(ref e)
                if e.kind() == std::io::ErrorKind::WouldBlock
                    || e.kind() == std::io::ErrorKind::ResourceBusy => {}
            Err(e) => eprintln!("{}", e),
        }
        // 尽管是非阻塞的，但是轮训太频繁导致cpu占用会很高，稍微歇会儿吧
        thread::sleep(Duration::from_millis(5));
    }

    Ok(())
}

#[allow(dead_code)]
fn t_handle(tu: (usize, SocketAddr), us: &UdpSocket) -> std::io::Result<()> {
    let utc_now = Utc::now();

    let _size = us.send_to(utc_now.to_rfc2822().as_bytes(), tu.1)?;

    Ok(())
}
