/*
    # Rust Modules

    模块在rust中用来组织代码和作为命名空间阻止命名冲突以及控制代码的可见性.

    总之一句话，使用模块可以带来如下好处:

    1. 代码有组织性
    2. 复用性
    3. 封装性
    4. 代码更清晰，增强代码的可读、可维护性

    ##  如何创建Rust模块， 语法创建的模块

    创建一个模块的语法还是很简单的，使用mod name { // mod content }, 即可

    默认情况下，所有模块内的项目都是private私有的，通过在项目的前面添加pub关键字
    更改它的可见性为公开，这样可以在其他模块内使用它


    ## 文件/目录 模块，  基于文件目录的模块

    rust允许将模块组织到单独的一个文件或者目录，一个文件、一个目录就是一个模块！
    这就是基于文件的模块， 这种形式不需要显式的通过mod创建，每个文件、目录就是一个模块，
    文件名、目录名就是模块的名字，默认文件、目录内所有的项目内容都是私有的

    这样从模块的角度看，整个项目就是模块的树形结构集合

    举个例子:

    在项目目录src目录下面创建一个math.rs文件，与main.rs同级

    然后在其内放置两个rs文件，mod.rs以及a.rs

    其中mod.rs文件内是需要pub出来的其下的rs文件，a.rs文件内则是正常的rs代码

    ##
*/

// create a rust module
mod math1 {
    pub fn add1(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }
}

// 注意使用时需要在使用的mod内声明下
mod math;

// 注意这里的mod路径
use math::a::add2;

fn main() {
    let _sum = math1::add1(1, 2);
    let _difference = math1::subtract(2, 1);
    let _sum2 = add2(10, 20);
}
