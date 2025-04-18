use std::io::Read;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    println!("client started");
    let mut client = TcpStream::connect("127.0.0.1:9090").expect("Could't connect this server");
    let mut read_buffer: Vec<u8> = Vec::new();

    let r = client.read_to_end(&mut read_buffer)?;
    println!("{:?}", r);

    let s = String::from_utf8_lossy(&read_buffer);
    println!("{}", s);

    Ok(())
}
