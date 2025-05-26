use std::sync::Arc;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    sync::{Mutex, mpsc},
    time::{Duration, Instant, timeout},
};

const READ_TIMEOUT: Duration = Duration::from_secs(10);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on 127.0.0.1:8080");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("Client connected: {}", addr);
        tokio::spawn(handle_client(socket, addr.to_string()));
    }
}

async fn handle_client(socket: TcpStream, addr: String) {
    let (reader, writer) = socket.into_split();
    let mut reader = BufReader::new(reader).lines();

    let writer = Arc::new(Mutex::new(writer)); // 可共享写端, 因为writer经过into_split方法返回的类型没有实现Clone trait，导致不能clone，而后面又需要
    let writer_for_heartbeat = Arc::clone(&writer);

    let (tx, mut rx) = mpsc::unbounded_channel::<()>();

    //  心跳监测任务
    let timeout_handle = tokio::spawn(async move {
        loop {
            let deadline = Instant::now() + READ_TIMEOUT;
            if timeout(deadline.duration_since(Instant::now()), rx.recv())
                .await
                .is_err()
            {
                println!("Heartbeat timeout. Closing connection.");
                //  安全地锁定并关闭写端
                let _ = writer_for_heartbeat.lock().await.shutdown().await;
                return;
            }
        }
    });

    // 主循环读取来自客户端的消息
    while let Ok(Some(line)) = reader.next_line().await {
        if line.trim() == "ping" {
            println!("Heartbeat received");
            let _ = tx.send(()); // 重置心跳定时器
        } else {
            println!("Received: {}", line);
            let mut writer_lock = writer.lock().await; // 使用 writer 前加锁
            if let Err(e) = writer_lock.write_all(b"ACK\n").await {
                println!("Write error: {:?}", e);
                break;
            }
        }
    }

    println!("[{}] Client disconnected", addr);
    timeout_handle.abort(); // 清理心跳任务
}
