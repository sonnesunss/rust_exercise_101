use std::char;

fn main() {
    (70..=92).into_iter().for_each(|x| {
        println!("{} - {:?} - {:?}", x, char::from_u32(x), x as u8 as char);
    });
}
