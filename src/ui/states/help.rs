//! # Help State
//! 
//! This module handles the help state which includes:
//! - Application instructions
//! - Keyboard shortcuts
//! - Navigation help

/// Represents the help state for displaying instructions
#[derive(Debug)]
pub struct HelpState {
    /// Currently selected help section
    pub selected_section: usize,
    /// Available help sections
    pub sections: Vec<String>,
    /// Help content for each section
    pub content: Vec<Vec<String>>,
}

impl HelpState {
    /// Create a new help state with instructions
    pub fn new() -> Self {
        let sections = vec![
            "Navigation".to_string(),
            "Main Menu".to_string(),
            "Pokedex".to_string(),
            "Settings".to_string(),
            "Keyboard Shortcuts".to_string(),
        ];

        let content = vec![
            vec![
                "Navigation Controls:".to_string(),
                "".to_string(),
                "• Arrow Keys: Navigate menus and lists".to_string(),
                "• Enter: Select current option".to_string(),
                "• Escape: Go back or quit".to_string(),
                "• Tab: Switch between different views".to_string(),
                "".to_string(),
                "General Navigation:".to_string(),
                "• Use Up/Down arrows to move selection".to_string(),
                "• Press Enter to confirm selection".to_string(),
                "• Press Escape to go back or quit".to_string(),
            ],
            vec![
                "Main Menu Features:".to_string(),
                "".to_string(),
                "• Counter: Increment/decrement a counter".to_string(),
                "• Pokemon Display: Shows Pokemon based on counter".to_string(),
                "• Menu Options:".to_string(),
                "  - Increment Counter: Add 1 to counter".to_string(),
                "  - Decrement Counter: Subtract 1 from counter".to_string(),
                "  - Reset Counter: Set counter to 0".to_string(),
                "  - Open Pokedex: View Pokemon database".to_string(),
                "  - Settings: Configure application".to_string(),
                "  - Help: Show this help screen".to_string(),
                "  - Quit: Exit application".to_string(),
            ],
            vec![
                "Pokedex Features:".to_string(),
                "".to_string(),
                "• Pokemon List: Browse all 151 Pokemon".to_string(),
                "• Details View: See Pokemon stats and info".to_string(),
                "• Search: Find Pokemon by name".to_string(),
                "".to_string(),
                "Pokedex Controls:".to_string(),
                "• Arrow Keys: Navigate Pokemon list".to_string(),
                "• Enter: View Pokemon details".to_string(),
                "• Tab: Toggle between list and details view".to_string(),
                "• /: Start search mode".to_string(),
                "• Escape: Exit search or go back".to_string(),
            ],
            vec![
                "Settings Options:".to_string(),
                "".to_string(),
                "• Theme: Choose visual theme".to_string(),
                "  - Default: Standard appearance".to_string(),
                "  - Dark: Dark color scheme".to_string(),
                "  - Light: Light color scheme".to_string(),
                "  - Colorful: Vibrant colors".to_string(),
                "".to_string(),
                "• Animations: Enable/disable UI animations".to_string(),
                "• Refresh Rate: Adjust UI update frequency".to_string(),
                "".to_string(),
                "Settings Controls:".to_string(),
                "• Arrow Keys: Navigate options".to_string(),
                "• Enter: Modify selected setting".to_string(),
                "• +/-: Adjust numeric values".to_string(),
            ],
            vec![
                "Keyboard Shortcuts:".to_string(),
                "".to_string(),
                "Global Shortcuts:".to_string(),
                "• Escape: Go back or quit".to_string(),
                "• Ctrl+C: Force quit".to_string(),
                "• F1: Show help".to_string(),
                "".to_string(),
                "Navigation Shortcuts:".to_string(),
                "• Arrow Keys: Navigate".to_string(),
                "• Enter: Select/Confirm".to_string(),
                "• Tab: Switch views".to_string(),
                "• Space: Toggle options".to_string(),
                "".to_string(),
                "Special Shortcuts:".to_string(),
                "• /: Start search (in pokedex)".to_string(),
                "• +/-: Adjust values (in settings)".to_string(),
                "• 1-9: Quick navigation (where applicable)".to_string(),
            ],
        ];

        Self {
            selected_section: 0,
            sections,
            content,
        }
    }

    /// Move selection up in the help sections
    pub fn select_up(&mut self) {
        self.selected_section = if self.selected_section == 0 {
            self.sections.len() - 1
        } else {
            self.selected_section - 1
        };
    }

    /// Move selection down in the help sections
    pub fn select_down(&mut self) {
        self.selected_section = (self.selected_section + 1) % self.sections.len();
    }

    /// Get the currently selected section name
    pub fn get_selected_section(&self) -> &str {
        &self.sections[self.selected_section]
    }

    /// Get the content for the currently selected section
    pub fn get_current_content(&self) -> &Vec<String> {
        &self.content[self.selected_section]
    }

    /// Get the number of sections
    pub fn section_count(&self) -> usize {
        self.sections.len()
    }
} 