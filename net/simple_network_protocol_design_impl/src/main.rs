use bytes::{BufMut, BytesMut};
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};
use std::path::Path;

// 准备并编码协议需要的内容
#[allow(dead_code)]
fn encode_fime_msg(file_name: &str, file_content: &[u8]) -> BytesMut {
    // 准备协议需要的内容
    let magic_num: u32 = 0x930909;
    let ver_num: u16 = 1;
    let filename_bytes = file_name.as_bytes();
    let filename_len = filename_bytes.len() as u16;
    let file_content_size = file_content.len() as u32;

    let mut buf =
        BytesMut::with_capacity(4 + 2 + 2 + filename_len as usize + 4 + file_content_size as usize);

    // 多字节数据类型数据需要转换成网络字节序
    buf.put_u32(magic_num.to_be());
    buf.put_u16(ver_num.to_be());
    buf.put_u16(filename_len.to_be());

    // 文件名是字节流， 也就是字节数组，所以保持原样即可
    buf.extend_from_slice(filename_bytes);

    // 文件内容大小, 用以跟踪文件真实内容数据长度
    buf.put_u32(file_content_size.to_be());
    buf.extend_from_slice(file_content);

    buf
}

// 读取整个文件到一个恰好的vec结构内，避免过度或者不够的内存分配
#[allow(dead_code)]
fn read_whole_file(file_name: &str) -> std::io::Result<Vec<u8>> {
    let mut fd = File::open(Path::new(file_name))?;
    let file_meta = fd.metadata()?;
    let file_size = file_meta.len() as usize;
    let mut buf = Vec::with_capacity(file_size);

    let _ru = fd.read_to_end(&mut buf)?;

    Ok(buf)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = "README.md";
    let buf = read_whole_file(file1)?;

    let r: BytesMut = encode_fime_msg(file1, &buf);
    println!("{:?}", r);

    // tcp listener
    let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9000);
    let listener = TcpListener::bind(socket_addr)?;
    //  accept loop
    loop {
        match listener.accept() {
            Ok((mut tcp_stream, _addr)) => {
                // 暂时忽略掉返回的写入成功的字节数目
                let _ = tcp_stream.write(&r);
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }
}
