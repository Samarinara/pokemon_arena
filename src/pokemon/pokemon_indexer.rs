use std::collections::HashMap;
use serde_json;
use serde::{Deserialize};
 
use crate::serde_handler;

#[derive(Deserialize)]
pub struct PokemonStatBlock {
    pub number: String,
    pub hp: String,
    pub attack: String,
    pub defense: String,
    pub sp_attack: String,
    pub sp_defense: String,
    pub speed: String,
    pub type1: String,
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
