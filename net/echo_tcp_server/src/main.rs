use std::io::{Read, Write};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};

/*
    echo server不同于基于ICMP协议实现的echo程序， echo规范于rfc862文件中
    使用tcp传输的情况下，接收到用户传递的数据后会原样将其返回，然后直到
    用户主动关闭连接，不然连接会存在
*/
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket_addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 7000);
    let tcp_listener = TcpListener::bind(socket_addr)?;

    loop {
        match tcp_listener.accept() {
            Ok((mut tcp_stream, _addr)) => {
                let mut buf = [0; 1024];

                match tcp_stream.read(&mut buf) {
                    Ok(0) => {
                        println!("客户端断开了连接，退出");
                        continue;
                    }
                    Ok(n) => {
                        println!("成功读取了{}字节", n);

                        match tcp_stream.write_all(&buf[..n]) {
                            Ok(()) => {
                                println!("Echoed {} bytes", n);
                            }
                            Err(e) => {
                                eprintln!("failed to write to stream: {}", e);
                                continue;
                            }
                        }
                        if let Err(e) = tcp_stream.flush() {
                            eprintln!("Failed to flush stream: {}", e);
                            continue;
                        }
                    }
                    Err(e) => {
                        eprintln!("读取客户端数据时出错{}", e);
                    }
                }
            }
            Err(e) => eprintln!("{e}"),
        }
    }
}
