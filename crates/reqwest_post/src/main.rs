use pretty_printer::pretty_print;
use reqwest::{self, Client};

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

    let response_body = response.text().await?;

    println!("response body iis -> {:?}", response_body);

    pretty_print!(json_data);

    Ok(())
}
