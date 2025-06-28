//! # UI States Module
//! 
//! This module defines the different UI states that the application can be in,
//! and provides a state manager to handle transitions between them.

use std::time::Instant;

// Import all state modules
pub mod main_menu;
pub mod pokedex;
pub mod settings;
pub mod help;

// Re-export state types
pub use main_menu::MainMenuState;
pub use pokedex::PokedexState;
pub use settings::SettingsState;
pub use help::HelpState;

/// Represents the different UI states the application can be in
#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    /// Main menu with counter and basic options
    MainMenu,
    /// Pokedex view showing Pokemon information
    Pokedex,
    /// Settings or configuration screen
    Settings,
    /// Help/instructions screen
    Help,
}

/// Main application struct that manages all states and provides a unified interface
#[derive(Debug)]
pub struct App {
    /// State manager for handling transitions
    pub state_manager: StateManager,
    /// Main menu state
    pub main_menu: MainMenuState,
    /// Pokedex state
    pub pokedex: PokedexState,
    /// Settings state
    pub settings: SettingsState,
    /// Help state
    pub help: HelpState,
}

impl App {
    /// Create a new application instance with all states initialized
    pub fn new() -> Self {
        Self {
            state_manager: StateManager::new(),
            main_menu: MainMenuState::new(),
            pokedex: PokedexState::new(),
            settings: SettingsState::new(),
            help: HelpState::new(),
        }
    }

    /// Get a reference to the current state manager
    pub fn state_manager(&self) -> &StateManager {
        &self.state_manager
    }

    /// Get a mutable reference to the current state manager
    pub fn state_manager_mut(&mut self) -> &mut StateManager {
        &mut self.state_manager
    }

    /// Get the current application state
    pub fn current_state(&self) -> &AppState {
        &self.state_manager.current_state
    }

    /// Check if the application should quit
    pub fn should_quit(&self) -> bool {
        self.state_manager.should_quit
    }

    /// Update the application (called on each tick)
    pub fn tick(&mut self) {
        self.state_manager.tick();
    }

    /// Switch to main menu
    pub fn switch_to_main_menu(&mut self) {
        self.state_manager.switch_to_main_menu();
    }

    /// Switch to pokedex
    pub fn switch_to_pokedex(&mut self) {
        self.state_manager.switch_to_pokedex();
    }

    /// Switch to settings
    pub fn switch_to_settings(&mut self) {
        self.state_manager.switch_to_settings();
    }

    /// Switch to help
    pub fn switch_to_help(&mut self) {
        self.state_manager.switch_to_help();
    }

    /// Go back to previous state
    pub fn go_back(&mut self) {
        self.state_manager.go_back();
    }

    /// Quit the application
    pub fn quit(&mut self) {
        self.state_manager.quit();
    }
}

/// Manages the current state and provides methods to switch between states
#[derive(Debug)]
pub struct StateManager {
    /// Current UI state
    pub current_state: AppState,
    /// Previous state (for back navigation)
    pub previous_state: Option<AppState>,
    /// Whether the app should quit
    pub should_quit: bool,
    /// Timer for animations or periodic updates
    pub last_tick: Instant,
}

impl StateManager {
    /// Create a new state manager with default state
    pub fn new() -> Self {
        Self {
            current_state: AppState::MainMenu,
            previous_state: None,
            should_quit: false,
            last_tick: Instant::now(),
        }
    }

    /// Switch to a new state, storing the current state as previous
    pub fn switch_to(&mut self, new_state: AppState) {
        self.previous_state = Some(self.current_state.clone());
        self.current_state = new_state;
    }

    /// Go back to the previous state (if available)
    pub fn go_back(&mut self) {
        if let Some(prev_state) = self.previous_state.take() {
            self.current_state = prev_state;
        }
    }

    /// Switch to main menu
    pub fn switch_to_main_menu(&mut self) {
        self.switch_to(AppState::MainMenu);
    }

    /// Switch to pokedex
    pub fn switch_to_pokedex(&mut self) {
        self.switch_to(AppState::Pokedex);
    }

    /// Switch to settings
    pub fn switch_to_settings(&mut self) {
        self.switch_to(AppState::Settings);
    }

    /// Switch to help
    pub fn switch_to_help(&mut self) {
        self.switch_to(AppState::Help);
    }

    /// Quit the application
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    /// Update the last tick time
    pub fn tick(&mut self) {
        self.last_tick = Instant::now();
    }
} 