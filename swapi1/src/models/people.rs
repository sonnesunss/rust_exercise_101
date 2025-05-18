use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct People {
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
