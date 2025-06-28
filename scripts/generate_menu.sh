#!/bin/bash

# Pokemon Arena Menu Generator
# This script creates a new custom menu state with all necessary files and code changes

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_header() {
    echo -e "${BLUE}================================${NC}"
    echo -e "${BLUE}  Pokemon Arena Menu Generator${NC}"
    echo -e "${BLUE}================================${NC}"
}

# Check if menu name is provided
if [ $# -eq 0 ]; then
    print_error "Usage: $0 <menu_name>"
    echo "Example: $0 battle"
    echo "This will create a new menu state called 'battle'"
    exit 1
fi

MENU_NAME=$1
MENU_NAME_CAPITALIZED=$(echo $MENU_NAME | sed 's/^./\U&/')
MENU_NAME_UPPERCASE=$(echo $MENU_NAME | tr '[:lower:]' '[:upper:]')

print_header
print_status "Generating menu: $MENU_NAME"

# Create the menu state file
MENU_FILE="src/ui/states/${MENU_NAME}.rs"
print_status "Creating menu state file: $MENU_FILE"

cat > "$MENU_FILE" << EOF
//! # ${MENU_NAME_CAPITALIZED} Menu State
//! 
//! This module handles the ${MENU_NAME} menu state which includes:
//! - Custom functionality for ${MENU_NAME}
//! - Menu navigation
//! - State management

/// Represents the ${MENU_NAME} menu state
#[derive(Debug)]
pub struct ${MENU_NAME_CAPITALIZED}State {
    /// Currently selected menu option
    pub selected_option: usize,
    /// Available menu options
    pub options: Vec<String>,
    /// Custom data for ${MENU_NAME} functionality
    pub custom_data: String,
}

impl ${MENU_NAME_CAPITALIZED}State {
    /// Create a new ${MENU_NAME} menu state with default values
    pub fn new() -> Self {
        Self {
            selected_option: 0,
            options: vec![
                "Option 1".to_string(),
                "Option 2".to_string(),
                "Option 3".to_string(),
                "Back to Main Menu".to_string(),
            ],
            custom_data: "Default ${MENU_NAME} data".to_string(),
        }
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

    /// Custom method for ${MENU_NAME} functionality
    pub fn custom_action(&mut self) {
        self.custom_data = format!("${MENU_NAME} action performed at {:?}", std::time::Instant::now());
    }
}
EOF

print_status "Menu state file created successfully"

# Update the states module file
print_status "Updating states module file..."
MOD_FILE="src/ui/states/mod.rs"

# Add the module declaration
if ! grep -q "pub mod ${MENU_NAME};" "$MOD_FILE"; then
    # Find the line with other module declarations and add after it
    sed -i "/pub mod help;/a pub mod ${MENU_NAME};" "$MOD_FILE"
    print_status "Added module declaration"
fi

# Add the re-export
if ! grep -q "pub use ${MENU_NAME}::${MENU_NAME_CAPITALIZED}State;" "$MOD_FILE"; then
    # Find the line with other re-exports and add after it
    sed -i "/pub use help::HelpState;/a pub use ${MENU_NAME}::${MENU_NAME_CAPITALIZED}State;" "$MOD_FILE"
    print_status "Added re-export"
fi

# Add to AppState enum
if ! grep -q "/// ${MENU_NAME_CAPITALIZED} menu" "$MOD_FILE"; then
    # Find the Help state in AppState enum and add before it
    sed -i "/    /// Help\/instructions screen/a     /// ${MENU_NAME_CAPITALIZED} menu\n    ${MENU_NAME_CAPITALIZED}," "$MOD_FILE"
    print_status "Added to AppState enum"
fi

# Add to App struct
if ! grep -q "pub ${MENU_NAME}: ${MENU_NAME_CAPITALIZED}State," "$MOD_FILE"; then
    # Find the help field in App struct and add before it
    sed -i "/    pub help: HelpState,/a     /// ${MENU_NAME} state\n    pub ${MENU_NAME}: ${MENU_NAME_CAPITALIZED}State," "$MOD_FILE"
    print_status "Added to App struct"
fi

# Add to App::new()
if ! grep -q "${MENU_NAME}: ${MENU_NAME_CAPITALIZED}State::new()," "$MOD_FILE"; then
    # Find the help initialization and add before it
    sed -i "/            help: HelpState::new(),/a             ${MENU_NAME}: ${MENU_NAME_CAPITALIZED}State::new()," "$MOD_FILE"
    print_status "Added to App::new()"
fi

# Add switch method
if ! grep -q "pub fn switch_to_${MENU_NAME}" "$MOD_FILE"; then
    # Find the switch_to_help method and add after it
    sed -i "/    pub fn switch_to_help(&mut self) {/a     /// Switch to ${MENU_NAME}\n    pub fn switch_to_${MENU_NAME}(&mut self) {\n        self.state_manager.switch_to_${MENU_NAME}();\n    }\n" "$MOD_FILE"
    print_status "Added switch method"
fi

# Add to StateManager switch method
if ! grep -q "pub fn switch_to_${MENU_NAME}" "$MOD_FILE"; then
    # Find the switch_to_help method in StateManager and add after it
    sed -i "/    pub fn switch_to_help(&mut self) {/a     /// Switch to ${MENU_NAME}\n    pub fn switch_to_${MENU_NAME}(&mut self) {\n        self.switch_to(AppState::${MENU_NAME_CAPITALIZED});\n    }\n" "$MOD_FILE"
    print_status "Added StateManager switch method"
fi

print_status "States module updated successfully"

# Update the renderer
print_status "Updating renderer..."
RENDERER_FILE="src/ui/renderer.rs"

# Add to header match statement
if ! grep -q "AppState::${MENU_NAME_CAPITALIZED}" "$RENDERER_FILE"; then
    # Find the Help case and add before it
    sed -i "/        AppState::Help => (/a         AppState::${MENU_NAME_CAPITALIZED} => (\n            \"Pokemon Arena - ${MENU_NAME_CAPITALIZED}\",\n            \"${MENU_NAME_CAPITALIZED} menu: Arrow keys to navigate, Enter to select\"\n        )," "$RENDERER_FILE"
    print_status "Added to header match"
fi

# Add to main content match statement
if ! grep -q "AppState::${MENU_NAME_CAPITALIZED} => draw_${MENU_NAME}_content" "$RENDERER_FILE"; then
    # Find the Help case and add before it
    sed -i "/        AppState::Help => draw_help_content(f, area, app),/a         AppState::${MENU_NAME_CAPITALIZED} => draw_${MENU_NAME}_content(f, area, app)," "$RENDERER_FILE"
    print_status "Added to main content match"
fi

# Add the draw function (at the end of the file, before draw_footer)
if ! grep -q "fn draw_${MENU_NAME}_content" "$RENDERER_FILE"; then
    cat >> "$RENDERER_FILE" << EOF

/// Draw the ${MENU_NAME} content
fn draw_${MENU_NAME}_content(f: &mut Frame, area: Rect, app: &App) {
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
                "${MENU_NAME_CAPITALIZED} Menu",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ])
    .block(Block::default().borders(Borders::ALL).title("${MENU_NAME_CAPITALIZED}"));

    f.render_widget(title_widget, chunks[0]);

    // Draw menu
    let menu_items = app.${MENU_NAME}
        .options
        .iter()
        .enumerate()
        .map(|(index, option)| {
            if index == app.${MENU_NAME}.selected_option {
                Line::from(vec![
                    Span::styled(
                        format!("> {}", option),
                        Style::default()
                            .fg(Color::Black)
                            .bg(Color::White)
                            .add_modifier(Modifier::BOLD),
                    ),
                ])
            } else {
                Line::from(vec![
                    Span::styled(
                        format!("  {}", option),
                        Style::default().fg(Color::White),
                    ),
                ])
            }
        })
        .collect::<Vec<_>>();

    let menu_widget = Paragraph::new(menu_items)
        .block(Block::default().borders(Borders::ALL).title("Options"));

    f.render_widget(menu_widget, chunks[1]);

    // Draw info section
    let info_widget = Paragraph::new(vec![
        Line::from(vec![
            Span::styled(
                format!("Custom Data: {}", app.${MENU_NAME}.custom_data),
                Style::default().fg(Color::Green),
            ),
        ]),
    ])
    .block(Block::default().borders(Borders::ALL).title("Info"));

    f.render_widget(info_widget, chunks[2]);
}
EOF
    print_status "Added draw function"
fi

print_status "Renderer updated successfully"

# Update the input handler
print_status "Updating input handler..."
INPUT_FILE="src/ui/input_handler.rs"

# Add to main input match statement
if ! grep -q "AppState::${MENU_NAME_CAPITALIZED} => handle_${MENU_NAME}_input" "$INPUT_FILE"; then
    # Find the Help case and add before it
    sed -i "/        AppState::Help => handle_help_input(app, &key_event),/a         AppState::${MENU_NAME_CAPITALIZED} => handle_${MENU_NAME}_input(app, &key_event)," "$INPUT_FILE"
    print_status "Added to input match"
fi

# Add the input handler function (at the end of the file)
if ! grep -q "fn handle_${MENU_NAME}_input" "$INPUT_FILE"; then
    cat >> "$INPUT_FILE" << EOF

/// Handle input for the ${MENU_NAME} state
fn handle_${MENU_NAME}_input(app: &mut App, key_event: &crossterm::event::KeyEvent) {
    match key_event.code {
        // Arrow keys for navigation
        KeyCode::Up => {
            app.${MENU_NAME}.select_up();
        }
        KeyCode::Down => {
            app.${MENU_NAME}.select_down();
        }
        
        // Enter key to select current option
        KeyCode::Enter => {
            handle_${MENU_NAME}_selection(app);
        }
        
        // Ignore all other keys
        _ => {}
    }
}

/// Process the currently selected menu option in ${MENU_NAME}
fn handle_${MENU_NAME}_selection(app: &mut App) {
    match app.${MENU_NAME}.selected_option {
        0 => {
            // Option 1
            app.${MENU_NAME}.custom_action();
        }
        1 => {
            // Option 2
            app.${MENU_NAME}.custom_action();
        }
        2 => {
            // Option 3
            app.${MENU_NAME}.custom_action();
        }
        3 => {
            // Back to Main Menu
            app.switch_to_main_menu();
        }
        _ => {
            // This shouldn't happen, but handle it gracefully
            eprintln!("Invalid selection: {}", app.${MENU_NAME}.selected_option);
        }
    }
}
EOF
    print_status "Added input handler"
fi

print_status "Input handler updated successfully"

# Update main menu to include the new menu option
print_status "Adding menu option to main menu..."
MAIN_MENU_FILE="src/ui/states/main_menu.rs"

# Add the menu option (before "Help")
if ! grep -q "\"${MENU_NAME_CAPITALIZED}\"" "$MAIN_MENU_FILE"; then
    sed -i "/                \"Help\".to_string(),/a                 \"${MENU_NAME_CAPITALIZED}\".to_string()," "$MAIN_MENU_FILE"
    print_status "Added menu option to main menu"
fi

# Update the main menu input handler
print_status "Updating main menu input handler..."
INPUT_FILE="src/ui/input_handler.rs"

# Update the selection handler to include the new menu
if ! grep -q "app.switch_to_${MENU_NAME}();" "$INPUT_FILE"; then
    # Find the Help case and add before it
    sed -i "/            5 => {/a         5 => {\n            // ${MENU_NAME_CAPITALIZED}\n            app.switch_to_${MENU_NAME}();\n        }\n        6 => {" "$INPUT_FILE"
    # Update the Help case number
    sed -i "s/        6 => {/        7 => {/" "$INPUT_FILE"
    # Update the Quit case number
    sed -i "s/        7 => {/        8 => {/" "$INPUT_FILE"
    print_status "Updated main menu selection handler"
fi

print_status "Main menu updated successfully"

# Create documentation
print_status "Creating documentation..."
DOC_FILE="docs/menu_creation.md"

mkdir -p docs

cat > "$DOC_FILE" << EOF
# Adding Custom Menus to Pokemon Arena

This document explains how to add new custom menus to the Pokemon Arena application.

## Quick Start

To add a new menu called "battle", simply run:

\`\`\`bash
./scripts/generate_menu.sh battle
\`\`\`

This will automatically:
- Create the menu state file
- Update all necessary modules
- Add navigation to the main menu
- Create input handlers
- Add rendering functions

## Manual Process

If you prefer to create menus manually, follow these steps:

### 1. Create the State File

Create \`src/ui/states/your_menu.rs\`:

\`\`\`rust
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
\`\`\`

### 2. Update States Module

Add to \`src/ui/states/mod.rs\`:

\`\`\`rust
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
\`\`\`

### 3. Update Renderer

Add to \`src/ui/renderer.rs\`:

\`\`\`rust
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
\`\`\`

### 4. Update Input Handler

Add to \`src/ui/input_handler.rs\`:

\`\`\`rust
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
\`\`\`

### 5. Add to Main Menu

Update \`src/ui/states/main_menu.rs\`:

\`\`\`rust
// Add to options vector
options: vec![
    // ... existing options
    "Your Menu".to_string(),
    "Help".to_string(),
    "Quit".to_string(),
],
\`\`\`

Update \`src/ui/input_handler.rs\` main menu selection handler.

## Best Practices

1. **Naming Convention**: Use snake_case for file names and menu names
2. **State Management**: Always implement select_up/select_down methods
3. **Navigation**: Include a "Back to Main Menu" option
4. **Error Handling**: Handle invalid selections gracefully
5. **Documentation**: Add module documentation to your state file

## Example Custom Menu

Here's an example of a custom "Battle" menu:

\`\`\`rust
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
\`\`\`

## Troubleshooting

- **Compilation Errors**: Make sure all imports and references are correct
- **Navigation Issues**: Check that switch methods are properly implemented
- **Rendering Problems**: Verify draw functions are added to the match statement
- **Input Not Working**: Ensure input handlers are properly registered

For more help, check the existing menu implementations in \`src/ui/states/\`.
EOF

print_status "Documentation created: $DOC_FILE"

# Make the script executable
chmod +x scripts/generate_menu.sh

print_status "Script made executable"

print_header
print_status "Menu generation complete!"
print_status "New menu '$MENU_NAME' has been created and integrated into the application."
print_status ""
print_status "Next steps:"
print_status "1. Review the generated files in src/ui/states/${MENU_NAME}.rs"
print_status "2. Customize the menu options and functionality"
print_status "3. Test the menu by running the application"
print_status "4. Check the documentation at docs/menu_creation.md"
print_status ""
print_status "To add another menu, run: ./scripts/generate_menu.sh <menu_name>" 