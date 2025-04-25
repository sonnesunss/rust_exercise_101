fn main() {
    println!("Hello, world!");

    let _r1 = my_add(1, 2);
    let _r2 = my_add(1.1, 2.2);
    let _r3 = max_value(4, 321);
    let _r4 = max_value(5.324, 5668.321);

    let s1 = "haha".to_string();
    let s2 = "hahahahah".to_string();

    let _rs1 = max_value2(&s1, &s2);

    println!("{}", _rs1);
}

// 泛型函数
#[allow(dead_code)]
fn my_add<T: std::ops::Add<Output = T>>(one: T, two: T) -> T {
    one + two
}

#[allow(dead_code)]
fn max_value<T: PartialOrd + PartialEq>(one: T, two: T) -> T {
    if std::cmp::PartialOrd::gt(&one, &two) {
        return one;
    }

    two
}

#[allow(dead_code)]
fn max_value2<'a, T: Ord>(one: &'a T, two: &'a T) -> &'a T {
    if std::cmp::PartialOrd::gt(one, two) {
        return one;
    }

    two
}
