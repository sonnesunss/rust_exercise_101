// 还可以使用这个社区库来实现
use num;

// num crate提供了一个Num trait， 其几乎为所有的数字类型都实现了这个trait， 我们可以使用这个约束参数
fn my_add3<T>(one: T, two: T) -> T
where
    T: num::Num,
{
    one + two
}

// 实现一个泛型方法，其只允许参数是数字类型的参数，也就是说要加泛型约束
#[allow(dead_code)]
fn my_add<T: std::ops::Add<Output = T>>(one: T, two: T) -> T {
    one + two
}

// 上面的实现有一个问题，只要实现了Add trait的类型就可以
// 如果只希望在有限的几个类型上面实现加法可以这样做，如下

// 1. 定义一个trait
pub trait CustomAddTrait {}

// 2. 为某些希望的类型实现这个自定义的trait
impl CustomAddTrait for i32 {}
impl CustomAddTrait for u32 {}
impl CustomAddTrait for u8 {}
impl CustomAddTrait for i8 {}
impl CustomAddTrait for f32 {}
impl CustomAddTrait for f64 {}

// 3. 然后定义一个带有泛型约束的函数
fn my_add2<T>(one: T, two: T) -> T
where
    T: CustomAddTrait + std::ops::Add<Output = T>,
{
    one + two
}

fn main() {
    let a = 123;
    let b = 5;

    let i1: u64 = 1232;
    let i2: u64 = 2313;

    // 会出现错误， u64并没有实现CustomAddTrait
    let r1 = my_add2(i1, i2);
    let r3 = my_add3(i1, i2);

    println!("{}", my_add(a, b));
}
