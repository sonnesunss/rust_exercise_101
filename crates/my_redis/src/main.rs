use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        // The second item contains the IP and port of the new connection.
        let (socket, _) = listener.accept().await.unwrap();
        //  每进来一个请求，会await在这里停留等待process函数的执行结束
        //  创建一个task任务，使得可以异步处理这个任务
        tokio::spawn(async move {
            process(socket).await;
        });
    }
}

async fn process(socket: TcpStream) {
    use mini_redis::Command::{self, Get, Set};
    use std::collections::HashMap;

    // a HashMap iis used to store data
    let mut db = HashMap::new();

    // The `Connection` lets us read/write redis **frames** instead of
    // byte streams. The `Connection` type is defined by mini-redis.
    let mut connection = Connection::new(socket);

    // 持续从接收缓冲区读取frame
    while let Some(frame) = connection.read_frame().await.unwrap() {
        // 从读取到的frame解析出具体的命令
        let response = match Command::from_frame(frame).unwrap() {
            // 如果是set命令，就从中取出k-v并存储之
            Set(cmd) => {
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            }
            // 如果是get命令，则取出k，然后从db中根据key取出value，组合成Frame::Bluk返回
            Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }

            // 未实现的命令
            cmd => panic!("unimplemented! {:?}", cmd),
        };

        // 写回frame到底层的TcpStream中
        connection.write_frame(&response).await.unwrap();
    }
}
