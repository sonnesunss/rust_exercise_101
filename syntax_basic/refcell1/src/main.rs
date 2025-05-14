/*
    内部可变性支持

    rust 的借用规则有:

    1. 一个值在同一时空只允许同时拥有多个不可变引用
    2. 只有一个可变引用

    在编译时借用检查器会对这点进行检查， 如果不满足则报错拒绝继续编译

    refcell智能指针则提供了一个在运行时可以某个值的可变引用，进而通过这个可变引用修改它的能力
    即使那个值本身是immutable的， 它将借用检查从编译器推迟到了运行期，这点需要注意，借用检查
    没有消失， 如果一个值在一个时空内有不可变引用时通过refcell获取了可变引用会panic

*/

use std::cell::RefCell;

fn main() {
    let data = RefCell::new(10); // create a refcell containning 10

    // borrow mutably to modify the value
    {
        let mut value = data.borrow_mut();
        *value += 5;
    }

    // borrow immutably to read the value
    let value = data.borrow();
    println!("value in refcell: {}", value);
}
