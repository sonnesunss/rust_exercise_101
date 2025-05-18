use crate::films::Films;
use crate::people::People;
use pretty_printer::pretty_print;
use reqwest;
use tokio;

mod films;
mod people;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // https://swapi.info/api/people/x, x 属于数字N
    const SWAPI_GET_PEOPLE_BASE_URL: &str = "https://swapi.info/api/people/1";
    const SWAPI_GET_FILMS_BASE_URL: &str = "https://swapi.info/api/films/1";
    const SWAPI_GET_FILMS_BASE_URL1: &str = "https://swapi.info/api/films";

    let response1 = reqwest::get(SWAPI_GET_PEOPLE_BASE_URL).await?;
    let response2 = reqwest::get(SWAPI_GET_FILMS_BASE_URL).await?;
    let response3 = reqwest::get(SWAPI_GET_FILMS_BASE_URL1).await?;
    println!("get people Status: {}", response1.status());
    println!("get films status: {}", response2.status());

    let body = response1.json::<People>().await?;
    let body2 = response2.json::<Films>().await?;
    let body3 = response3.json::<Vec<Films>>().await?;

    pretty_print!(body);
    pretty_print!(body2);
    pretty_print!(body3);

    Ok(())
}
