use std::collections::HashMap;
use serde_json;

use crate::serde_handler;

pub fn get_pokemon_by_number(number: i32) -> String{
    let index: HashMap<i32, String> = serde_json::from_str(&serde_handler::load_json("src/pokemon/pokemon_by_number.json")).unwrap();
    return index.get(&number).unwrap().to_string();
}

