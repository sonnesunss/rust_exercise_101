use reqwest;
use serde::{Deserialize, Serialize};
use tokio;

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
struct People {
    name: String,
    height: String,
    mass: String,
    hair_color: String,
    skin_color: String,
    eye_color: String,
    birth_year: String,
    gender: String,
    homeworld: String,
    films: Vec<String>,
    species: Vec<String>,
    vehicles: Vec<String>,
    starships: Vec<String>,
    created: String,
    edited: String,
    url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // https://swapi.info/api/people/x, x 属于数字N
    const SWAPI_GET_PEOPLE_BASE_URL: &str = "https://swapi.info/api/people/1";

    let response1 = reqwest::get(SWAPI_GET_PEOPLE_BASE_URL).await?;
    println!("Status: {}", response1.status());

    let body = response1.json::<People>().await?;
    println!("Body: \n {:?}", body);

    Ok(())
}
