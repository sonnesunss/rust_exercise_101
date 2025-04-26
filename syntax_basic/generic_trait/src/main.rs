// Generic Trait

/*
    假设有一个FromI32的Trait， 所有实现了该trait的类型都可以从i32类型转型
*/
#[allow(dead_code)]
pub trait FromI32 {
    // 可以看到这里的函数签名参数是需要一个i32类型的，就是说只允许实现了该trait的类型从i32类型转型到自身
    // 如果还希望能够从u8,u32,u64, i64等类型皆可以转型，那么就需要定义多个参数不同其他部分相同的trait
    fn from1(val: i32) -> Self;
}

// 例如，定义u8, u32，i64, u64
#[allow(dead_code)]
pub trait FromU8 {
    fn from1(val: u8) -> Self;
}

#[allow(dead_code)]
pub trait FromU32 {
    fn from1(val: u32) -> Self;
}

#[allow(dead_code)]
pub trait FromU64 {
    fn from1(val: u64) -> Self;
}

#[allow(dead_code)]
pub trait FromI64 {
    fn from1(val: i64) -> Self;
}

#[allow(dead_code)]
#[derive(Debug)]
struct MyType1 {
    n1: &'static str,
    a1: i32,
}

impl FromI32 for MyType1 {
    fn from1(val: i32) -> Self {
        MyType1 {
            n1: "GaGa",
            a1: val,
        }
    }
}

// 然后我们就需要为我们的MyType1类型分别实现这些trait
// 可以看到有很多的模板代码的， 如下
impl FromU8 for MyType1 {
    fn from1(val: u8) -> Self {
        MyType1 {
            n1: "GaGa",
            a1: val as i32,
        }
    }
}

fn main() {
    // TODO
    let i1: i32 = 123;
    let mt1 = <MyType1 as FromI32>::from1(i1);

    let u1: u8 = 255;
    let mt2 = <MyType1 as FromU8>::from1(u1);

    println!("{:?}", mt1);
    println!("{:?}", mt2);
}
