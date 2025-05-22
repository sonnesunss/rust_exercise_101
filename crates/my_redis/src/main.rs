use bytes::Bytes;
use mini_redis::Command::{self, Get, Set};
use mini_redis::{Connection, Frame};
use pretty_printer::pretty_print;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};

type DB = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    // a HashMap is used to store data
    // 如果希望db可以在多个线程之间能够共享使用，按照rust的属于就是要使得这个db允许共享所有权
    // 显然可以通过Arc智能指针实现
    let db: DB = Arc::new(Mutex::new(HashMap::new()));

    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let dbc = db.clone();
        // The second item contains the IP and port of the new connection.
        let (socket, _) = listener.accept().await.unwrap();
        //  每进来一个请求，会await在这里停留等待process函数的执行结束
        //  创建一个task任务，使得可以异步处理这个任务
        tokio::spawn(async move {
            process(socket, dbc).await;
        });
    }
}

async fn process(socket: TcpStream, db: DB) {
    // The `Connection` lets us read/write redis **frames** instead of
    // byte streams. The `Connection` type is defined by mini-redis.
    let mut connection = Connection::new(socket);

    // 持续从接收缓冲区读取frame
    while let Some(frame) = connection.read_frame().await.unwrap() {
        // 从读取到的frame解析出具体的命令
        let response = match Command::from_frame(frame).unwrap() {
            // 如果是set命令，就从中取出k-v并存储之
            Set(cmd) => {
                db.lock()
                    .unwrap()
                    .insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            // 如果是get命令，则取出k，然后从db中根据key取出value，组合成Frame::Bluk返回
            Get(cmd) => {
                if let Some(value) = db.lock().unwrap().get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }

            // 未实现的命令
            cmd => panic!("unimplemented! {:?}", cmd),
        };

        // 写回frame到底层的TcpStream中
        connection.write_frame(&response).await.unwrap();

        pretty_print!(db);
    }
}
