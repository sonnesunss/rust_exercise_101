/*
    策略模式

    策略模式的基本思想是，给定一个解决特定问题的算法，我们只在抽象层面上定义算法的骨架，并将具体
的算法实现分成不同的部分

    使用该算法的客户可以选择一个具体的实现，而一般的算法工作流程保持不变。 换句话说，类的抽象规范并不取决于派生类的具体实现，但具体实现必须遵守抽象规范。 这就是为什么我们称之为“依赖反转”

    想象一下，我们正在做一个每月都会生成报告的项目。 我们需要以不同的格式（策略）生成报告，例如，以JSON或Plain Text格式。 但事情随着时间的推移而变化，我们不知道未来可能得到什么样的要求。 例如，我们可能需要以一种全新的格式生成我们的报告，或者只是修改现有的一种格式。

*/
use std::collections::HashMap;

type Data = HashMap<String, u32>;

#[allow(dead_code)]
trait Formatter {
    fn format(&self, data: &Data, buf: &mut String);
}

#[allow(dead_code)]
struct Report;

impl Report {
    fn generate<T: Formatter>(g: T, s: &mut String) {
        let mut data = HashMap::new();
        data.insert("one".to_string(), 1);
        data.insert("two".to_string(), 2);
        // generate Report
        g.format(&data, s);
    }
}

#[allow(dead_code)]
struct Text;

impl Formatter for Text {
    fn format(&self, data: &Data, buf: &mut String) {
        for (k, v) in data {
            let entry = format!("{} {}\n", k, v);
            buf.push_str(&entry);
        }
    }
}

#[allow(dead_code)]
struct Json;

impl Formatter for Json {
    fn format(&self, data: &Data, buf: &mut String) {
        buf.push('[');

        for (k, v) in data.into_iter() {
            let entry = format!(r#"{{"{}":"{}"}}"#, k, v);
            buf.push_str(&entry);
            buf.push(',');
        }
        buf.pop();
        buf.push(']');
    }
}

fn main() {
    let mut s = String::from("");
    Report::generate(Text, &mut s);
    assert!(s.contains("one 1"));
    assert!(s.contains("two 2"));

    s.clear();

    Report::generate(Json, &mut s);
    assert!(s.contains(r#"{"one":"1"}"#));
    assert!(s.contains(r#"{"two":"2"}"#));
}
