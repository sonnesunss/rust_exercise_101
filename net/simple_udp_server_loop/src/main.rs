use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    println!("UDP Client started");

    let msg = "Hello udp, from another udp\n";
    let udp = UdpSocket::bind("127.0.0.1:54321").expect("Could't binding this socket addr");
    let mut buffer = [0; 100];
    let mut response_count = 0;

    loop {
        if response_count == 10 {
            break;
        }
        response_count += 1;

        let (size_bytes, from_addr) = udp.recv_from(&mut buffer)?;
        let received = String::from_utf8_lossy(&buffer);
        println!(
            "Received from {} - {} size_bytes is {}",
            from_addr, received, size_bytes
        );

        udp.send_to(msg.as_bytes(), from_addr)?;
        println!("Sent response to {}", from_addr);
    }

    println!("bye - bye");

    Ok(())
}
