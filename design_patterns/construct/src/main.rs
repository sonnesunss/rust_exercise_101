/*
    Rust构造器

    Rust没有构造器作为语言构造.

    惯例是使用一个关联函数new来创建一个对象.

*/

#[allow(dead_code)]
#[derive(Debug)]
pub struct Second {
    value: u64,
}

impl Second {
    // Constructs a new instance of ['Second']
    // Note this is an associated function - no self.
    // 关联方法
    pub fn new(value: u64) -> Self {
        Self { value }
    }

    // 实例方法
    // return the value in seconds
    pub fn value(&self) -> u64 {
        self.value
    }
}

// 仅仅只是一个演示，不建议同时为一个类型提供new函数以及实现default trait
// 默认构造器
impl Default for Second {
    fn default() -> Self {
        Self { value: 10 }
    }
}

// 使用派生宏的default
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct Minutes {
    value: u64,
}

fn main() {
    let s1 = Second::new(120);
    println!("s1: {:?}", s1);

    let s2 = Second::default();
    println!("s2: {:?}", s2);

    let m1 = Minutes::default();
    println!("m1: {:?}", m1);
}
