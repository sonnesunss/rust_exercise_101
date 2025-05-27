use std::fs::read;
use std::sync::Arc;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{Mutex, mpsc};
use tokio::time::Duration;
use tokio::time::{Instant, timeout};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 在正常收发来自tcp的客户端的消息的前提下实现心跳检测机制
    // 心跳检测机制，定期向客户端发送一个特定的消息，或者客户端在一定时间间隔内需要往服务器发送过数据
    // 的机制，特定的或者寻常的，如果超出某个时间范围则判定已经无须为客户端维持连接， 主动断开
    // 通常心跳机制应该是自动的，而无需tcp客户端用户手动输入
    // 例如服务器主动发送心跳检测包，然后客户端收到，则在规定时间内发回一个数据
    // 还有双向心跳

    // 两种方式适应的场景不同， 在需要服务器主导的场景中，服务器发送心跳包更常见更适合，例如游戏服务器这种需要高可靠、高性能的场景
    // dropbox这种则可能使用客户端心跳主动向服务器报告状态
    // websocket则是双向心跳的
    // 无一例外，它们都是希望能够维持长连接，快速清理无须维持的客户端连接的场景下

    // 例子使用一个客户端心跳演示
    let server_addr = "127.0.0.1:9000";

    let server = TcpListener::bind(server_addr).await?;
    loop {
        let (socket, addr) = server.accept().await?;

        tokio::spawn(handle_client(socket, addr.to_string()));
    }
}

async fn handle_client(stream: TcpStream, addr: String) {
    println!("[{}] is coming, {:?}", addr, std::thread::current().id());
    let (tx, mut rx) = mpsc::unbounded_channel::<()>();
    let (reader, writer) = stream.into_split();
    let mut reader = BufReader::new(reader).lines();
    let writer = Arc::new(Mutex::new(writer)); // 用在读取写入tcp stream
    let writer_c = writer.clone(); // 用在心跳检测

    // heartbeat
    // 需要几个装置：
    // 1. 获取当前时间
    // 2. 一个超时时间
    // 3. 计算两个时间之间的距离，然后根据距离判断走向
    // 4. 超时分支
    let heartbeat_task = tokio::spawn(async move {
        loop {
            let duration_deadline = Instant::now() + Duration::from_secs(10);
            if timeout(duration_deadline.duration_since(Instant::now()), rx.recv())
                .await
                .is_err()
            {
                eprintln!("超时未收到客户端发来的消息，将断开连接");
                let _ = writer_c.lock().await.shutdown().await;
                return;
            }
        }
    });

    while let Ok(Some(line)) = reader.next_line().await {
        if line.trim() == "PING" {
            println!("heartbeat received!");
            let _ = tx.send(());
        } else {
            println!("normal msg received! {:?}", line);
            let mut writer_lock = writer.lock().await;
            if let Err(e) = writer_lock.write_all(b"ACK\n").await {
                println!("write error {:?}", e);
                break;
            }
        }
    }

    println!("Client disconnected");
    heartbeat_task.abort();
}
