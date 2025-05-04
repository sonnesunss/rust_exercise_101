use std::{
    io::Write,
    net::{Ipv4Addr, SocketAddrV4, TcpListener},
};

use chrono::{TimeZone, Utc};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket_addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 9000);

    let server = TcpListener::bind(socket_addr)?;

    loop {
        match server.accept() {
            Ok((mut socket_addr, addr)) => {
                println!("New connection is coming from {addr}");

                let dur_seconds = get_rfc868_timestamp() as u32;
                println!("{}", dur_seconds);

                socket_addr.write(&dur_seconds.to_be_bytes())?;
            }
            Err(e) => {
                eprintln!("{e}");
            }
        }
    }
}

#[allow(dead_code)]
fn get_rfc868_timestamp() -> i64 {
    let now = Utc::now();
    let utc_epoch = Utc.with_ymd_and_hms(1900, 1, 1, 0, 0, 0).unwrap();

    let dur_seconds = now.signed_duration_since(utc_epoch).num_seconds();

    dur_seconds
}
