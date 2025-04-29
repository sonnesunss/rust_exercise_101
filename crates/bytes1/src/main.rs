use bytes::{Buf, BufMut, Bytes, BytesMut};

fn main() {
    let mut bytes_buf = BytesMut::with_capacity(1024);

    bytes_buf.put_i32(0x930909);
    bytes_buf.put_i16(0x3131);

    println!("{:?}", bytes_buf);

    println!("Hello, world!");
}
