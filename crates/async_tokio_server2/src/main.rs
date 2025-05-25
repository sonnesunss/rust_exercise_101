use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

fn main() {
    /*
       使用长度前缀消息framing读取tcp server发送来的消息
    接收逻辑，需要双方有一个约定，先发送实体大小，然后再发送实体，这依靠的是tcp的可靠
    传输特性设计的
    let mut len_bytes = [0; 4];

    match reader.read_exact(&mut len_bytes).await {
        Ok(0) => {
            println!("Server closed the connection.");
            break;
        }
        Ok(_) => {
            let len = u32::from_be_bytes(len_bytes) as usize;
            let mut message_buffer = vec![0; len]; // allocate dynamically
            match reader.read_exact(&mut message_buffer).await {
                Ok(0) => {
                    println!("Server closed connection while reading message body.");
                    break;
                }
                Ok(_) => {
                    println!(
                        "Client received: {:?}",
                        String::from_utf8_lossy(&message_buffer)
                    );
                }
                Err(e) => {
                    eprintln!("Error reading message body: {}", e);
                    break;
                }
            }
        }
    }

    // 发送的逻辑
    // 先获取消息长度，然后发送消息长度，再发送消息实体
    let msg1 = b"dsadsa";
    let msg2 = b"gjghjhotwe";

    let len1 = msg1.len() as u32;
    if let Err(e) = writer.write_all(&len1.to_be_bytes()).await {
        eprintln!("Error sending length for msg1: {}", e);
        return Ok(());
    }

    if let Err(e) = writer.write_all(msg1).await {
        eprintln!("Error sending msg1: {}", e);
        return Ok(());
    }

    println!("Clien sent msg1 ({} bytes)", msg1.len());
    */
}
