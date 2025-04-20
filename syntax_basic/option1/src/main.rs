fn main() {
    let option1 = Some(23);
    let option2 = Some(String::from("this is an option var"));
    let option3: Option<u8> = None;
    let mut option4 = Some(321);
    let option5 = Some("oooooo".to_string());
    let option6: Option<i32> = None;
    let option7 = Some("pppppp".to_string());

    // 如果只考虑其中某个Option值的case可以使用 if let解绑定option
    if let Some(has_val) = option1 {
        println!("option1 is {}", has_val);
    }

    if let None = option3 {
        println!("option3 -。-");
    }

    // 映射Some(e: E)到Some(u: U), or None to None
    let len = option2.map(|e| e.len());
    println!("option2 Some val len -> {:?}", len);

    // take一个Some(val)从一个option中，会在原option处留已给None
    let take_val = option4.take();
    println!("take {:?}", take_val);

    // 提供一个default value，如果option当前是None，则返回defualt value
    let s1: String = option5.map_or("321".to_string(), |e| String::from(e));
    println!("s1 -> {}", s1);

    // 提供两个闭包，一个当option是None时执行的，另一个则是当option是Some时执行的
    let v1 = option6.map_or_else(|| 123, |e| e * 2);
    println!("option6 {}", v1);

    // if None return None, if Some(T) then fnOnce(T) -> Option(U)
    let v2 = option7.and_then(|s| Some(s.len()));
    println!("v2 -> {:?}", v2);
}
