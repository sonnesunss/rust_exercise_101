use mini_redis::{Connection, Frame, blocking_client::connect};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    let (socket, _addr) = listener.accept().await.unwrap();

    let mut connection = Connection::new(socket);

    if let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);

        let response = Frame::Error("unimplemented!".to_string());
        connection.write_frame(&response).await.unwrap();
    }
}
