use std::net::TcpListener;

fn main() {
    let server = TcpListener::bind("127.0.0.1:9090").unwrap();
    let mut count = 0;

    println!("Server started");
    for x in server.incoming() {
        count += 1;
        if count == 3 {
            break;
        }

        match x {
            Ok(c) => println!("Incoming client -> {:?} ", c.peer_addr()),
            Err(e) => eprintln!("error {}", e),
        }
    }

    println!("Hello, world!");
}
