use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u8,
    email: String,
}

impl Default for User {
    fn default() -> Self {
        User {
            name: "sun".to_string(),
            age: 30,
            email: "sun@example.com".to_string(),
        }
    }
}

fn main() {
    let user = User::default();

    let json_data = serde_json::to_string(&user);

    // 显式处理错误
    match json_data {
        Ok(data) => println!("Serialized JSON: {}", data),
        Err(e) => eprintln!("{e}"),
    }

    let jd1 = r#"
        {
            "name": "sun",
            "age": 30,
            "email": "sun@example.com"
        }
    "#;

    // 粗暴
    let user: User = serde_json::from_str(jd1).expect("Failed to deserialize data");
    println!(
        "name: {}, age: {}, email: {}",
        user.name, user.age, user.email
    );
}
