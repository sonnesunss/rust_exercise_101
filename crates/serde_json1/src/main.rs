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

fn main() {
    let w1 = W { a: 0, b: 0 };
    let x1 = X(1, 2);
    let y1 = Y(3);
    let z1 = Z;

    let r1 = serde_json::to_string(&w1).unwrap();
    let r2 = serde_json::to_string(&x1).unwrap();
    let r3 = serde_json::to_string(&y1).unwrap();
    let r4 = serde_json::to_string(&z1).unwrap();

    println!("{:?}", r1);
    println!("{:?}", r2);
    println!("{:?}", r3);
    println!("{:?}", r4);
}
