//  实现自己的Box智能指针
//  rust自带的几个智能指针均持有值所有权、管理生命周期、资源，具备指针行为
//
//  Box是一个允许在堆上分配内存空间的智能指针
//  我们要自己实现在堆上分配合适的空间，返回地址，解引用，管理资源释放、生命周期

// step1: 引入手动管理内存的mod
use std::{
    alloc::{Layout, alloc, dealloc},
    ops::{Deref, DerefMut},
    ptr,
};

// step2: 实现结构体实现智能指针，创建一个泛型结构体
#[allow(dead_code)]
#[derive(Debug)]
pub struct MyBox<T> {
    // 定义一个结构体字段，原始指针指向堆空间
    ptr: *mut T,
}

#[allow(dead_code)]
impl<T> MyBox<T> {
    // 遵循rust习惯，创建构造函数，然后在这个函数内分配堆内存空间并将值移动到堆上
    fn new(value: T) -> Self {
        // 我们并不知道泛型T的大小尺寸、布局，因此需要计算它的内存大小尺寸、布局这些信息
        let layout = Layout::new::<T>();
        let ptr = unsafe { alloc(layout) } as *mut T;

        // 检查是否分配成功
        if ptr.is_null() {
            panic!("Memory allocation failed!");
        }

        // 将值写入堆内存
        unsafe {
            ptr::write(ptr, value);
        }

        MyBox { ptr }
    }
}

// step3: 使其具备指针行为，通过实现Deref、DerefMut trait实现
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // *解引用运算符的优先级别要高于&取地址运算符，避免混乱最好加个清晰的括号表明这一点

        // 为什么需要这样？ self.ptr本身不就已经是一个地址了吗？ 解引用再取地址取得依然还是地址，为什么需要重复？
        //
        // 这与安全rust与非安全rust之间的界限有关系
        // 我们实现的自己的Box智能指针是属于safe rust的范畴，而在堆内存空间分配内存空间在unsafe rust的范畴内
        // 先将self.ptr解引用是取得堆上的数据，然后得到数据本身后，使用&取地址运算符后得到一个safe rust范畴内的不可变引用&T

        // 那么，为什么&运算符的操作是安全的？
        //
        // 使用&取地址运算符获得的不可变引用&T是安全的，因为&T本身是一个受Rust借用检查器约束的操作，它会确保生成的引用符合Rust的内存安全规则
        // 例如 生命周期和借用规则
        unsafe { &(*self.ptr) }
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.ptr }
    }
}

// step4: 管理资源释放， 通过实现Drop trait实现
impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        let layout = Layout::new::<T>();
        unsafe {
            ptr::drop_in_place(self.ptr);
            dealloc(self.ptr as *mut u8, layout);
        }
    }
}

fn main() {
    let box1 = MyBox::new(123);
    let box2 = MyBox::new("Hello MyBox".to_string());
    let box3 = MyBox::new(1.123);

    println!("{}", *box1);
    println!("{}", *box2);
    println!("{:?}", box3);
}
