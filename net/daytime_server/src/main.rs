use chrono::Utc;
use std::io::{self, Write};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
use std::sync::mpsc::channel;
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = channel();
    ctrlc::set_handler(move || tx.send(()).expect("Cloud't setting ctrlc handler"))
        .expect("Could't send msg from channel");
    println!("Press Ctrl-C quit app");

    let socket_addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 13000);
    let server = TcpListener::bind(socket_addr)?;
    server.set_nonblocking(true)?;

    loop {
        match rx.try_recv() {
            Ok(_) => {
                break;
            }
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                break;
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => {}
        }

        match server.accept() {
            Ok((tcp_stream, _addr)) => {
                thread::spawn(move || {
                    t_handle(tcp_stream);
                });
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                eprintln!("{}", e)
            }
        }
    }

    Ok(())
}

#[allow(dead_code)]
fn t_handle(mut stream: TcpStream) {
    println!("current thread id -> {:?}", thread::current().id());
    let utc_now = Utc::now();
    let wus = stream.write(utc_now.to_rfc2822().as_bytes());

    match wus {
        Ok(s) => println!("写入成功字节数: {}", s),
        Err(e) => println!("写入错误: {}", e),
    }

    // 显式shutdown， 网络编程中最好进行显式资源释放
    if let Err(e) = stream.shutdown(std::net::Shutdown::Both) {
        eprintln!("关闭连接出错: {}", e);
    }
}
