use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Films {
    title: String,
    episode_id: u32,
    opening_crawl: String,
    director: String,
    producer: String,
    release_date: String,
    characters: Vec<String>,
    planets: Vec<String>,
    starships: Vec<String>,
    vehicles: Vec<String>,
    species: Vec<String>,
    created: String,
    edited: String,
    url: String,
}
