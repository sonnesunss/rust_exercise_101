use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Planets {
    name: String,
    rotation_period: String,
    diameter: String,
    climate: String,
    gravity: String,
    terrain: String,
    surface_water: String,
    population: String,
    residents: Vec<String>,
    films: Vec<String>,
    created: String,
    edited: String,
    url: String,
}
