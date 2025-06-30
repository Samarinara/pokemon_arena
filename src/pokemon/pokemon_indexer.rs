use std::collections::HashMap;
use rand::Rng;
use serde_json;
use serde::{Deserialize};
 

#[derive(Deserialize)]
pub struct PokemonStatBlock {
    pub number: String,
    #[serde(rename = "HP")]
    pub hp: String,
    #[serde(rename = "Attack")]
    pub attack: String,
    #[serde(rename = "Defense")]
    pub defense: String,
    #[serde(rename = "SpAttack")]
    pub sp_attack: String,
    #[serde(rename = "SpDefense")]
    pub sp_defense: String,
    #[serde(rename = "Speed")]
    pub speed: String,
    #[serde(rename = "Type1")]
    pub type1: String,
    #[serde(rename = "Type2")]
    pub type2: String, // "none" will be stored as a string here
}

pub fn get_pokemon_by_number(number: i32) -> String {
    // Try to load the Pokemon data, return a fallback if it fails
    let json_data = match std::fs::read_to_string("src/pokemon/pokemon_by_number.json") {
        Ok(data) => data,
        Err(_) => return format!("Pokemon #{}", number),
    };
    
    let index: HashMap<i32, String> = match serde_json::from_str(&json_data) {
        Ok(index) => index,
        Err(_) => return format!("Pokemon #{}", number),
    };
    
    match index.get(&number) {
        Some(name) => name.to_string(),
        None => format!("Pokemon #{}", number),
    }
}

pub fn get_pokemon_stat_block(name: &str) -> Option<PokemonStatBlock> {
    // Try to load the Pokemon stats data, return None if it fails
    let json_data = match std::fs::read_to_string("src/pokemon/pokemon_stats.json") {
        Ok(data) => data,
        Err(_) => return None,
    };
    
    let mut index: HashMap<String, PokemonStatBlock> = match serde_json::from_str(&json_data) {
        Ok(index) => index,
        Err(_) => return None,
    };
    
    index.remove(name)
}

pub fn get_random_pokemon() -> String {
    // Try to load the Pokemon data, return a fallback if it fails
    let json_data = match std::fs::read_to_string("src/pokemon/pokemon_by_number.json") {
        Ok(data) => data, 
        Err(_) => return "Unknown Pokemon".to_string(), // Fallback to a generic string
    };

    // Create a variable with the contents of the json
    let index: HashMap<i32, String> = match serde_json::from_str(&json_data) {
        Ok(index) => index,
        Err(_) => return "Unknown Pokemon".to_string(), // Fallback to a generic string
    };

    let mut rng = rand::thread_rng();

    let keys: Vec<i32> = index.keys().cloned().collect();
    let random_key = keys[rng.gen_range(0..keys.len())];

    return index.get(&random_key).unwrap().to_string();
}