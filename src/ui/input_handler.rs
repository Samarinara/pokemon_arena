//! # Input Handler Module
//! 
//! This module handles user input processing for different UI states.
//! It routes input events to the appropriate state handlers based on
//! the current application state.

use crossterm::event::{Event, KeyCode, KeyModifiers};
use crate::ui::states::{App, AppState};

/// Handle user input and update application state accordingly
/// This function routes input to the appropriate state handler
pub fn handle_input(app: &mut App, event: Event) {
    match event {
        // Handle key press events
        Event::Key(key_event) => {
            // Check for global shortcuts first
            if handle_global_shortcuts(app, &key_event) {
                return;
            }

            // Route to state-specific handlers
            match app.current_state() {
                AppState::MainMenu => handle_main_menu_input(app, &key_event),
                AppState::Pokedex => handle_pokedex_input(app, &key_event),
                AppState::Settings => handle_settings_input(app, &key_event),
                AppState::Help => handle_help_input(app, &key_event),
            }
        }
        
        // Handle mouse events (if needed)
        Event::Mouse(_) => {
            // Mouse handling would go here
        }
        
        // Handle resize events
        Event::Resize(_, _) => {
            // Terminal resize handling would go here
        }
        
        // Ignore other event types
        _ => {}
    }
}

/// Handle global shortcuts that work in any state
fn handle_global_shortcuts(app: &mut App, key_event: &crossterm::event::KeyEvent) -> bool {
    match key_event.code {
        // Escape key to go back or quit
        KeyCode::Esc => {
            match app.current_state() {
                AppState::MainMenu => {
                    app.quit();
                }
                _ => {
                    app.go_back();
                }
            }
            true
        }
        
        // F1 key to show help
        KeyCode::F(1) => {
            app.switch_to_help();
            true
        }
        
        // Ctrl+C to force quit
        KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
            app.quit();
            true
        }
        
        _ => false,
    }
}

/// Handle input for the main menu state
fn handle_main_menu_input(app: &mut App, key_event: &crossterm::event::KeyEvent) {
    match key_event.code {
        // Arrow keys for navigation
        KeyCode::Up => {
            app.main_menu.select_up();
        }
        KeyCode::Down => {
            app.main_menu.select_down();
        }
        
        // Enter key to select current option
        KeyCode::Enter => {
            handle_main_menu_selection(app);
        }
        
        // Ignore all other keys
        _ => {}
    }
}

/// Process the currently selected menu option in main menu
fn handle_main_menu_selection(app: &mut App) {
    match app.main_menu.selected_option {
        0 => {
            // Increment counter
            app.main_menu.increment_counter();
        }
        1 => {
            // Decrement counter
            app.main_menu.decrement_counter();
        }
        2 => {
            // Reset counter
            app.main_menu.reset_counter();
        }
        3 => {
            // Open Pokedex
            app.switch_to_pokedex();
        }
        4 => {
            // Settings
            app.switch_to_settings();
        }
        5 => {
            // Help
            app.switch_to_help();
        }
        6 => {
            // Quit
            app.quit();
        }
        _ => {
            // This shouldn't happen, but handle it gracefully
            eprintln!("Invalid selection: {}", app.main_menu.selected_option);
        }
    }
}

/// Handle input for the pokedex state
fn handle_pokedex_input(app: &mut App, key_event: &crossterm::event::KeyEvent) {
    match key_event.code {
        // Arrow keys for navigation
        KeyCode::Up => {
            app.pokedex.increment_number();
        }
        KeyCode::Down => {
            app.pokedex.decrement_number();
        }
        
        // Plus and minus keys for adjusting Pokemon number
        KeyCode::Char('+') | KeyCode::Char('=') => {
            app.pokedex.increment_number();
        }
        KeyCode::Char('-') => {
            app.pokedex.decrement_number();
        }
        
        // Ignore all other keys
        _ => {}
    }
}

/// Handle input for the settings state
fn handle_settings_input(app: &mut App, key_event: &crossterm::event::KeyEvent) {
    match key_event.code {
        // Arrow keys for navigation
        KeyCode::Up => {
            app.settings.select_up();
        }
        KeyCode::Down => {
            app.settings.select_down();
        }
        
        // Enter key to modify selected setting
        KeyCode::Enter => {
            handle_settings_selection(app);
        }
        
        // Plus and minus keys for adjusting values
        KeyCode::Char('+') | KeyCode::Char('=') => {
            match app.settings.selected_option {
                2 => app.settings.increase_refresh_rate(), // Refresh Rate
                _ => {}
            }
        }
        KeyCode::Char('-') => {
            match app.settings.selected_option {
                2 => app.settings.decrease_refresh_rate(), // Refresh Rate
                _ => {}
            }
        }
        
        // Ignore all other keys
        _ => {}
    }
}

/// Process the currently selected setting option
fn handle_settings_selection(app: &mut App) {
    match app.settings.selected_option {
        0 => {
            // Theme - cycle through themes
            app.settings.cycle_theme();
        }
        1 => {
            // Animations - toggle on/off
            app.settings.toggle_animations();
        }
        2 => {
            // Refresh Rate - already handled by +/- keys
        }
        3 => {
            // Back to Main Menu
            app.switch_to_main_menu();
        }
        _ => {
            // This shouldn't happen, but handle it gracefully
            eprintln!("Invalid selection: {}", app.settings.selected_option);
        }
    }
}

/// Handle input for the help state
fn handle_help_input(app: &mut App, key_event: &crossterm::event::KeyEvent) {
    match key_event.code {
        // Arrow keys for navigation between help sections
        KeyCode::Up => {
            app.help.select_up();
        }
        KeyCode::Down => {
            app.help.select_down();
        }
        
        // Enter key to go back (same as escape)
        KeyCode::Enter => {
            app.go_back();
        }
        
        // Ignore all other keys
        _ => {}
    }
} 