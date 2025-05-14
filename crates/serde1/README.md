# Serialization/Deserialization

简单说,

序列化就是转换原始数据结构到一个特定的格式，然后就可以放置在文件中、通过网络传输、存储在数据库中.

而反序列化就是这个过程的反向操作, 会将序列化后的数据恢复成原始数据结构.


应用场景:

1. 保存数据  保存数据到文件中
2. 数据传输  通过网络在不同的服务间传输数据
3. 数据交互  在不同系统、编程语言之间交换数据


## Rust中的序列化

在rust中，一个流行的序列化/反序列化库是Serde. 快速、可靠、支持多种格式是它的标签.


1. Setting Up Serde

```toml
[dependencies]
serde = { version = "x.y", features = ["derive"] }
serde_json = "x.y"
```

+ serde： 序列化/反序列化的核心实现
+ serde_json： serde with JSON

2. Serializing to JSON

```Rust
// 引入serde
use serfe::Serialize;
use serde_json;

// 使用派生宏为结构体添加或者说启用序列化的能力
#[derive(Serialize)]
struct User {
    name: String,
    age: u8,
    email: String,
}

fn main() {
    let user = User {
        name: String::from("Sun"),
        age: 30,
        email: String::from("sun@example.com"),
    };

    // serde_json::to_string()转换结构体到json字符串
    let json_data = serde_json::to_string(&user).expect("Failed to serialize data");
    println!("Serialized JSON: {}", json_data);
}
```


3. Deserialization from JSON

反序列化json string 到原始rust结构体

```rust
use serde::Deserialize;
use serde_json;

// 使用派生宏为结构体添加或者启用反序列化的能力
#[derive(Deserialize)]
struct User {
    name: String,
    age: u8,
    email: String,
}

fn main() {
    // r#raw string#
    let json_data = r#"{"name":"sun","age":30,"email":"sun@example.com"}"#;

    // serde_json：：from_str方法将 json string 反序列化为rust结构体
    let user: User = serde_json::from_str(json_data).expect("Failed to deserialize data");
    println!("Name: {}, Age: {}, Email: {}", user.name, user.age, user.email);
}
```


4. 处理序列化/反序列化中的错误

rust强制要求错误显式处理，在序列化、反序列化时会产生错误， 可以使用rust提供的错误处理工具处理之， 例如resutl
、match等

5. Serde with other formats

+ for example: TOML

```toml
[dependencies]
toml = "x.y"
```

```rust
use serde::{Serialize, Deserialize};
use toml;

#[derive(Serialize, Deserialize)]
struct Config {
    app_name: String,
    version: String,
    debug: bool,
}

fn main() {
    let config = Config {
        app_name: String::from("MyApp"),
        version: String::from("1.0.0"),
        debug: true,
    };

    // Serialize to TOML
    let toml_data = toml::to_string(&config).expect("Failed to serialize to TOML");
    println!("Serialized TOML:\n{}", toml_data);

    // Deserialize from TOML
    let parsed_config: Config = toml::from_str(&toml_data).expect("Failed to deserialize TOML");
    println!("Parsed Config - App Name: {}, Version: {}, Debug: {}",
             parsed_config.app_name, parsed_config.version, parsed_config.debug);
}
```

+ for example binary

```toml
[dependencies]
bincode = "x.y"
```

```rust
use serde::{Serialize, Deserialize};
use bincode;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 10, y: 20 };

    // Serialize to binary
    let binary_data = bincode::serialize(&point).expect("Failed to serialize to binary");
    println!("Serialized Binary: {:?}", binary_data);

    // Deserialize from binary
    let deserialized_point: Point = bincode::deserialize(&binary_data).expect("Failed to deserialize binary");
    println!("Deserialized Point: {:?}", deserialized_point);
}
```
