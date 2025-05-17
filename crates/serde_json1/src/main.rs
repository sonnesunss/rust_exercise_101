use serde::{Deserialize, Serialize};

// C-Like struct
#[derive(Debug, Clone, Serialize)]
struct W {
    a: i32,
    b: i32,
}

// struct (Type);
#[derive(Debug, Clone, Serialize)]
struct X(i32, i32);

#[derive(Debug, Clone, Serialize)]
struct Y(i32);

// struct Name;
#[derive(Debug, Clone, Serialize)]
struct Z;

#[derive(Debug, Serialize, Deserialize)]
struct Params {
    p1: String,
    p2: i32,
    p3: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Value {
    n: String,
}

#[derive(Debug, Serialize, Deserialize)]
enum Message {
    Request {
        id: String,
        method: String,
        params: Params,
    },
    Response {
        id: String,
        result: Value,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "root_type")]
enum Message2 {
    Request { id: String, method: String },

    Response { id: String, result: Value },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum Message3 {
    Request { id: String, method: String },

    Response { id: String, result: Value },
}

fn main() {
    // struct
    let w1 = W { a: 0, b: 0 };
    let x1 = X(1, 2);
    let y1 = Y(3);
    let z1 = Z;

    let r1 = serde_json::to_string(&w1).unwrap();
    let r2 = serde_json::to_string(&x1).unwrap();
    let r3 = serde_json::to_string(&y1).unwrap();
    let r4 = serde_json::to_string(&z1).unwrap();

    // enum
    let msg1 = Message::Response {
        id: "101".to_string(),
        result: Value {
            n: "sun".to_string(),
        },
    };

    let msg2 = Message2::Request {
        id: "67889".to_string(),
        method: "get".to_string(),
    };

    let msg3 = Message3::Request {
        id: "102".to_string(),
        method: "post".to_string(),
    };

    let r5 = serde_json::to_string(&msg1).unwrap();
    let r6 = serde_json::to_string(&msg2).unwrap();
    let r7 = serde_json::to_string(&msg3).unwrap();

    println!("c-like struct Serialized -> {:?}", r1);
    println!("struct-tuple Serialized  -> {:?}", r2);
    println!("struct-tuple Serialized  -> {:?}", r3);
    println!("struct-empty Serialized  -> {:?}", r4);
    println!("tagged enum Serialized   ->{:?}", r5);
    println!("tagged enum Serialized   ->{:?}", r6);
    println!("untagged enum Serialized ->{:?}", r7);
}
