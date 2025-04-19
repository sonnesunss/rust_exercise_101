use std::io::Result;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::thread;

// 能够处理多个tcp客户端连接的能力，并给每个tcp连接写入消息
fn main() -> Result<()> {
    println!("TCP server listening on 127.0.0.1:9090");
    let server =
        TcpListener::bind("127.0.0.1:9090").expect("Could't binding this addr pls restart");

    server.incoming().for_each(|es| {
        match es {
            // 这里处理每一个tcp连接
            Ok(tc) => {
                println!("有新TCP连接进入，正在处理中...{:?}", tc.peer_addr());
                thread::spawn(|| {
                    println!("I'm thread-{:?}, running", thread::current().id());
                    if let Err(e) = handle_client(tc) {
                        eprintln!("Error {}", e);
                    }
                });
            }
            Err(e) => eprintln!("{}", e),
        }
    });

    Ok(())
}

fn handle_client(mut stream: TcpStream) -> Result<()> {
    let _ =
        stream.write_all(b"Hello TCP client, I'm TCP server, nice to meet u & have a nice day")?;
    stream.flush()?;

    Ok(())
}
