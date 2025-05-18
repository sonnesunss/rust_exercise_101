use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Vehicles {
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
    vehicle_class: String,
    #[serde(default = "default_pilots")]
    pilots: Vec<String>,
    films: Vec<String>,
    created: String,
    edited: String,
    url: String,
}

fn default_pilots() -> Vec<String> {
    vec![]
}
