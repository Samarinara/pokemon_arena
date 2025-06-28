//! # Settings State
//! 
//! This module handles the settings state which includes:
//! - Application configuration options
//! - Theme settings
//! - Display preferences

/// Represents the settings state for application configuration
#[derive(Debug)]
pub struct SettingsState {
    /// Currently selected setting option
    pub selected_option: usize,
    /// Available settings options
    pub options: Vec<String>,
    /// Current theme setting
    pub theme: Theme,
    /// Whether to show animations
    pub show_animations: bool,
    /// UI refresh rate in milliseconds
    pub refresh_rate: u64,
}

/// Available theme options
#[derive(Debug, Clone, PartialEq)]
pub enum Theme {
    /// Default theme
    Default,
    /// Dark theme
    Dark,
    /// Light theme
    Light,
    /// Colorful theme
    Colorful,
}

impl SettingsState {
    /// Create a new settings state
    pub fn new() -> Self {
        Self {
            selected_option: 0,
            options: vec![
                "Theme".to_string(),
                "Animations".to_string(),
                "Refresh Rate".to_string(),
                "Back to Main Menu".to_string(),
            ],
            theme: Theme::Default,
            show_animations: true,
            refresh_rate: 250,
        }
    }

    /// Move selection up in the settings menu
    pub fn select_up(&mut self) {
        self.selected_option = if self.selected_option == 0 {
            self.options.len() - 1
        } else {
            self.selected_option - 1
        };
    }

    /// Move selection down in the settings menu
    pub fn select_down(&mut self) {
        self.selected_option = (self.selected_option + 1) % self.options.len();
    }

    /// Cycle through theme options
    pub fn cycle_theme(&mut self) {
        self.theme = match self.theme {
            Theme::Default => Theme::Dark,
            Theme::Dark => Theme::Light,
            Theme::Light => Theme::Colorful,
            Theme::Colorful => Theme::Default,
        };
    }

    /// Toggle animations on/off
    pub fn toggle_animations(&mut self) {
        self.show_animations = !self.show_animations;
    }

    /// Increase refresh rate
    pub fn increase_refresh_rate(&mut self) {
        self.refresh_rate = (self.refresh_rate + 50).min(1000);
    }

    /// Decrease refresh rate
    pub fn decrease_refresh_rate(&mut self) {
        self.refresh_rate = self.refresh_rate.saturating_sub(50).max(50);
    }

    /// Get the currently selected option
    pub fn get_selected_option(&self) -> &str {
        &self.options[self.selected_option]
    }

    /// Get theme name as string
    pub fn get_theme_name(&self) -> &str {
        match self.theme {
            Theme::Default => "Default",
            Theme::Dark => "Dark",
            Theme::Light => "Light",
            Theme::Colorful => "Colorful",
        }
    }
} 