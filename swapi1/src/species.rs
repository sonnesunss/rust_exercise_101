use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Species {
    name: String,
    classification: String,
    designation: String,
    average_height: String,
    skin_colors: String,
    hair_colors: String,
    eye_colors: String,
    average_lifespan: String,
    homeworld: String,
    language: String,
    people: Vec<String>,
    films: Vec<String>,
    created: String,
    edited: String,
    url: String,
}
