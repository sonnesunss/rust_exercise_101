use std::{
    io::Read,
    net::{Ipv4Addr, SocketAddrV4, TcpListener},
};

// discard协议，从客户端读取的数据会被丢弃然后理解关闭连接
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket_addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 9000);

    let server = TcpListener::bind(socket_addr)?;

    match server.accept() {
        Ok((mut tcp_stream, addr)) => {
            println!("new connection is coming from {:?}", addr);
            let mut buf: [u8; 5] = [0; 5];

            loop {
                match tcp_stream.read(&mut buf) {
                    Ok(0) => {
                        println!("客户端断开连接");
                        break;
                    }
                    Ok(n) => {
                        println!("从客户端读入了{}个字节的数据, 但是读了就丢弃，不管不顾!", n);
                    }
                    Err(e) => {
                        eprintln!("{e}");
                        break;
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("{e}");
        }
    }

    Ok(())
}
