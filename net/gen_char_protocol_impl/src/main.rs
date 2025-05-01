use std::{
    char,
    io::{ErrorKind, Write},
    net::{Ipv4Addr, SocketAddrV4, TcpListener},
    time::Duration,
};

// 生成ASCII字符序列的开始和结束范围
fn gen_char(cur_line: usize) -> (u8, u8) {
    // ASCII 可打印字符从 32 到 126，共 95 个
    // 每行 72 个字符，从 (0+N mod 95) 到 (71+N mod 95)
    let start = ((cur_line % 95) as u8) + 32; // 起始字符
    let end = (((cur_line + 71) % 95) as u8) + 32; // 结束字符
    (start, end)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket_addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 9000);
    let server = TcpListener::bind(socket_addr)?;
    println!("Server listening on {}", socket_addr);

    // 处理多个连接
    for stream in server.incoming() {
        match stream {
            Ok(mut tcp_stream) => {
                println!("New connection: {:?}", tcp_stream.peer_addr());

                // 设置写超时，避免长时间阻塞
                tcp_stream.set_write_timeout(Some(Duration::from_secs(5)))?;

                let mut line = 0; // 每新连接从第0行开始

                // 持续发送数据直到客户端断开
                loop {
                    // 生成当前行的字符范围
                    let (start, end) = gen_char(line);
                    let mut buf = Vec::new();

                    // 处理字符范围，考虑环形序列
                    if start <= end {
                        for c in start..=end {
                            buf.push(c);
                        }
                    } else {
                        // 处理跨95模的环形情况
                        for c in start..=126 {
                            buf.push(c);
                        }
                        for c in 32..=end {
                            buf.push(c);
                        }
                    }

                    // 添加回车和换行
                    buf.push(b'\r');
                    buf.push(b'\n');

                    // 写入TCP流
                    match tcp_stream.write_all(&buf) {
                        Ok(_) => {
                            // 刷新流
                            if let Err(e) = tcp_stream.flush() {
                                eprintln!("Failed to flush stream: {}", e);
                                break; // 刷新失败，断开连接
                            }
                        }
                        Err(e)
                            if e.kind() == ErrorKind::BrokenPipe
                                || e.kind() == ErrorKind::ConnectionReset =>
                        {
                            println!("Client disconnected: {:?}", tcp_stream.peer_addr());
                            break; // 客户端断开连接
                        }
                        Err(e) => {
                            eprintln!("Failed to write to stream: {}", e);
                            break; // 其他错误，断开连接
                        }
                    }

                    // 增加行数
                    line = (line + 1) % 95;

                    // 添加短暂休眠以避免过快发送
                    std::thread::sleep(Duration::from_millis(100));
                }
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }

    Ok(())
}
