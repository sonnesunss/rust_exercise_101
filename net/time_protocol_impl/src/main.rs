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

                let dur_seconds = get_rfc868_timestamp();
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
fn get_rfc868_timestamp() -> u32 {
    let now = Utc::now();
    let utc_epoch = Utc.with_ymd_and_hms(1900, 1, 1, 0, 0, 0).unwrap();

    let dur_seconds = now.signed_duration_since(utc_epoch).num_seconds();

    const RFC868_WRAPAROUND_TIMESTAMP: i64 = u32::MAX as i64; // 4294967295
    const WARN_THRESHOLD: i64 = RFC868_WRAPAROUND_TIMESTAMP - (86400 * 30); // 一个月前发出警告

    if dur_seconds >= WARN_THRESHOLD {
        eprintln!(
            "[警告] 当前时间已接近 RFC868 时间戳回绕极限（2036-02-07），dur_seconds = {}",
            dur_seconds
        );
    }

    if dur_seconds > u32::MAX as i64 {
        panic!("时间戳已超过 RFC868 能表示的最大值，服务应升级协议");
    }

    dur_seconds as u32
}
