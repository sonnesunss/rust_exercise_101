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

    let writer = Arc::new(Mutex::new(writer)); // 可共享写端, 因为writer经过into_split方法返回的类型没有实现Clone trait，导致不能clone，而后面又需要多处使用
    let writer_for_heartbeat = Arc::clone(&writer);

    let addr = Arc::new(Mutex::new(addr));
    let addrc = addr.clone();

    let (tx, mut rx) = mpsc::unbounded_channel::<()>();

    //  心跳监测任务
    let timeout_handle = tokio::spawn(async move {
        loop {
            let deadline = Instant::now() + READ_TIMEOUT;
            // timeout会等待两个参数谁先完成，如果是参数以的计时器先完成，那么会返回一个超时错误，
            // is_err是一个Option方法，用于检查是否产生超时错误
            // 如果rx.recv（）先完成，在下面代码逻辑中是跳过if body继续往下执行的
            // duration_since是一个Instant方法， 用于计算两个给定的时间间隔间隔的值，类型是Duration，如果返回的是正数，则表示还有多少时间可以等待
            if timeout(deadline.duration_since(Instant::now()), rx.recv())
                .await
                .is_err()
            {
                println!("[{:?}] Heartbeat timeout. Closing connection.", addrc);
                //  安全地锁定并关闭写端
                let _ = writer_for_heartbeat.lock().await.shutdown().await;
                return;
            }
        }
    });

    // 主循环读取来自客户端的消息
    while let Ok(Some(line)) = reader.next_line().await {
        if line.trim() == "ping" {
            println!("[{:?}] Heartbeat received", addr.lock().await);
            let _ = tx.send(()); // 重置心跳定时器, 向接收端发送一个消息，让等大的rx先完成，这样timeout中，rx就是先执行的，会理解进入下一个loop， 再重新检查持续时间，相当于重置了计时器
        } else {
            println!("[{:?}] Received: {}", addr.lock().await, line);
            let mut writer_lock = writer.lock().await; // 使用 writer 前加锁
            if let Err(e) = writer_lock.write_all(b"ACK\n").await {
                println!("[{:?}] Write error: {:?}", addr.lock().await, e);
                break;
            }

            let _ = tx.send(());
        }
    }

    println!("[{:?}] Client disconnected", addr.lock().await);
    timeout_handle.abort(); // 清理心跳任务
}
