//! # Pokedex State
//! 
//! This module handles the pokedex state which includes:
//! - Pokemon number input (like counter in main menu)
//! - Pokemon stats display
//! - ASCII sprite placeholder
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

/// Represents the pokedex state for viewing Pokemon information
#[derive(Debug)]
pub struct PokedexState {
    /// Current Pokemon number (1-151)
    pub pokemon_number: u32,
    /// Current Pokemon name
    pub pokemon_name: String,
}

impl PokedexState {
    /// Create a new pokedex state
    pub fn new() -> Self {
        Self {
            pokemon_number: 1,
            pokemon_name: "Bulbasaur".to_string(),
        }
    }

    /// Increment the Pokemon number and update name
    pub fn increment_number(&mut self) {
        self.pokemon_number = (self.pokemon_number % 151) + 1;
        self.update_pokemon_name();
    }

    /// Decrement the Pokemon number and update name
    pub fn decrement_number(&mut self) {
        self.pokemon_number = if self.pokemon_number == 1 {
            151
        } else {
            self.pokemon_number - 1
        };
        self.update_pokemon_name();
    }

    /// Set the Pokemon number directly
    pub fn set_number(&mut self, number: u32) {
        self.pokemon_number = number.clamp(1, 151);
        self.update_pokemon_name();
    }

    /// Update the Pokemon name based on current number
    pub fn update_pokemon_name(&mut self) {
        use crate::pokemon::pokemon_indexer;
        self.pokemon_name = pokemon_indexer::get_pokemon_by_number(self.pokemon_number as i32);
    }

    /// Get the currently selected Pokemon number
    pub fn get_pokemon_number(&self) -> u32 {
        self.pokemon_number
    }

    /// Get the currently selected Pokemon name
    pub fn get_pokemon_name(&self) -> &str {
        &self.pokemon_name
    }

    /// Get Pokemon stats for the selected Pokemon
    pub fn get_pokemon_stats(&self) -> Option<crate::pokemon::pokemon_indexer::PokemonStatBlock> {
        use crate::pokemon::pokemon_indexer;
        pokemon_indexer::get_pokemon_stat_block(&self.pokemon_name)
    }
} 