fn main() {
    let i1 = 123;
    let i2 = false;
    let i3 = 1.23;
    let i4 = "str slice ref";
    let i5 = [12, 321, 90];

    let i6 = "string".to_string();

    let i7 = Some(321);

    println!(
        "before: {} {} {} {} {:?} {} {:?}",
        i1, i2, i3, i4, i5, i6, i7
    );

    let x1 = i1; // copy
    let x2 = i2; // copy
    let x3 = i3; // copy
    let x4 = i4;
    let x5 = i5; // copy
    let x6 = i6.clone();
    let x7 = i7;

    println!(
        "later: {} {} {} {} {:?} {} {:?}",
        x1, x2, x3, x4, x5, x6, x7
    );
    println!("{} {} {} {} {:?} {} {:?}", i1, i2, i3, i4, i5, i6, i7);
}
