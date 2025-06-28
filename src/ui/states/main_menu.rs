//! # Main Menu State
//! 
//! This module handles the main menu state which includes:
//! - Counter functionality
//! - Menu navigation
//! - Pokemon display based on counter

/// Represents the main menu state with counter and menu functionality
#[derive(Debug)]
pub struct MainMenuState {
    /// Current counter value
    pub counter: u32,
    /// Current Pokemon name based on counter
    pub pokemon: String,
    /// Currently selected menu option
    pub selected_option: usize,
    /// Available menu options
    pub options: Vec<String>,
}

impl MainMenuState {
    /// Create a new main menu state with default values
    pub fn new() -> Self {
        Self {
            counter: 0,
            pokemon: "None".to_string(),
            selected_option: 0,
            options: vec![
                "Increment Counter".to_string(),
                "Decrement Counter".to_string(),
                "Reset Counter".to_string(),
                "Open Pokedex".to_string(),
                "Settings".to_string(),
                "Help".to_string(),
                "Quit".to_string(),
            ],
        }
    }

    /// Increment the counter and update Pokemon
    pub fn increment_counter(&mut self) {
        self.counter = self.counter.saturating_add(1);
        self.update_pokemon();
    }

    /// Decrement the counter and update Pokemon
    pub fn decrement_counter(&mut self) {
        self.counter = self.counter.saturating_sub(1);
        self.update_pokemon();
    }

    /// Reset the counter to zero
    pub fn reset_counter(&mut self) {
        self.counter = 0;
        self.update_pokemon();
    }

    /// Update the Pokemon name based on current counter
    pub fn update_pokemon(&mut self) {
        // Import the pokemon module here to avoid circular dependencies
        use crate::pokemon::pokemon_indexer;
        self.pokemon = pokemon_indexer::get_pokemon_by_number(self.counter as i32);
    }

    /// Move selection up in the menu
    pub fn select_up(&mut self) {
        self.selected_option = if self.selected_option == 0 {
            self.options.len() - 1
        } else {
            self.selected_option - 1
        };
    }

    /// Move selection down in the menu
    pub fn select_down(&mut self) {
        self.selected_option = (self.selected_option + 1) % self.options.len();
    }

    /// Get the currently selected option
    pub fn get_selected_option(&self) -> &str {
        &self.options[self.selected_option]
    }

    /// Get the number of options in the menu
    pub fn option_count(&self) -> usize {
        self.options.len()
    }
} 