fn main() {
    let array1 = [5u8, 2, 0, 2, 0, 0, 0, 0, 0, 0];

    let r = extract_payload(&array1[..]);

    println!("{:?}", r);
}

fn extract_payload(buf: &[u8]) -> Option<&[u8]> {
    if buf.len() < 2 {
        return None;
    }

    let start_idx = 2;
    let len = buf[1] as usize;
    let end_idx = start_idx + len;

    if buf.len() >= end_idx {
        Some(&buf[start_idx..end_idx])
    } else {
        None
    }
}
