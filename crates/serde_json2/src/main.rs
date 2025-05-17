// default value for a filed
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Request {
    #[serde(default = "default_resource")]
    resource: String,
    #[serde(default)]
    timeout: Timeout,
    #[serde(default = "Priority::lowest")]
    priority: Priority,
}

/// Timeout in seconds.
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Timeout(u32);

#[derive(Debug, Deserialize)]
enum Priority {
    ExtraHigh,
    High,
    Normal,
    Low,
    ExtraLow,
}

impl Default for Timeout {
    fn default() -> Self {
        Timeout(30)
    }
}

impl Priority {
    fn lowest() -> Self {
        Priority::ExtraLow
    }
}

fn default_resource() -> String {
    "i\'m default value".to_string()
}

fn main() {
    let json1 = r#"[
        {
            "resource": "/users"
        },
        {
            "timeout": 5,
            "priority": "High"
        }
    ]"#;

    let request: Vec<Request> = serde_json::from_str(json1).unwrap();

    println!("{:?}", request[0]);
    println!("{:?}", request[1]);
}
