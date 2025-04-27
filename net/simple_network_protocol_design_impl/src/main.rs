use bytes::{BufMut, BytesMut};
use std::fs::File;
use std::io::Read;
use std::path::Path;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = "README.md";
    let mut buf: [u8; 1024] = [0; 1024];
    let mut file_descriptor = File::open(Path::new(file1))?;

    let ur = file_descriptor.read(&mut buf);
    match ur {
        Ok(_) => {
            let r = encode_fime_msg(file1, &buf);
            println!("{:?}", r);
        }
        Err(e) => eprint!("{}", e),
    }

    Ok(())
}
