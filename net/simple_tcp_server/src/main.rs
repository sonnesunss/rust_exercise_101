use std::io::Write;
use std::net::TcpListener;

fn main() {
    let server = TcpListener::bind("127.0.0.1:9090").unwrap();
    let mut count = 0;

    println!("Server started");
    for stream in server.incoming() {
        count += 1;
        if count == 3 {
            break;
        }

        match stream {
            Ok(mut c) => {
                println!("Incoming client -> {:?} ", c.peer_addr());
                let r = c.write(b"Hello my friend, thank you for your trust my services");
                println!("{:?}", r);
            }
            Err(e) => eprintln!("error {}", e),
        }
    }

    println!("Hello, world!");
}
