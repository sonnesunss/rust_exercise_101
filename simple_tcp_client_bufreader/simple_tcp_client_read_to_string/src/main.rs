use std::io::{BufRead, BufReader, Read};
use std::net::{TcpListener, TcpStream};

fn main() {
    let client = TcpStream::connect("127.0.0.1:9090").expect("Could't connect this server");
    let mut buffer = String::new();
    let mut buf_reader = BufReader::new(&client);

    let readc = buf_reader.read_to_string(&mut buffer);

    println!("read chars count -> {}", readc.unwrap());
    println!("read content -> {}", buffer);

    println!("Hello, world!");
}
