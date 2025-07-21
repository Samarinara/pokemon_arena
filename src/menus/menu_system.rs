use crate::{AppState, AuthState};

/// Represents a menu with its items and navigation logic
pub struct Menu {
    pub items: Vec<&'static str>,
    pub title: &'static str,
}

impl Menu {
    pub fn new(items: Vec<&'static str>, title: &'static str) -> Self {
        Self { items, title }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

/// Centralized menu management system
pub struct MenuSystem;

impl MenuSystem {
    /// Get the current menu based on app state
    pub fn get_current_menu(app_state: AppState, auth_state: AuthState) -> Menu {
        match app_state {
            AppState::MainMenu => Menu::new(
                vec!["Start", "Settings", "Pokedex", "Quit"],
                "Main Menu"
            ),
            AppState::Settings => Menu::new(
                vec!["Option 1", "Option 2", "Option 3", "Back"],
                "Settings"
            ),
            AppState::Pokedex => Menu::new(
                vec!["Search Pokemon", "View All", "Favorites", "Back"],
                "Pokedex"
            ),
            AppState::Auth => match auth_state {
                AuthState::InputEmail => Menu::new(
                    vec!["Send Verification Email", "Exit"],
                    "Email Input"
                ),
                AuthState::VerifyEmail => Menu::new(
                    vec!["Submit", "Resend Email", "Change Email", "Exit"],
                    "Verification"
                ),
                AuthState::LoggedIn => Menu::new(
                    vec!["Continue", "Logout"],
                    "Logged In"
                ),
            },
        }
    }

    /// Get the number of items in the current menu
    pub fn get_menu_size(app_state: AppState, auth_state: AuthState) -> usize {
        Self::get_current_menu(app_state, auth_state).len()
    }

    /// Check if a selection is valid for the current menu
    pub fn is_valid_selection(selection: usize, app_state: AppState, auth_state: AuthState) -> bool {
        let menu_size = Self::get_menu_size(app_state, auth_state);
        selection < menu_size
    }

    /// Clamp selection to valid range for current menu
    pub fn clamp_selection(selection: &mut usize, app_state: AppState, auth_state: AuthState) {
        let menu_size = Self::get_menu_size(app_state, auth_state);
        if *selection >= menu_size {
            *selection = menu_size.saturating_sub(1);
        }
    }

    /// Reset selection to 0 when menu changes (useful for state transitions)
    pub fn reset_selection_for_state_change(selection: &mut usize, app_state: AppState, auth_state: AuthState) {
        *selection = 0;
    }

    /// Handle up arrow navigation
    pub fn handle_up_arrow(selection: &mut usize, app_state: AppState, auth_state: AuthState) {
        if *selection > 0 {
            *selection -= 1;
        }
    }

    /// Handle down arrow navigation
    pub fn handle_down_arrow(selection: &mut usize, app_state: AppState, auth_state: AuthState) {
        let menu_size = Self::get_menu_size(app_state, auth_state);
        if *selection < menu_size.saturating_sub(1) {
            *selection += 1;
        }
    }
}

/// Legacy support - keep the old MENU_ITEMS constant for backward compatibility
pub const MENU_ITEMS: &[&str] = &["Start", "Settings", "Pokedex", "Quit"]; 