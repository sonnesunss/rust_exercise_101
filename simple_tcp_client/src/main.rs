use std::io::Read;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    println!("client started");
    let mut client = TcpStream::connect("127.0.0.1:9090").expect("Could't connect this server");
    let mut read_buffer: [u8; 1024] = [0; 1024];

    let r = client.read(&mut read_buffer)?;
    println!("{:?}", r);

    let s = String::from_utf8_lossy(&read_buffer);
    println!("{}", s);

    Ok(())
}
