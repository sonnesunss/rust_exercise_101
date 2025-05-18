use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Starships {
    name: String,
    model: String,
    manufacturer: String,
    cost_in_credits: String,
    length: String,
    max_atmosphering_speed: String,
    crew: String,
    passengers: String,
    cargo_capacity: String,
    consumables: String,
    hyperdrive_rating: String,
    #[serde(rename = "MGLT")]
    mglt: String,
    starship_class: String,
    pilots: Vec<String>,
    films: Vec<String>,
    created: String,
    edited: String,
    url: String,
}
