use std::net::UdpSocket;

// UDP 客户端
fn main() {
    let buffer = "Hi, I'm UDP Client".as_bytes();
    let udp_client = UdpSocket::bind("127.0.0.1:54322").expect("Could't binding to 127.0.0.1");
    udp_client
        .connect("127.0.0.1:54321")
        .expect("Could't connect to 127.0.0.1");
    let size_bytes = udp_client.send(&buffer);

    println!("send to 127.0.0.1:54321 size bytes -> {:?}", size_bytes);
}
