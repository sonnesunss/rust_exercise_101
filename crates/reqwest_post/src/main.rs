use pretty_printer::pretty_print;
use reqwest::{self, Client};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //
    let json_data = r#"{"email": "john@mail.com","password": "changeme"}"#;

    let response = Client::new()
        .post("https://api.escuelajs.co/api/v1/auth/login")
        .header("Content-Type", "application/json")
        .body(json_data)
        .send()
        .await?;

    println!("response is -> {:?}", response.status());
    let rb = response.json::<OAuthToken>().await?;
    println!("response body is -> {:?}", rb);
    println!("response body is ---> {:?}", rb.access_token);
    println!("response body is ---> {:?}", rb.refresh_token);

    pretty_print!(json_data);

    Ok(())
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
struct OAuthToken {
    access_token: String,
    refresh_token: String,
}
