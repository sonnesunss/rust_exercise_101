use std::io::BufRead;
use std::io::BufReader;
use std::net::TcpStream;

fn main() {
    let client_stream = TcpStream::connect("127.0.0.1:9090").unwrap();
    let mut buffer = String::new();
    let mut bufReader = BufReader::new(&client_stream);
    let c = bufReader.read_line(&mut buffer);
    println!("{:?}", c);
    println!("{:?}", buffer);
}
