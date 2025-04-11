fn main() {
    let bp = BlogPost::new();
    let r = bp.summarize();

    println!("{}", r);
}

/*
定义一个 Summary trait，包含一个方法 fn summarize(&self) -> String。

创建一个结构体（例如 BlogPost），包含标题、作者、内容等字段，并为其实现 Summary trait。

在 main 中实例化该结构体并调用 summarize() 方法
*/
trait Summary {
    fn summarize(&self) -> String;
}

struct BlogPost {
    title: String,
    author: String,
    content: String,
    date: String,
}

impl BlogPost {
    fn new() -> Self {
        BlogPost {
            title: "rust trait basic".to_string(),
            author: "sun".to_string(),
            content: "xxxxxx".to_string(),
            date: "2025.04.11".to_string(),
        }
    }
}

impl Summary for BlogPost {
    fn summarize(&self) -> String {
        "summarize".to_string()
    }
}
