#!/usr/bin/env python3
"""
Pokemon Arena Menu Generator
This script creates a new custom menu state with all necessary files and code changes.
"""

import os
import sys
import re
from pathlib import Path

# Colors for output
class Colors:
    RED = '\033[0;31m'
    GREEN = '\033[0;32m'
    YELLOW = '\033[1;33m'
    BLUE = '\033[0;34m'
    NC = '\033[0m'  # No Color

def print_status(message):
    print(f"{Colors.GREEN}[INFO]{Colors.NC} {message}")

def print_warning(message):
    print(f"{Colors.YELLOW}[WARNING]{Colors.NC} {message}")

def print_error(message):
    print(f"{Colors.RED}[ERROR]{Colors.NC} {message}")

def print_header():
    print(f"{Colors.BLUE}================================{Colors.NC}")
    print(f"{Colors.BLUE}  Pokemon Arena Menu Generator{Colors.NC}")
    print(f"{Colors.BLUE}================================{Colors.NC}")

def capitalize_first(s):
    """Capitalize the first letter of a string"""
    return s[0].upper() + s[1:] if s else s

def create_menu_state_file(menu_name, menu_name_capitalized):
    """Create the menu state file"""
    menu_file = f"src/ui/states/{menu_name}.rs"
    print_status(f"Creating menu state file: {menu_file}")
    
    content = f'''//! # {menu_name_capitalized} Menu State
//! 
//! This module handles the {menu_name} menu state which includes:
//! - Custom functionality for {menu_name}
//! - Menu navigation
//! - State management

/// Represents the {menu_name} menu state
#[derive(Debug)]
pub struct {menu_name_capitalized}State {{
    /// Currently selected menu option
    pub selected_option: usize,
    /// Available menu options
    pub options: Vec<String>,
    /// Custom data for {menu_name} functionality
    pub custom_data: String,
}}

impl {menu_name_capitalized}State {{
    /// Create a new {menu_name} menu state with default values
    pub fn new() -> Self {{
        Self {{
            selected_option: 0,
            options: vec![
                "Option 1".to_string(),
                "Option 2".to_string(),
                "Option 3".to_string(),
                "Back to Main Menu".to_string(),
            ],
            custom_data: "Default {menu_name} data".to_string(),
        }}
    }}

    /// Move selection up in the menu
    pub fn select_up(&mut self) {{
        self.selected_option = if self.selected_option == 0 {{
            self.options.len() - 1
        }} else {{
            self.selected_option - 1
        }};
    }}

    /// Move selection down in the menu
    pub fn select_down(&mut self) {{
        self.selected_option = (self.selected_option + 1) % self.options.len();
    }}

    /// Get the currently selected option
    pub fn get_selected_option(&self) -> &str {{
        &self.options[self.selected_option]
    }}

    /// Get the number of options in the menu
    pub fn option_count(&self) -> usize {{
        self.options.len()
    }}

    /// Custom method for {menu_name} functionality
    pub fn custom_action(&mut self) {{
        self.custom_data = format!("{menu_name} action performed at {{:?}}", std::time::Instant::now());
    }}
}}
'''
    
    with open(menu_file, 'w') as f:
        f.write(content)
    
    print_status("Menu state file created successfully")

def update_states_module(menu_name, menu_name_capitalized):
    """Update the states module file"""
    mod_file = "src/ui/states/mod.rs"
    print_status("Updating states module file...")
    
    with open(mod_file, 'r') as f:
        content = f.read()
    
    # Add module declaration
    if f"pub mod {menu_name};" not in content:
        content = re.sub(
            r'(pub mod help;)',
            f'\\1\npub mod {menu_name};',
            content
        )
        print_status("Added module declaration")
    
    # Add re-export
    if f"pub use {menu_name}::{menu_name_capitalized}State;" not in content:
        content = re.sub(
            r'(pub use help::HelpState;)',
            f'\\1\npub use {menu_name}::{menu_name_capitalized}State;',
            content
        )
        print_status("Added re-export")
    
    # Add to AppState enum
    if f"/// {menu_name_capitalized} menu" not in content:
        content = re.sub(
            r'(    /// Help/instructions screen\n    Help,)',
            f'    /// {menu_name_capitalized} menu\n    {menu_name_capitalized},\n\\1',
            content
        )
        print_status("Added to AppState enum")
    
    # Add to App struct
    if f"pub {menu_name}: {menu_name_capitalized}State," not in content:
        content = re.sub(
            r'(    pub help: HelpState,)',
            f'    /// {menu_name} state\n    pub {menu_name}: {menu_name_capitalized}State,\n\\1',
            content
        )
        print_status("Added to App struct")
    
    # Add to App::new()
    if f"{menu_name}: {menu_name_capitalized}State::new()," not in content:
        content = re.sub(
            r'(            help: HelpState::new\(\),)',
            f'            {menu_name}: {menu_name_capitalized}State::new(),\n\\1',
            content
        )
        print_status("Added to App::new()")
    
    # Add switch method to App
    if f"pub fn switch_to_{menu_name}" not in content:
        content = re.sub(
            r'(    pub fn switch_to_help\(&mut self\) \{\n        self\.state_manager\.switch_to_help\(\);\n    \})',
            f'\\1\n\n    /// Switch to {menu_name}\n    pub fn switch_to_{menu_name}(&mut self) {{\n        self.state_manager.switch_to_{menu_name}();\n    }}',
            content
        )
        print_status("Added switch method to App")
    
    # Add switch method to StateManager
    if f"pub fn switch_to_{menu_name}" not in content:
        content = re.sub(
            r'(    pub fn switch_to_help\(&mut self\) \{\n        self\.switch_to\(AppState::Help\);\n    \})',
            f'\\1\n\n    /// Switch to {menu_name}\n    pub fn switch_to_{menu_name}(&mut self) {{\n        self.switch_to(AppState::{menu_name_capitalized});\n    }}',
            content
        )
        print_status("Added switch method to StateManager")
    
    with open(mod_file, 'w') as f:
        f.write(content)
    
    print_status("States module updated successfully")

def update_renderer(menu_name, menu_name_capitalized):
    """Update the renderer file"""
    renderer_file = "src/ui/renderer.rs"
    print_status("Updating renderer...")
    
    with open(renderer_file, 'r') as f:
        content = f.read()
    
    # Add to header match statement
    if f"AppState::{menu_name_capitalized}" not in content:
        content = re.sub(
            r'(        AppState::Help => \()',
            f'        AppState::{menu_name_capitalized} => (\n            "Pokemon Arena - {menu_name_capitalized}",\n            "{menu_name_capitalized} menu: Arrow keys to navigate, Enter to select"\n        ),\n\\1',
            content
        )
        print_status("Added to header match")
    
    # Add to main content match statement
    if f"AppState::{menu_name_capitalized} => draw_{menu_name}_content" not in content:
        content = re.sub(
            r'(        AppState::Help => draw_help_content\(f, area, app\),)',
            f'        AppState::{menu_name_capitalized} => draw_{menu_name}_content(f, area, app),\n\\1',
            content
        )
        print_status("Added to main content match")
    
    # Add the draw function
    if f"fn draw_{menu_name}_content" not in content:
        draw_function = f'''

/// Draw the {menu_name} content
fn draw_{menu_name}_content(f: &mut Frame, area: Rect, app: &App) {{
    // Split the area into sections
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Title section
            Constraint::Min(0),     // Menu section
            Constraint::Length(3),  // Info section
        ].as_ref())
        .split(area);

    // Draw title
    let title_widget = Paragraph::new(vec![
        Line::from(vec![
            Span::styled(
                "{menu_name_capitalized} Menu",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ])
    .block(Block::default().borders(Borders::ALL).title("{menu_name_capitalized}"));

    f.render_widget(title_widget, chunks[0]);

    // Draw menu
    let menu_items = app.{menu_name}
        .options
        .iter()
        .enumerate()
        .map(|(index, option)| {{
            if index == app.{menu_name}.selected_option {{
                Line::from(vec![
                    Span::styled(
                        format!("> {{}}", option),
                        Style::default()
                            .fg(Color::Black)
                            .bg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    ),
                ])
            }} else {{
                Line::from(vec![
                    Span::styled(
                        format!("  {{}}", option),
                        Style::default().fg(Color::White),
                    ),
                ])
            }}
        }})
        .collect::<Vec<_>>();

    let menu_widget = Paragraph::new(menu_items)
        .block(Block::default().borders(Borders::ALL).title("Options"));

    f.render_widget(menu_widget, chunks[1]);

    // Draw info section
    let info_widget = Paragraph::new(vec![
        Line::from(vec![
            Span::styled(
                format!("Custom Data: {{}}", app.{menu_name}.custom_data),
                Style::default().fg(Color::Green),
            ),
        ]),
    ])
    .block(Block::default().borders(Borders::ALL).title("Info"));

    f.render_widget(info_widget, chunks[2]);
}}
'''
        
        # Insert before draw_footer function
        content = re.sub(
            r'(fn draw_footer)',
            f'{draw_function}\n\\1',
            content
        )
        print_status("Added draw function")
    
    with open(renderer_file, 'w') as f:
        f.write(content)
    
    print_status("Renderer updated successfully")

def update_input_handler(menu_name, menu_name_capitalized):
    """Update the input handler file"""
    input_file = "src/ui/input_handler.rs"
    print_status("Updating input handler...")
    
    with open(input_file, 'r') as f:
        content = f.read()
    
    # Add to main input match statement
    if f"AppState::{menu_name_capitalized} => handle_{menu_name}_input" not in content:
        content = re.sub(
            r'(        AppState::Help => handle_help_input\(app, &key_event\),)',
            f'        AppState::{menu_name_capitalized} => handle_{menu_name}_input(app, &key_event),\n\\1',
            content
        )
        print_status("Added to input match")
    
    # Add the input handler functions
    if f"fn handle_{menu_name}_input" not in content:
        input_functions = f'''

/// Handle input for the {menu_name} state
fn handle_{menu_name}_input(app: &mut App, key_event: &crossterm::event::KeyEvent) {{
    match key_event.code {{
        // Arrow keys for navigation
        KeyCode::Up => {{
            app.{menu_name}.select_up();
        }}
        KeyCode::Down => {{
            app.{menu_name}.select_down();
        }}
        
        // Enter key to select current option
        KeyCode::Enter => {{
            handle_{menu_name}_selection(app);
        }}
        
        // Ignore all other keys
        _ => {{}}
    }}
}}

/// Process the currently selected menu option in {menu_name}
fn handle_{menu_name}_selection(app: &mut App) {{
    match app.{menu_name}.selected_option {{
        0 => {{
            // Option 1
            app.{menu_name}.custom_action();
        }}
        1 => {{
            // Option 2
            app.{menu_name}.custom_action();
        }}
        2 => {{
            // Option 3
            app.{menu_name}.custom_action();
        }}
        3 => {{
            // Back to Main Menu
            app.switch_to_main_menu();
        }}
        _ => {{
            // This shouldn't happen, but handle it gracefully
            eprintln!("Invalid selection: {{}}", app.{menu_name}.selected_option);
        }}
    }}
}}
'''
        
        # Add at the end of the file
        content += input_functions
        print_status("Added input handler")
    
    with open(input_file, 'w') as f:
        f.write(content)
    
    print_status("Input handler updated successfully")

def update_main_menu(menu_name, menu_name_capitalized):
    """Update the main menu to include the new menu option"""
    main_menu_file = "src/ui/states/main_menu.rs"
    print_status("Adding menu option to main menu...")
    
    with open(main_menu_file, 'r') as f:
        content = f.read()
    
    # Add the menu option (before "Help")
    if f'"{menu_name_capitalized}"' not in content:
        content = re.sub(
            r'("Help".to_string\(\),)',
            f'                "{menu_name_capitalized}".to_string(),\n\\1',
            content
        )
        print_status("Added menu option to main menu")
    
    with open(main_menu_file, 'w') as f:
        f.write(content)
    
    # Update the main menu input handler
    input_file = "src/ui/input_handler.rs"
    print_status("Updating main menu input handler...")
    
    with open(input_file, 'r') as f:
        content = f.read()
    
    # Update the selection handler to include the new menu
    if f"app.switch_to_{menu_name}();" not in content:
        # Find the current selection handler and update it
        # This is a bit more complex, so we'll do it step by step
        
        # First, let's find the pattern and add our new case
        pattern = r'(\s+)(\d+) => \{\s+// Help\s+app\.switch_to_help\(\);\s+\}\s+(\d+) => \{\s+// Quit\s+app\.quit\(\);\s+\}'
        replacement = f'\\1\\2 => {{\n            // {menu_name_capitalized}\n            app.switch_to_{menu_name}();\n        }}\n        \\3 => {{\n            // Help\n            app.switch_to_help();\n        }}\n        \\4 => {{\n            // Quit\n            app.quit();\n        }}'
        
        # Update the numbers
        replacement = re.sub(r'\\3', str(int(re.search(r'(\d+) => \{\s+// Help', content).group(1)) + 1), replacement)
        replacement = re.sub(r'\\4', str(int(re.search(r'(\d+) => \{\s+// Help', content).group(1)) + 2), replacement)
        
        content = re.sub(pattern, replacement, content)
        print_status("Updated main menu selection handler")
    
    with open(input_file, 'w') as f:
        f.write(content)
    
    print_status("Main menu updated successfully")

def create_documentation():
    """Create documentation for menu creation"""
    doc_file = "docs/menu_creation.md"
    print_status("Creating documentation...")
    
    os.makedirs("docs", exist_ok=True)
    
    content = '''# Adding Custom Menus to Pokemon Arena

This document explains how to add new custom menus to the Pokemon Arena application.

## Quick Start

To add a new menu called "battle", simply run:

```bash
python3 scripts/generate_menu.py battle
```

This will automatically:
- Create the menu state file
- Update all necessary modules
- Add navigation to the main menu
- Create input handlers
- Add rendering functions

## Manual Process

If you prefer to create menus manually, follow these steps:

### 1. Create the State File

Create `src/ui/states/your_menu.rs`:

```rust
//! # Your Menu State

#[derive(Debug)]
pub struct YourMenuState {
    pub selected_option: usize,
    pub options: Vec<String>,
    // Add your custom fields here
}

impl YourMenuState {
    pub fn new() -> Self {
        Self {
            selected_option: 0,
            options: vec![
                "Option 1".to_string(),
                "Option 2".to_string(),
                "Back to Main Menu".to_string(),
            ],
        }
    }

    pub fn select_up(&mut self) {
        self.selected_option = if self.selected_option == 0 {
            self.options.len() - 1
        } else {
            self.selected_option - 1
        };
    }

    pub fn select_down(&mut self) {
        self.selected_option = (self.selected_option + 1) % self.options.len();
    }

    pub fn get_selected_option(&self) -> &str {
        &self.options[self.selected_option]
    }
}
```

### 2. Update States Module

Add to `src/ui/states/mod.rs`:

```rust
// Add module declaration
pub mod your_menu;

// Add re-export
pub use your_menu::YourMenuState;

// Add to AppState enum
#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    // ... existing states
    YourMenu,
}

// Add to App struct
pub struct App {
    // ... existing fields
    pub your_menu: YourMenuState,
}

// Add to App::new()
impl App {
    pub fn new() -> Self {
        Self {
            // ... existing initializations
            your_menu: YourMenuState::new(),
        }
    }

    // Add switch method
    pub fn switch_to_your_menu(&mut self) {
        self.state_manager.switch_to_your_menu();
    }
}

// Add to StateManager
impl StateManager {
    pub fn switch_to_your_menu(&mut self) {
        self.switch_to(AppState::YourMenu);
    }
}
```

### 3. Update Renderer

Add to `src/ui/renderer.rs`:

```rust
// Add to header match
AppState::YourMenu => (
    "Pokemon Arena - Your Menu",
    "Your menu: Arrow keys to navigate, Enter to select"
),

// Add to main content match
AppState::YourMenu => draw_your_menu_content(f, area, app),

// Add draw function
fn draw_your_menu_content(f: &mut Frame, area: Rect, app: &App) {
    // Your rendering logic here
}
```

### 4. Update Input Handler

Add to `src/ui/input_handler.rs`:

```rust
// Add to main input match
AppState::YourMenu => handle_your_menu_input(app, &key_event),

// Add input handler
fn handle_your_menu_input(app: &mut App, key_event: &crossterm::event::KeyEvent) {
    match key_event.code {
        KeyCode::Up => app.your_menu.select_up(),
        KeyCode::Down => app.your_menu.select_down(),
        KeyCode::Enter => handle_your_menu_selection(app),
        _ => {}
    }
}

fn handle_your_menu_selection(app: &mut App) {
    match app.your_menu.selected_option {
        0 => { /* Option 1 action */ }
        1 => { /* Option 2 action */ }
        2 => { app.switch_to_main_menu(); }
        _ => {}
    }
}
```

### 5. Add to Main Menu

Update `src/ui/states/main_menu.rs`:

```rust
// Add to options vector
options: vec![
    // ... existing options
    "Your Menu".to_string(),
    "Help".to_string(),
    "Quit".to_string(),
],
```

Update `src/ui/input_handler.rs` main menu selection handler.

## Best Practices

1. **Naming Convention**: Use snake_case for file names and menu names
2. **State Management**: Always implement select_up/select_down methods
3. **Navigation**: Include a "Back to Main Menu" option
4. **Error Handling**: Handle invalid selections gracefully
5. **Documentation**: Add module documentation to your state file

## Example Custom Menu

Here's an example of a custom "Battle" menu:

```rust
#[derive(Debug)]
pub struct BattleState {
    pub selected_option: usize,
    pub options: Vec<String>,
    pub player_hp: u32,
    pub enemy_hp: u32,
}

impl BattleState {
    pub fn new() -> Self {
        Self {
            selected_option: 0,
            options: vec![
                "Attack".to_string(),
                "Defend".to_string(),
                "Use Item".to_string(),
                "Run".to_string(),
            ],
            player_hp: 100,
            enemy_hp: 100,
        }
    }

    pub fn attack(&mut self) {
        // Battle logic here
        self.enemy_hp = self.enemy_hp.saturating_sub(20);
    }
}
```

## Troubleshooting

- **Compilation Errors**: Make sure all imports and references are correct
- **Navigation Issues**: Check that switch methods are properly implemented
- **Rendering Problems**: Verify draw functions are added to the match statement
- **Input Not Working**: Ensure input handlers are properly registered

For more help, check the existing menu implementations in `src/ui/states/`.
'''
    
    with open(doc_file, 'w') as f:
        f.write(content)
    
    print_status(f"Documentation created: {doc_file}")

def main():
    """Main function"""
    if len(sys.argv) != 2:
        print_error("Usage: python3 generate_menu.py <menu_name>")
        print("Example: python3 generate_menu.py battle")
        print("This will create a new menu state called 'battle'")
        sys.exit(1)
    
    menu_name = sys.argv[1].lower()
    menu_name_capitalized = capitalize_first(menu_name)
    
    print_header()
    print_status(f"Generating menu: {menu_name}")
    
    # Create the menu state file
    create_menu_state_file(menu_name, menu_name_capitalized)
    
    # Update the states module
    update_states_module(menu_name, menu_name_capitalized)
    
    # Update the renderer
    update_renderer(menu_name, menu_name_capitalized)
    
    # Update the input handler
    update_input_handler(menu_name, menu_name_capitalized)
    
    # Update the main menu
    update_main_menu(menu_name, menu_name_capitalized)
    
    # Create documentation
    create_documentation()
    
    print_header()
    print_status("Menu generation complete!")
    print_status(f"New menu '{menu_name}' has been created and integrated into the application.")
    print_status("")
    print_status("Next steps:")
    print_status(f"1. Review the generated files in src/ui/states/{menu_name}.rs")
    print_status("2. Customize the menu options and functionality")
    print_status("3. Test the menu by running the application")
    print_status("4. Check the documentation at docs/menu_creation.md")
    print_status("")
    print_status("To add another menu, run: python3 scripts/generate_menu.py <menu_name>")

if __name__ == "__main__":
    main() 